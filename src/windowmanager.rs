#![allow(dead_code)]
use libc::{c_int, c_uint};
use std::collections::HashMap;

use crate::config::*;
use crate::layout::*;
use crate::xlibwrapper::{
    masks::*,
    core::*,
    util::*,
    xlibmodels::*,
};
use crate::models::{
    windowwrapper::*,
    rect::*,
    dockarea::*,
    window_type::*,
    WindowState,
    Direction
};

use std::rc::Rc;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Floating,
    Tiled
}

pub struct WindowManager {
    pub lib: Rc<XlibWrapper>,
    pub mode: Mode,
    pub focus_w: Window,
    pub dock_area: DockArea,
    pub layout: Box<dyn Layout>,
    pub clients: HashMap<u64, WindowWrapper>,
    pub drag_start_pos: (c_int, c_int),
    pub drag_start_frame_pos: (c_int, c_int),
    pub drag_start_frame_size: (c_uint, c_uint),
    pub current_ws: u32
}

impl WindowManager {

    /*
     * Open a connection to X display
     * Check for failure
     * return WindowManager
     */
    pub fn new (lib: Rc<XlibWrapper>) -> Self {
        let root = lib.get_root();
        let mode = Mode::Floating;
        let _screen = lib.get_screen();
        Self {
            lib: lib,
            mode,
            focus_w: root,
            dock_area: Default::default(),
            clients: HashMap::new(),
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
            current_ws: 1,
            layout: match mode {
                Mode::Floating => Box::new(floating::Floating),
                Mode::Tiled => Box::new(tiled::Tiled)
            },
        }
    }

