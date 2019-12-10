#![allow(dead_code)]
use std::collections::{
    HashMap,
};
use std::rc::Rc;

use crate::{
    HadlockResult,
    HadlockOption,
    config::*,
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
                    monitors.insert(i as u32, Monitor::new(i as u32, val.clone(), Workspace::new(i as u32)));
                });
            let mon_count = monitors.iter().count();
            debug!("Monitor on start: {}", mon_count);
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

    fn get_all_active_ws(&self) -> Vec<u32> {
        self.monitors
            .values()
            .map(|mon| mon.get_active_ws_tags())
            .flatten()
            .collect::<Vec<u32>>()
    }

    pub fn current_monitor(&mut self) -> Option<&mut Monitor> {
        self.monitors.get_mut(&self.current_monitor)
    }

    pub fn set_current_monitor_by_mouse(&mut self) {
        let mon_vec = self.monitors
            .iter()
            .filter(|(_key, mon)| self.pointer_is_inside(&mon.screen))
            .map(|(key, _mon)| *key)
            .collect::<Vec<u32>>();
        match mon_vec.get(0) {
            Some(mon_id) => self.current_monitor = *mon_id,
            None => { return }
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
    
    fn window_inside_screen(w_geom: &Geometry, screen: &Screen) -> bool {
        let inside_width = w_geom.x >= screen.x && w_geom.x < screen.x + screen.width;
        let inside_height = w_geom.y >= screen.y && w_geom.y < screen.y + screen.height; 
        inside_width && inside_height
    }


    pub fn setup_window(&mut self, w: Window) -> HadlockOption<()> {
        warn!("setup window");
        if self.lib.get_window_type(w) == WindowType::Dock {
            match self.lib.get_window_strut_array(w) {
                Some(dock) => {
                    let w_geom = self.lib.get_geometry(w);
                    let mon = self.monitors
                        .values_mut()
                        .filter(|mon| Self::window_inside_screen(&w_geom, &mon.screen))
                        .collect::<Vec<&mut Monitor>>().remove(0);
                    mon.set_dock_area(dock);
                }
                None => {
                    return None
                }
            }
        }

        if !self.should_be_managed(w) {
            warn!("{} should not be managed", w);
            return None
        }

        if self.current_monitor().expect("setup_window: current monitor 2").contains_window(w) {
            warn!("Current monitor already contains: {}", w);
            return None
        }

        if self.client_exist(w) { return None}

        let geom = self.lib.get_geometry(w);
        let mut ww = WindowWrapper::new(w, Rect::from(geom));
        if self.decorate {
            self.decorate_window(&mut ww);
        }
        self.lib.add_to_save_set(w);
        self.lib.add_to_root_net_client_list(w);
        self.subscribe_to_events(w);
        self.current_monitor()?.add_window(w, ww);
        self.window_initial_size(w);
        self.place_window(w);
        self.show_client(w);
        self.raise_window(&ww);
        Some(())
    }

    fn client_exist(&self, w: Window) -> bool {
        self.monitors
            .values()
            .filter(|mon| mon.contains_window(w))
            .count() > 0
    }

    pub fn hide_client(&mut self, w: Window) -> HadlockOption<()> {
        let client = match self.current_monitor()?.get_client_mut(w) {
            Some(client) => client,
            None => { 
                debug!("no such client");
                return None
            }
        };
        debug!("Hide client: {}", w);
        client.save_restore_position();
        match client.get_dec() {
            Some(dec) => {
                self.lib.unmap_window(dec);
                self.lib.unmap_window(w);
            },
            None => self.lib.unmap_window(w)
        }
        self.lib.sync(true);
        Some(())
    }

    fn show_client(&mut self, w: Window) -> HadlockOption<()> {
        let client = match self.current_monitor()?.get_client_mut(w) {
            Some(client) => client,
            None => { return None }
        };
        debug!("show client: {}", w);
        let pos = client.get_restore_position();
        client.set_position(pos);

        match client.get_dec() {
            Some(dec) => {
                self.lib.map_window(w);
                self.lib.map_window(dec);
            },
            None => self.lib.map_window(w)
        }
        self.lib.sync(true);
        Some(())
    }

    fn is_ws_visible(&self, ws: u32) -> bool {
        let ret = self.monitors
            .values()
            .filter(|mon| mon.get_current_ws_tag() == ws)
            .count() > 0;
        debug!("ws:{} is visible: {}",ws,ret);
        ret
    }

    fn get_monitor_by_ws(&mut self, ws: u32) -> Option<&mut Monitor> {
        let mon = self.monitors
            .values_mut()
            .filter(|mon| mon.contains_ws(ws))
            .collect::<Vec<&mut Monitor>>();
        mon.into_iter().nth(0)
    }

    pub fn set_current_ws(&mut self, ws: u32) -> HadlockOption<()> {
        info!("Desktop changed to: {}", ws);

        if ws == self.current_monitor().expect("set_current_ws: current_monitor 1").get_current_ws_tag() {
            return None
        }

        if self.current_monitor()?.contains_ws(ws) && !self.is_ws_visible(ws) {

            self.current_monitor()?
                .get_current_windows()
                .iter()
                .for_each(|win| {self.hide_client(*win);});

            self.current_monitor()?
                .set_current_ws(ws);

            self.current_monitor()?
                .get_current_windows()
                .iter()
                .for_each(|win| {self.show_client(*win);});

            let current = self.current_monitor()?.get_current_ws_tag();
            self.lib.update_desktops(current, None);
            return Some(())
        }

        if self.is_ws_visible(ws) {
            match self.get_monitor_by_ws(ws) {
                Some(mon) => {
                    mon.set_current_ws(ws);
                    let pos = Position {
                        x: mon.screen.x + mon.screen.width / 2,
                        y: mon.screen.y + mon.screen.height / 2
                    };
                    // show clients again?
                    self.current_monitor = mon.id;
                    self.lib.sync(false);
                    self.lib.move_cursor(pos);
                    self.unset_focus(self.focus_w);
                    self.current_monitor()?.set_current_ws(ws as u32);
                    let current = self.current_monitor()?.get_current_ws_tag();
                    self.current_monitor()?
                        .get_current_windows()
                        .iter()
                        .for_each(|win| {self.show_client(*win);});
                    self.lib.update_desktops(current, None);
                    return Some(())
                },
                None => {return None}
            }
        }

        // no monitor has ws
        match self.get_monitor_by_ws(ws) {
            None => {
                self.current_monitor()?
                    .get_current_windows()
                    .iter()
                    .for_each(|win| {self.hide_client(*win);});
                self.current_monitor()?
                    .set_current_ws(ws);

                self.current_monitor()?
                    .get_current_windows()
                    .iter()
                    .for_each(|win| {self.show_client(*win);});
                let current = self.current_monitor()?.get_current_ws_tag();
                self.lib.update_desktops(current, None);
            },
            _ => {}
        }

        // other monitor has ws and it is not visible
        let(mon_id, pos) = match self.get_monitor_by_ws(ws) {
            Some(mon) => {
                let pos = Position {
                    x: mon.screen.x + mon.screen.width / 2,
                    y: mon.screen.y + mon.screen.height / 2
                };
                (mon.id, pos)
            },
            None => (self.current_monitor, self.lib.pointer_pos())
        };
        self.current_monitor = mon_id;
        self.lib.move_cursor(pos);
        self.set_current_monitor_by_mouse();
        self.current_monitor()?
            .get_current_windows()
            .iter()
            .for_each(|win| {self.hide_client(*win);});
        self.current_monitor()?
            .set_current_ws(ws);

        self.current_monitor()?
            .get_current_windows()
            .iter()
            .for_each(|win| {self.show_client(*win);});
        let current = self.current_monitor()?.get_current_ws_tag();
        self.lib.update_desktops(current, None);
        Some(())
    }

    fn print_monitors(&self) {
        self.monitors
            .values()
            .for_each(|mon| debug!("{:#?}", mon));
    }

    pub fn move_to_ws(&mut self, w: Window, ws: u8) -> HadlockOption<()> {
        debug!("move_to_ws called with: {}", w);
        if w != self.focus_w {
            debug!("Trying to move {}, and it is not focus_w: {}", w, self.focus_w);
            return None
        }
    
        let current_ws = self.current_monitor()?.get_current_ws_tag();

        if ws as u32 == current_ws {
            debug!("Trying to move {} from {} to {}", w, ws, current_ws);
            return None
        }

        let w = self.focus_w;

        if let None = self.get_monitor_by_ws(ws as u32) {
            let temp = self.current_monitor()?.get_current_ws_tag();
            let ww = self.current_monitor().expect("move_to_ws: current_monitor 2").remove_window(w);
            self.current_monitor()?.set_current_ws(ws as u32);
            self.current_monitor()?.add_window(w, ww);
            self.current_monitor()?.set_current_ws(temp);
        } else {
            self.hide_client(w);
            let current = self.current_monitor()?.id;
            let ww = self.current_monitor()?.remove_window(w);
            let mon = self.get_monitor_by_ws(ws.into()).unwrap();
            let temp = mon.id;
            self.current_monitor = temp;
            self.current_monitor()?.add_window_to_ws(w, ww, ws.into());
            self.place_window(w);
            self.current_monitor = current;
        }
        Some(())
    }

    pub fn set_focus(&mut self, w: Window) -> HadlockOption<()> {
        let root = self.lib.get_root();
        let client = self.current_monitor()?.get_client(w).map(|x| *x).clone();
        let ww = match client {
            Some(ww) => ww,
            None if w == root => {
                let root = self.lib.get_root();
                self.focus_w = root;
                self.lib.take_focus(self.focus_w);
                return Some(())
            }
            None => { return None}
        };

        //self.unset_focus(self.focus_w);
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
        Some(())
    }

    pub fn unset_focus(&mut self, w: Window) -> HadlockOption<()> {
        let ww = match self.current_monitor()?.get_client(w).map(|x| *x).clone() {
            Some(ww) => ww,
            None => { return None}
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
        Some(())
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

    pub fn toggle_maximize(&mut self, w: Window) -> HadlockOption<()> {
        // TODO: cleanup this mess...
        if !self.current_monitor()?.contains_window(w) {
            debug!("toggle_maximize: Window not in client list: {}", w);
            debug!("Client list:");
            self.current_monitor()?.get_client_keys()
                .iter()
                .for_each(|key| {
                    println!("Client: {}", key)
                });
            return None
        }


        let ww = self.current_monitor()?.get_client_mut(w)?;

        let (state, restore_pos) = (ww.get_window_state(), ww.get_restore_position());

        match state {
            WindowState::Maximized => {
                let others = self.current_monitor()?
                    .get_current_windows()
                    .into_iter()
                    .filter(|win| *win != w)
                    .collect::<Vec<Window>>();

                others
                    .into_iter()
                    .for_each(|win| {
                        self.show_client(win);
                    });
                self.show_client(w);
                let ww = self.current_monitor()?.get_client_mut(w)?;
                ww.restore_prev_state();
                let size = ww.get_restore_size();
                self.resize_window(w, size.width, size.height);
                self.move_window(w,restore_pos.x, restore_pos.y);
            },
            _ => {
                ww.save_restore_position();
                ww.set_window_state(WindowState::Maximized);
                ww.save_restore_size();
                self.current_monitor()?.get_client_mut(w)?.save_restore_position();

                let screen = self.get_focused_screen();
                self.move_window(w, screen.x, screen.y);
                let size = self.current_monitor()?.maximize(w);
                let others = self.current_monitor()?
                    .get_current_windows()
                    .into_iter()
                    .filter(|win| *win != w)
                    .collect::<Vec<Window>>();

                others
                    .into_iter()
                    .for_each(|win| {
                        self.hide_client(win);
                    });
                self.resize_window(w, size.width, size.height);
            }
        }
        self.set_focus(w);
        self.center_cursor(w);
        Some(())
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) -> HadlockOption<()> {
        if !self.current_monitor()?.contains_window(w) {
            return None
        }

        let (dec_size, window_size) = self.current_monitor()?.resize_window(w, width, height);

        let win = self.current_monitor()?.get_client_mut(w)?.window();
        let dec_win = self.current_monitor()?.get_client_mut(w)?.get_dec();


        if let Some(dec_win) = dec_win {
            self.current_monitor()?.get_client_mut(w)?.set_dec_size(dec_size);
            self.lib.resize_window(dec_win, dec_size.width, dec_size.height);
        }

        self.current_monitor()?.get_client_mut(w)?.set_inner_size(window_size);
        self.lib.resize_window(win, window_size.width, window_size.height);
        Some(())
    }

    fn subscribe_to_events(&mut self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask | FocusChangeMask | PropertyChangeMask
        );
    }

    pub fn shift_window(&mut self, w: Window, direction: Direction) -> HadlockOption<()> {
        if !self.current_monitor()?.contains_window(w) {
            return None
        }

        let (pos, size) = self.current_monitor()?.shift_window(w, direction);
        self.move_window(w, pos.x, pos.y);
        self.resize_window(w, size.width, size.height);
        self.set_focus(w);
        self.center_cursor(w);
        self.current_monitor()?.get_client_mut(w)?.set_window_state(WindowState::Snapped);
        Some(())
    }
    


    pub fn place_window(&mut self, w: Window) -> HadlockOption<()> {
        if !self.current_monitor()?.contains_window(w) {
            return None

        }

        let (size, pos) = self.current_monitor()?.place_window(w);
        self.move_window(w, pos.x, pos.y);
        self.resize_window(w, size.width, size.height);

        let ww = self.current_monitor()?.get_client_mut(w)?;

        ww.save_restore_position();
        ww.set_position(pos);
        Some(())
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

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) -> HadlockOption<()> {
        let ww = match self.current_monitor()?.get_client(w) {
            Some(ww) => ww.clone(),
            None => { return None }
        };
        let (outer_pos, inner_pos) = self.current_monitor()?.move_window(w, x, y);

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
                let ww = self.current_monitor()?.get_client_mut(w)?;
                ww.set_position(outer_pos);
                Some(())
            },
            None => {
                self.lib.move_window(
                    ww.window(),
                    outer_pos
                );
                let ww = self.current_monitor()?.get_client_mut(w)?;
                ww.set_position(outer_pos);
                Some(())
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
        //debug!("pointer pos: {:?}", pointer_pos);
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

    pub fn center_cursor(&mut self, w: Window) -> HadlockOption<()> {
        let ww = match self.current_monitor()?.get_client(w) {
            Some(ww) => ww.clone(),
            None => { return None }
        };
        self.lib.center_cursor(&ww);
        Some(())
    }

    pub fn kill_window(&mut self, w: Window) -> HadlockOption<()> {
        if !self.current_monitor()?.contains_window(w) || w == self.lib.get_root() {
            return None
        }

        let frame = self.current_monitor()?.get_client(w)?.clone();

        if self.lib.kill_client(frame.window()) {
            if frame.is_decorated() {
                self.destroy_dec(frame);
            }
            self.current_monitor()?.remove_window(w);
            let clients: Vec<Window> = self.current_monitor()?
                .get_client_keys()
                .iter()
                .map(|c| {
                    *c
                }).collect();
            self.lib.update_net_client_list(clients);
        }
        info!("Top level windows: {}", self.lib.top_level_window_count());
        Some(())
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
        "c",
        "h", "j", "k", "l",
        "d",
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
    }
}
