#![allow(dead_code)]
use std::collections::{
    HashMap,
    HashSet,
    BTreeMap
};
use std::rc::Rc;

use crate::{
    config::*,
    layout::*,
    xlibwrapper::{
        masks::*,
        core::*,
        util::*,
        xlibmodels::*
    },
    models::{
        windowwrapper::*,
        rect::*,
        dockarea::*,
        window_type::*,
        monitor::Monitor,
        screen::*,
        workspace::*,
        WindowState,
        Direction
    }
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Mode {
    Floating,
    Tiled
}

pub struct WindowManager {
    pub lib: Rc<XlibWrapper>,
    pub mode: Mode,
    pub focus_w: Window,
    pub monitors: HashMap<u32, Monitor>,
    pub current_monitor: u32,
    pub dock_area: DockArea,
    pub drag_start_pos: (i32, i32),
    pub drag_start_frame_pos: (i32, i32),
    pub drag_start_frame_size: (u32, u32),
    pub decorate: bool,
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
        let monitors = {
            let mut monitors = HashMap::default();
            let _ = lib.get_screens()
                .iter()
                .enumerate()
                .for_each(|(i, val)| {
                    info!("Monitors in init: {}", i);
                    monitors.insert(i as u32, Monitor::new(val.clone(), Workspace::new(i as u32)));
                });
            monitors
        };

        Self {
            lib: lib,
            mode,
            focus_w: root,
            monitors,
            current_monitor: 0,
            dock_area: Default::default(),
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
            decorate: CONFIG.decorate
        }
    }

    pub fn current_monitor(&mut self) -> &mut Monitor {
        self.monitors.get_mut(&self.current_monitor).unwrap()
    }

    pub fn set_current_monitor(&mut self) {
        self.current_monitor = self.monitors
            .iter()
            .filter(|(_key, mon)| self.pointer_is_inside(&mon.screen))
            .map(|(key, _mon)| *key)
            .fold(0, |acc, key| acc + key);
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
            match self.lib.get_window_strut_array(w) {
                Some(dock) => {
                    self.current_monitor().set_dock_area(DockArea::from(dock));
                }
                None => {
                    return
                }
            }
        }

        if !self.should_be_managed(w) {
            return
        }

        if self.current_monitor().contains_window(w) {
            return
        }

        let geom = self.lib.get_geometry(w);
        let mut ww = WindowWrapper::new(w, Rect::from(geom));
        if self.decorate {
            self.decorate_window(&mut ww);
        }
        self.lib.add_to_save_set(w);
        self.lib.add_to_root_net_client_list(w);
        self.subscribe_to_events(w);
        self.current_monitor().add_window(w, ww);
        self.window_initial_size(w);
        self.place_window(w);
        self.lib.map_window(w);
        self.raise_window(&ww);
    }


    fn hide_client(&mut self, w: Window) {
        let client = match self.current_monitor().get_client_mut(w) {
            Some(client) => client,
            None => { return }
        };
        client.save_restore_position();
        //self.move_window(w, self.lib.get_screen().width * 3, client_pos.y)
        match client.get_dec() {
            Some(dec) => {
                self.lib.unmap_window(dec);
                self.lib.unmap_window(w)
            },
            None => self.lib.unmap_window(w)
        }
    }

    fn show_client(&mut self, w: Window) {
        let client = match self.current_monitor().get_client_mut(w) {
            Some(client) => client,
            None => { return }
        };
        let pos = client.get_restore_position();
        client.set_position(pos);

        match client.get_dec() {
            Some(dec) => {
                self.lib.map_window(dec);
                self.lib.map_window(w)
            },
            None => self.lib.map_window(w)
        }
        //self.move_window(w, pos.x, pos.y)
    }

    pub fn set_current_ws(&mut self, ws: u32) {
        info!("Desktop changed to: {}", ws);

        self.current_monitor()
            .get_current_windows()
            .iter()
            .for_each(|win| self.hide_client(*win));

        self.current_monitor()
            .set_current_ws(ws);

        self.current_monitor()
            .get_current_windows()
            .iter()
            .for_each(|win| self.show_client(*win));
        let current = self.current_monitor().current_ws;
        self.lib.ewmh_current_desktop(current);
    }

    pub fn move_to_ws(&mut self, w: Window, ws: u8) {
        debug!("move_to_ws called with: {}", w);
        if self.current_monitor().contains_window(w) {
            let current = self.current_monitor().current_ws;
            self.hide_client(w);
            let ww_move = self.current_monitor().remove_window(w);
            self.current_monitor().set_current_ws(ws.into());
            self.current_monitor().add_window(w, ww_move);
            self.current_monitor().set_current_ws(current);
            self.lib.sync(false);
        } else {
            debug!("move_to_ws: no such client: {}", w);
        }
    }

    pub fn set_focus(&mut self, w: Window) {
        let root = self.lib.get_root();
        let client = self.current_monitor().get_client(w).map(|x| *x).clone();
        let ww = match client {
            Some(ww) => ww,
            None if w == root => {
                let root = self.lib.get_root();
                self.focus_w = root;
                self.lib.take_focus(self.focus_w);
                return
            },
            None => { return }
        };

        self.lib.remove_focus(self.focus_w);
        self.focus_w = ww.window();
        self.lib.ungrab_all_buttons(self.focus_w);
        self.grab_buttons(self.focus_w);
        self.lib.ungrab_keys(self.focus_w);
        self.grab_keys(self.focus_w);
        self.lib.take_focus(self.focus_w);

        match ww.get_dec() {
            Some(dec) => {
                self.lib.set_border_color(dec, CONFIG.border_color);
                self.lib.set_window_background_color(dec, CONFIG.focused_background_color);
            },
            None => {
                self.lib.set_border_width(w, CONFIG.border_width as u32);
                self.lib.set_border_color(w, CONFIG.border_color);
                let size = ww.get_size();
                self.lib.resize_window(w, size.width - 2* CONFIG.border_width as u32, size.height - 2*CONFIG.border_width as u32);
                self.lib.raise_window(w);
            }
        }
        // need to rethink focus for non floating modes
        info!("focus_w: {}", self.focus_w);
    }

    pub fn unset_focus(&mut self, w: Window) {
        let ww = match self.current_monitor().get_client(w).map(|x| *x).clone() {
            Some(ww) => ww,
            None => { return }
        };
        match ww.get_dec() {
            Some(dec) => {
                self.lib.set_border_color(dec, CONFIG.background_color);
                self.lib.set_window_background_color(dec, CONFIG.background_color);
            },
            None => {
                self.lib.set_border_width(w, 0);
                self.lib.set_border_color(ww.window(), CONFIG.background_color);
                let size = ww.get_size();
                self.lib.resize_window(w, size.width, size.height);
            }
        }
    }

    fn window_initial_size(&mut self, w: Window) {

        if self.mode != Mode::Floating {
            return
        }

        let screen = self.get_focused_screen();
        if let Some(dock_rect) = self.dock_area.as_rect(&screen) {
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
        if !self.current_monitor().contains_window(w) {
            debug!("toggle_maximize: Window not in client list: {}", w);
            debug!("Client list:");
            self.current_monitor().get_client_keys()
                .iter()
                .for_each(|key| {
                    println!("Client: {}", key)
                });
            return
        }

        let ww = self.current_monitor().get_client(w).unwrap();
        let (state, restore_pos) = (ww.get_window_state(), ww.get_restore_position());

        match state {
            WindowState::Maximized => {
                let others = self.current_monitor()
                    .get_current_windows()
                    .into_iter()
                    .filter(|win| *win != w)
                    .collect::<Vec<Window>>();

                others
                    .into_iter()
                    .for_each(|win| {
                        self.show_client(win)
                    });
                self.show_client(w);
                let ww = self.current_monitor().get_client_mut(w).expect("How can it not be in list?!");
                ww.restore_prev_state();
                let size = ww.get_restore_size();
                self.resize_window(w, size.width, size.height);
                self.move_window(w,restore_pos.x, restore_pos.y);
            },
            _ => {
                self.current_monitor().get_client_mut(w).expect("Not in list?!").save_restore_position();
                let screen = self.get_focused_screen();
                self.move_window(w, screen.x, screen.y);
                let size = self.current_monitor().maximize(w);
                let others = self.current_monitor()
                    .get_current_windows()
                    .into_iter()
                    .filter(|win| *win != w)
                    .collect::<Vec<Window>>();

                others
                    .into_iter()
                    .for_each(|win| {
                        self.hide_client(win)
                    });
                let ww = self.current_monitor().get_client_mut(w).expect("Not in list?!");
                ww.set_window_state(WindowState::Maximized);
                ww.save_restore_size();
                self.resize_window(w, size.width, size.height);
            }
        }
        /*let ww = self.clients.get(&w).expect("Not in list? {WindowManager::maximize}");
          match ww.get_dec() {
          Some(dec) => {
          self.lib.raise_window(dec);
          self.lib.raise_window(w);
          },
          None => self.lib.raise_window(w)
          }*/
        self.set_focus(w);
        self.center_cursor(w);
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) {
        if !self.current_monitor().contains_window(w) {
            return
        }

        let (dec_size, window_size) = self.current_monitor().resize_window(w, width, height);

        let win = self.current_monitor().get_client_mut(w).expect("Client not found in resize_window").window();
        let dec_win = self.current_monitor().get_client_mut(w).expect("Client not found in resize_window").get_dec();


        if let Some(dec_win) = dec_win {
            self.current_monitor().get_client_mut(w).expect("Client not found in resize_window").set_dec_size(dec_size);
            self.lib.resize_window(dec_win, dec_size.width, dec_size.height);
        }

        self.current_monitor().get_client_mut(w).expect("Client not found in resize_window").set_inner_size(window_size);
        self.lib.resize_window(win, window_size.width, window_size.height);
    }

    fn subscribe_to_events(&mut self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask | FocusChangeMask | PropertyChangeMask
        );
    }
    pub fn shift_window(&mut self, w: Window, direction: Direction) {
        if !self.current_monitor().contains_window(w) {
            return
        }

        let (pos, size) = self.current_monitor().shift_window(w, direction);
        self.move_window(w, pos.x, pos.y);
        self.resize_window(w, size.width, size.height);
        self.set_focus(w);
        self.center_cursor(w);
        self.current_monitor().get_client_mut(w).unwrap().set_window_state(WindowState::Snapped);
    }

    fn place_window(&mut self, w: Window) {
        if !self.current_monitor().contains_window(w) {
            return
        }

        let pos = self.current_monitor().place_window(w);
        self.move_window(w, pos.x, pos.y);
        let ww = self.current_monitor().get_client_mut(w).unwrap();
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
        let mut ww = match self.current_monitor().get_client(w) {
            Some(ww) => ww.clone(),
            None => { return }
        };
        let (outer_pos, inner_pos) = self.current_monitor().move_window(w, x, y);

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
                let ww = self.current_monitor().get_client_mut(w).unwrap();
                ww.set_position(outer_pos);
            },
            None => {
                self.lib.move_window(
                    ww.window(),
                    outer_pos
                );
                let ww = self.current_monitor().get_client_mut(w).unwrap();
                ww.set_position(outer_pos);
            }
        }
    }

    pub fn get_focused_screen(&self) -> Screen {
        let screens = self.lib.get_screens()
            .into_iter()
            .filter(|screen| self.pointer_is_inside(screen))
            .collect::<Vec<Screen>>();


        debug!("focused screens len: {}", screens.len());
        let focused = screens.get(0).expect("No screen dafuq?").clone();
        focused
    }

    pub fn pointer_is_inside(&self, screen: &Screen) -> bool {
        let pointer_pos = self.lib.pointer_pos();
        debug!("pointer pos: {:?}", pointer_pos);
        let inside_height = pointer_pos.y >= screen.y &&
            pointer_pos.y <= screen.y + screen.height as i32;

        let inside_width = pointer_pos.x >= screen.x &&
            pointer_pos.x <= screen.x + screen.width as i32;

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

    pub fn center_cursor(&mut self, w: Window) {
        let ww = match self.current_monitor().get_client(w) {
            Some(ww) => ww.clone(),
            None => { return }
        };
        self.lib.center_cursor(&ww)
    }

    pub fn kill_window(&mut self, w: Window) {
        if !self.current_monitor().contains_window(w) || w == self.lib.get_root() {
            return;
        }

        let frame = self.current_monitor().get_client(w).expect("KillWindow: No such window in client list").clone();

        if self.lib.kill_client(frame.window()) {
            if frame.is_decorated() {
                self.destroy_dec(frame);
            }
            self.current_monitor().remove_window(w);
            let clients: Vec<Window> = self.current_monitor()
                .get_client_keys()
                .iter()
                .map(|c| {
                    *c
                }).collect();
            self.lib.update_net_client_list(clients);
        }
        info!("Top level windows: {}", self.lib.top_level_window_count());
    }

    pub fn grab_keys(&self, w: Window) {

        let _keys = vec!["q",
        "Left",
        "Up",
        "Right",
        "Down",
        "Return",
        "f",
        "e",
        "h", "j", "k", "l",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"]
            .iter()
            .map(|key| { keysym_lookup::into_keysym(key).expect("Core: no such key") })
            .for_each(|key_sym| { self.lib.grab_keys(w, key_sym, Mod4Mask | Shift) });
    }

    pub fn grab_buttons(&self, w: Window) {
        let buttons = vec![Button1, Button3];
        buttons
            .iter()
            .for_each(|button| {
                self.lib.grab_button(
                    *button,
                    Mod4Mask,
                    w,
                    false,
                    (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                    GrabModeAsync,
                    GrabModeAsync,
                    0,0);
            });
        buttons
            .iter()
            .for_each(|button| {
                self.lib.grab_button(
                    *button,
                    0,
                    w,
                    false,
                    (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                    GrabModeAsync,
                    GrabModeAsync,
                    0,0);
            });
    }
}