    fn decorate_window(&self, w: &mut WindowWrapper) {
        let window_geom = self.lib.get_geometry(w.window());
        let position = Position { x: window_geom.x - CONFIG.inner_border_width - CONFIG.border_width,
        y: window_geom.y - CONFIG.inner_border_width - CONFIG.border_width - CONFIG.decoration_height };

        let size = Size { width: window_geom.width + (2 * CONFIG.inner_border_width as u32), height: window_geom.height + (2 * CONFIG.inner_border_width as u32) + CONFIG.decoration_height as u32 };
        let dec_window = self.lib.create_simple_window(
            self.lib.get_root(),
            position.clone(),
            size.clone(),
            CONFIG.border_width as u32,
            CONFIG.background_color,
            CONFIG.background_color
        );

        self.lib.select_input(dec_window, ExposureMask);

        self.lib.grab_button(
            Button1,
            AnyModifier,
            dec_window,
            true,
            (ButtonPressMask | ButtonReleaseMask | PointerMotionMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,
            0
        );

        self.grab_keys(dec_window);
        w.set_decoration(dec_window, Rect::new(position, size));
        self.lib.map_window(dec_window);
        self.lib.sync(false);
    }

    pub fn setup_window(&mut self, w: Window) {
        if self.lib.get_window_type(w) == WindowType::Dock {
            self.dock_area = match self.lib.get_window_strut_array(w) {
                Some(dock) => {
                    let dock = DockArea::from(dock);
                    //println!("dock geometry: {:?}", dock.as_rect(self.lib.get_screen()).expect("No fekking dock area!"));
                    dock
                }
                None => {
                    return
                }
            }
        }

        if !self.should_be_managed(w) {
            return
        }

        if self.clients.contains_key(&w) {
            return
        }

        let geom = self.lib.get_geometry(w);
        let mut ww = WindowWrapper::new(w, Rect::from(geom), self.current_ws);
        self.lib.set_border_width(w, CONFIG.inner_border_width as u32);
        self.lib.set_border_color(w, CONFIG.border_color);
        self.decorate_window(&mut ww);
        self.lib.add_to_save_set(w);
        self.lib.add_to_root_net_client_list(w);
        self.subscribe_to_events(w);
        self.clients.insert(w, ww);
        self.window_initial_size(w);
        self.place_window(w);
        self.lib.map_window(w);
        self.raise_window(&ww);
    }

    pub fn get_windows_in_current_ws(&self) -> Vec<Window> {
        self.get_windows_by_ws(self.current_ws)
    }

    pub fn get_windows_by_ws(&self, ws: u32) -> Vec<Window> {
        self.clients.iter()
            .filter(|(_key, val)| {
                val.get_desktop() == ws
            }).map(|(key, _val)| {
                *key
            }).collect::<Vec<Window>>()
    }

    pub fn set_current_ws(&mut self, ws: u32) {
        println!("Desktop changed to: {}", ws);

        let to_hide: Vec<Window> = self.clients.iter()
            .filter(|(_key, val)| {
                val.get_desktop() == self.current_ws
            }).map(|(key, _val)| {
                *key
            }).collect();

        to_hide.iter()
            .for_each(|win| {
                match self.clients.get_mut(win) {
                    Some(ww) => {
                        ww.save_restore_position();
                        let ww_pos = ww.get_position();
                        self.move_window(*win, self.lib.get_screen().width * 3, ww_pos.y)
                    },
                    None => {}
                }
            });

        self.current_ws = ws;

        let to_show: Vec<Window> = self.clients.iter()
            .filter(|(_key, val)| {
                val.get_desktop() == self.current_ws
            }).map(|(key, _val)| {
                *key
            }).collect();

        to_show.iter()
            .for_each(|win| {
                match self.clients.get_mut(win) {
                    Some(ww) => {
                        let pos = ww.get_restore_position();
                        ww.set_position(pos);
                        self.move_window(*win, pos.x, pos.y)
                    },
                    None => {}
                }
            });

        if to_show.is_empty() {
            self.focus_w = self.lib.get_root();
            self.lib.take_focus(self.lib.get_root());
        } else {
            let w = *(to_show.get(0).unwrap());
            self.focus_w = w;
            self.lib.take_focus(w);
        }
        self.lib.ewmh_current_desktop(ws);
    }

    pub fn move_to_ws(&mut self, w: Window, ws: u8) {
        match self.clients.get_mut(&w) {
            Some(ww) => {
                ww.save_restore_position();
                ww.set_desktop(ws as u32);
            },
            _ => ()
        }
    }

    fn client_hide(&mut self, ww: &mut WindowWrapper) {
        ww.save_restore_position();
        self.move_window(ww.window(), self.lib.get_screen().width * 2, ww.get_position().y)
    }

    fn window_initial_size(&mut self, w: Window) {

        if self.mode != Mode::Floating {
            return
        }

        let screen = self.lib.get_screen();
        if let Some(dock_rect) = self.dock_area.as_rect(&self.lib.get_screen()) {
            //println!("DockArea: {:?}", dock_rect);
            let new_width = (screen.width - (screen.width / 10)) as u32;
            let new_height = ((screen.height - dock_rect.get_size().height as i32) - (screen.height / 10)) as u32;

            self.resize_window(w, new_width, new_height);

        } else {

            let new_width = (screen.width - (screen.width / 10)) as u32;
            let new_height = (screen.height - (screen.height / 10)) as u32;

            self.resize_window(w, new_width, new_height);
        }
    }

    pub fn toggle_maximize(&mut self, w: Window) {
        // TODO: cleanup this mess...
        if !self.clients.contains_key(&w) {
            eprintln!("toggle_maximize: Window not in client list: {}", w);
            println!("Client list:");
            self.clients.keys()
                .for_each(|key| {
                    println!("Client: {}", key)
                });
            return
        }

        let state = self.clients.get(&w).unwrap().get_window_state();
        let restore_pos = self.clients.get(&w).unwrap().get_restore_position();
        match state {
            WindowState::Maximized => {
                self.move_window(w, restore_pos.x, restore_pos.y);
                let ww = self.clients.get_mut(&w).expect("How can it not be in list?!");
                ww.restore_prev_state();
                let size = ww.get_restore_size();
                self.resize_window(w, size.width, size.height);
            },
            _ => {
                self.clients.get_mut(&w).expect("Not in list?!").save_restore_position();
                self.move_window(w, 0 + CONFIG.border_width, 0);
                let size = self.layout.maximize(&self, w);
                let ww = self.clients.get_mut(&w).expect("Not in list?!");
                ww.set_window_state(WindowState::Maximized);
                ww.save_restore_size();
                self.resize_window(w, size.width, size.height);
            }
        }
    }
    
    pub fn shift_window(&mut self, w: Window, direction: Direction) {
        if !self.clients.contains_key(&w) {
            return
        }
        
        let (pos, size) = self.layout.shift_window(&self, w, direction);
        self.move_window(w, pos.x, pos.y);
        self.resize_window(w, size.width, size.height);
        self.clients.get_mut(&w).unwrap().set_window_state(WindowState::Snapped);
    }

    fn place_window(&mut self, w: Window) {
        if !self.clients.contains_key(&w) {
            return
        }
         
        let pos = self.layout.place_window(&self, w);
        self.move_window(w, pos.x, pos.y);
        let ww = self.clients.get_mut(&w).unwrap();
        ww.set_position(pos);
    }

    pub fn should_be_managed(&self, w: Window) -> bool {
        if let Some(prop_val) = self.lib.get_window_type_atom(w) {
            if vec![self.lib.xatom.NetWMWindowTypeDock,
            self.lib.xatom.NetWMWindowTypeToolbar,
            self.lib.xatom.NetWMWindowTypeUtility,
            self.lib.xatom.NetWMWindowTypeDialog,
            self.lib.xatom.NetWMWindowTypeMenu].contains(&prop_val) {
                return false;
            }
        }

        if self.lib.get_window_attributes(w).override_redirect {
            return false;
        }
        true
    }

    pub fn raise_window(&self, ww: &WindowWrapper) {
        match ww.get_dec() {
            Some(dec) => {
                self.lib.raise_window(dec);
                self.lib.raise_window(ww.window());
            },
            None => self.lib.raise_window(ww.window())
        }
    }

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) {
        let ww = match self.clients.get(&w) {
            Some(ww) => ww,
            None => { return }
        };
        let (outer_pos, inner_pos) = self.layout.move_window(&self, w, x, y);

        match ww.get_dec() {
            Some(dec) => {
                self.lib.move_window(
                    dec,
                    outer_pos
                );
                self.lib.move_window(
                    ww.window(),
                    inner_pos
                );
                let ww = self.clients.get_mut(&w).unwrap();
                ww.set_position(outer_pos);
            },
            None => {
                self.lib.move_window(
                    ww.window(),
                    outer_pos
                );
                let ww = self.clients.get_mut(&w).unwrap();
                ww.set_position(outer_pos);
            }
        }
    }

