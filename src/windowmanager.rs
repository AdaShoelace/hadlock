#![allow(dead_code)]
use libc::{c_int, c_uint};
use std::collections::HashMap;
use crate::xlibwrapper::masks::*;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::util::*;
use crate::xlibwrapper::xlibmodels::*;
use crate::models::{
    windowwrapper::*,
    rect::*,
    screen::*,
    dockarea::*,
    window_type::*,
};

use std::rc::Rc;

pub const DecorationHeight: i32 = 20;
pub const BorderWidth: i32 = 2;
pub const InnerBorderWidth: i32 = 0;

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
    pub drag_start_frame_size: (c_uint, c_uint)
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
            drag_start_frame_size: (0, 0)
        }
    }

    fn decorate_window(&self, w: &mut WindowWrapper) {
        let window_geom = self.lib.get_geometry(w.window());
        let position = Position { x: window_geom.x - InnerBorderWidth - BorderWidth,
        y: window_geom.y - InnerBorderWidth - BorderWidth - DecorationHeight };

        let size = Size { width: window_geom.width + (2 * InnerBorderWidth as u32), height: window_geom.height + (2 * InnerBorderWidth as u32) + DecorationHeight as u32 };
        let dec_window = self.lib.create_simple_window(
            self.lib.get_root(),
            position.clone(),
            size.clone(),
            BorderWidth as u32,
            Color::SolarizedPurple,
            Color::SolarizedPurple
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

        //self.grab_keys(dec_window);
        w.set_decoration(dec_window, Rect::new(position, size));
        self.lib.map_window(dec_window);
        self.lib.sync(false);
    }

    pub fn setup_window(&mut self, w: Window) {

        if self.lib.get_window_type(w) == WindowType::Dock {
            self.dock_area = match self.lib.get_window_strut_array(w) {
                Some(dock) => dock,
                None => { return }
            }
        }

        if !self.should_be_managed(w) {
            return
        }

        if self.clients.contains_key(&w) {
            return
        }

        let geom = self.lib.get_geometry(w);
        let mut ww = WindowWrapper::new(w, Rect::from(geom));
        self.lib.set_border_width(w, InnerBorderWidth as u32);
        self.lib.set_border_color(w, Color::SolarizedPurple);
        self.decorate_window(&mut ww);
        self.lib.add_to_save_set(w);
        self.lib.add_to_root_net_client_list(w);
        self.lib.ungrab_all_buttons(w);
        self.subscribe_to_events(w);
        self.grab_buttons(w);
        self.grab_keys(w);
        self.clients.insert(w, ww);
        self.window_initial_size(w);
        self.center_window(w);
        self.lib.map_window(w);
        self.lib.raise_window(w);
    }

    fn window_initial_size(&mut self, w: Window) {

        if self.mode != Mode::Floating {
            return
        }

        let screen = self.lib.get_screen();
        if let Some(dock_rect) = self.dock_area.as_rect(screen.width, screen.height) {
            println!("DockArea: {:?}", dock_rect);
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

        if let Some(dock_rect) = self.dock_area.as_rect(screen.width, screen.height) {
            println!("HEFFAKLUMP: {}", dock_rect.get_size().height);
            dh = ((screen.height + dock_rect.get_size().height as i32) - ww.get_height() as i32).abs() / 2;
        }

        println!("Screen width: {} Screen height: {}", screen.width, screen.height);
        println!("Window width: {} Window height: {}", ww.get_width(), ww.get_height());

        println!("dw: {}, dh: {}", dw, dh);
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
        match ww.get_dec() {
            Some(dec) => {
                self.lib.move_window(
                    dec,
                    x,
                    y
                );
                self.lib.move_window(
                    ww.window(),
                    x + InnerBorderWidth + BorderWidth,
                    y + InnerBorderWidth + BorderWidth + DecorationHeight
                );
                ww.set_position(Position { x, y });
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
        println!("Window pos: {:?}", ww.get_position());
    }

    fn toggle_decorations(&mut self) {
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

        let mut ww = match self.clients.get_mut(&w) {
            Some(ww) => ww,
            None => {
                return;
            }
        };

        if let Some(dec_rect) = ww.get_dec_rect() {
            let mut dec_w = width - (2 * BorderWidth as u32);
            let mut dec_h = height - (2 * BorderWidth as u32);

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
        let mut d_width = width - (2* InnerBorderWidth as u32) - (2 * BorderWidth as u32);
        let mut d_height = height - (2* InnerBorderWidth as u32) - (2 * BorderWidth as u32) - DecorationHeight as u32;

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

    fn grab_keys(&self, w: Window) {

        let _keys = vec!["q", "Left", "Up", "Right", "Down"]
            .iter()
            .map(|key| { keysym_lookup::into_keysym(key).expect("Core: no such key") })
            .for_each(|key_sym| { self.lib.grab_keys(w, key_sym, Mod4Mask | Shift) });
    }

    fn grab_buttons(&self, w: Window) {
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
