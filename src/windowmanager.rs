#![allow(dead_code)]
use libc::{c_int, c_uint};
use std::collections::HashMap;

use crate::config::*;
use crate::xlibwrapper::{
    masks::*,
    core::*,
    util::*,
    xlibmodels::*,
};
use crate::models::{
    windowwrapper::*,
    rect::*,
    screen::*,
    dockarea::*,
    window_type::*,
};

use std::rc::Rc;

#[derive(PartialEq, Eq)]
pub enum Mode {
    Floating,
    Tiled
}

pub struct WindowManager {
    pub lib: Rc<XlibWrapper>,
    pub mode: Mode,
    pub focus_w: Window,
    pub dock_area: DockArea,
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
        Self {
            lib: lib,
            mode: Mode::Floating,
            focus_w: root,
            dock_area: Default::default(),
            clients: HashMap::new(),
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
            current_ws: 1
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
        self.center_window(w);
        self.lib.map_window(w);
        self.lib.raise_window(w);
    }

    pub fn set_current_ws(&mut self, ws: u32) {
        println!("Desktop changed to: {}", ws);

        //TODO: hide all clients on current desktop
        //unhide all clients (if any) on new desktop
    

        let to_hide: Vec<Window> = self.clients.iter()
            .filter(|(key, val)| {
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
            .filter(|(key, val)| {
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


        self.lib.ewmh_current_desktop(ws);
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
        if let Some(dock_rect) = self.dock_area.as_rect(self.lib.get_screen()) {
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

    fn center_window(&mut self, w: Window) {
        let ww = match self.clients.get_mut(&w) {
            Some(ww) => ww,
            None => { return }
        };
        let screen = self.lib.get_screen();

        let mut dw = (screen.width - ww.get_width() as i32).abs() / 2;
        let mut dh = (screen.height - ww.get_height() as i32).abs() / 2;

        if let Some(dock_rect) = self.dock_area.as_rect(self.lib.get_screen()) {
            dh = ((screen.height + dock_rect.get_size().height as i32) - ww.get_height() as i32).abs() / 2;
        }

        /*println!("Screen width: {} Screen height: {}", screen.width, screen.height);
          println!("Window width: {} Window height: {}", ww.get_width(), ww.get_height());

          println!("dw: {}, dh: {}", dw, dh);*/
        //self.move_window(ww, new_x, new_y);

        self.move_window(w, dw, dh);
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

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) {
        let ww = match self.clients.get_mut(&w) {
            Some(ww) => ww,
            None => { return }
        };
        let mut y = y;
        match self.dock_area.as_rect(self.lib.get_screen()) {
            Some(dock) => {
                if y < dock.get_size().height as i32 {
                    y = dock.get_size().height as i32;
                }
            }
            None => {}
        }


        match ww.get_dec() {
            Some(dec) => {
                self.lib.move_window(
                    dec,
                    x,
                    y
                );
                self.lib.move_window(
                    ww.window(),
                    x + CONFIG.inner_border_width + CONFIG.border_width,
                    y + CONFIG.inner_border_width + CONFIG.border_width + CONFIG.decoration_height
                );
                let new_pos = Position { x, y };
                ww.set_position(new_pos);
            },
            None => {
                self.lib.move_window(
                    ww.window(),
                    x,
                    y
                );
                ww.set_position(Position { x, y });
            }
        }
        //println!("Window pos: {:?}", ww.get_position());
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

        //println!("Pointer_pos: {:?}, Window_pos: {:?}", pointer_pos, window_pos);

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
                .map(|(c, w)| {
                    *c
                }).collect();
            self.lib.update_net_client_list(clients);
        }
        println!("Top level windows: {}", self.lib.top_level_window_count());
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) {

        let ww = match self.clients.get_mut(&w) {
            Some(ww) => ww,
            None => {
                return;
            }
        };

        if let Some(dec_rect) = ww.get_dec_rect() {
            let mut dec_w = width - (2 * CONFIG.border_width as u32);
            let mut dec_h = height - (2 * CONFIG.border_width as u32);

            if width == dec_rect.get_size().width {
                dec_w = width;
            } else if height == dec_rect.get_size().height {
                dec_h = height;
            }

            let dec_size = Size { width: dec_w, height: dec_h };
            ww.set_dec_size(dec_size);
            self.lib.resize_window(ww.get_dec().unwrap(), dec_size.width, dec_size.height);

        }

        let window_rect = ww.get_inner_rect();
        let mut d_width = width - (2* CONFIG.inner_border_width as u32) - (2 * CONFIG.border_width as u32);
        let mut d_height = height - (2* CONFIG.inner_border_width as u32) - (2 * CONFIG.border_width as u32) - CONFIG.decoration_height as u32;

        if width == window_rect.get_size().width {
            d_width = width;
        } else if height == window_rect.get_size().height {
            d_height = height;
        }

        let window_size = Size { width: d_width, height: d_height };

        ww.set_inner_size(window_size);
        self.lib.resize_window(ww.window(), window_size.width, window_size.height);

        //println!("Window width: {}, height: {}", ww.get_width(), ww.get_height());
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
        "1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|key| { keysym_lookup::into_keysym(key).expect("Core: no such key") })
            .for_each(|key_sym| { self.lib.grab_keys(w, key_sym, Mod4Mask | Shift) });
    }

    pub fn grab_buttons(&self, w: Window) {
        vec![Button1, Button3]
            .into_iter()
            .for_each(|button| {
                self.lib.grab_button(
                    button,
                    Mod4Mask,
                    w,
                    false,
                    (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                    GrabModeAsync,
                    GrabModeAsync,
                    0,0);
            })
    }
}