    pub fn pointer_is_inside(&self, w: Window) -> bool {
        let pointer_pos = self.lib.pointer_root_pos(w);
        if !self.clients.contains_key(&w) {
            let geom = self.lib.get_geometry(w);

            let inside_height = pointer_pos.y > geom.y &&
                pointer_pos.y < geom.height as i32 + geom.y;

            let inside_width = pointer_pos.x > geom.x &&
                pointer_pos.x < geom.width as i32 + geom.x;

            return inside_height && inside_width;
        }

        let ww = self.clients.get(&w).unwrap();
        let window_size = ww.get_size();
        let window_pos = ww.get_position();

        let inside_height = pointer_pos.y > window_pos.y &&
            pointer_pos.y < window_pos.y + window_size.height as i32;

        let inside_width = pointer_pos.x > window_pos.x &&
            pointer_pos.x < window_pos.x + window_size.width as i32;

        inside_height && inside_width
    }

    fn destroy_dec(&self, ww: WindowWrapper) {
        match ww.get_dec() {
            Some(dec) => {
                self.lib.unmap_window(dec);
                self.lib.destroy_window(dec);
            },
            None => {}
        }
    }

    pub fn center_cursor(&self, w: Window) {
        match self.clients.get(&w) {
            Some(ww) => self.lib.center_cursor(&ww),
            None => { return }
        }
    }

    pub fn kill_window(&mut self, w: Window) {
        if !self.clients.contains_key(&w) || w == self.lib.get_root() {
            return;
        }

        let frame = self.clients.get(&w).expect("KillWindow: No such window in client list");


        if self.lib.kill_client(frame.window()) {

            if frame.decorated() {
                self.destroy_dec(*frame);
            }
            self.clients.remove(&w);
            let clients: Vec<Window> = self.clients
                .iter()
                .map(|(c, _w)| {
                    *c
                }).collect();
            self.lib.update_net_client_list(clients);
        }
        println!("Top level windows: {}", self.lib.top_level_window_count());
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) {
        if !self.clients.contains_key(&w) {
            return
        }

        let (dec_size, window_size) = self.layout.resize_window(&self, w, width, height);

        let ww = self.clients.get_mut(&w).expect("Client not found in resize_window");

        if let Some(_) = ww.get_dec_rect() {
            ww.set_dec_size(dec_size);
            self.lib.resize_window(ww.get_dec().expect("resize_window: no dec"), dec_size.width, dec_size.height);
        }

        ww.set_inner_size(window_size);
        self.lib.resize_window(ww.window(), window_size.width, window_size.height);
    }

    fn subscribe_to_events(&mut self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask | FocusChangeMask | PropertyChangeMask
        );
    }

    pub fn grab_keys(&self, w: Window) {

        let _keys = vec!["q",
        "Left",
        "Up",
        "Right",
        "Down",
        "Return",
        "f",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|key| { keysym_lookup::into_keysym(key).expect("Core: no such key") })
            .for_each(|key_sym| { self.lib.grab_keys(w, key_sym, Mod4Mask | Shift) });
    }

    pub fn grab_buttons(&self, w: Window) {
        let modifiers = vec![0, Mod4Mask];
        vec![Button1, Button3]
            .into_iter()
            .for_each(|button| {
                modifiers.iter()
                    .for_each(|modifier| {
                        self.lib.grab_button(
                            button,
                            *modifier,
                            w,
                            false,
                            (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                            GrabModeAsync,
                            GrabModeAsync,
                            0,0);
                    })
            })
    }
}
