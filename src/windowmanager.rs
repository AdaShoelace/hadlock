
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::collections::HashMap;
use crate::xlibwrapper::masks::*;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::util::*;
use crate::models::windowwrapper::*;

pub const DecorationHeight: i32 = 15;
pub const BorderWidth: i32 = 2;
pub const InnerBorderWidth: i32 = 0;

pub struct WindowManager {
    lib:  XlibWrapper,
    clients: HashMap<u64, WindowWrapper>,
    drag_start_pos: (c_int, c_int),
    drag_start_frame_pos: (c_int, c_int),
    drag_start_frame_size: (c_uint, c_uint)
}

impl WindowManager {

    /*
     * Open a connection to X display
     * Check for failure
     * return WindowManager
     */
    pub fn new () -> Self {
        Self {
            lib: XlibWrapper::new(),
            clients: HashMap::new(),
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0)
        }
    }

    fn decorate_window(&self, w: &mut WindowWrapper) {
        let window_geom = self.lib.get_geometry(w.window()).expect("Failed to get window geometry");
        let dec_window = self.lib.create_simple_window(
            self.lib.get_root(),
            Position { x: window_geom.x - InnerBorderWidth - BorderWidth, y: window_geom.y - InnerBorderWidth - BorderWidth - DecorationHeight },
            Size { width: window_geom.width + 2 * InnerBorderWidth as u32, height: window_geom.height + 2 * (BorderWidth + DecorationHeight) as u32 },
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

        // self.grab_keys(dec_window);
        w.set_decoration(dec_window);
        self.lib.map_window(dec_window);
        self.lib.sync(false);
    }

    fn setup_window(&mut self, w: Window) {

        let mut ww = WindowWrapper::new(w);
        let inner_window = ww.window();
        self.lib.set_border_width(w, InnerBorderWidth as u32);
        self.lib.set_border_color(w, Color::SolarizedPurple);
        self.decorate_window(&mut ww);

        self.lib.add_to_save_set(inner_window);
        self.lib.add_to_root_net_client_list(inner_window);
        self.lib.ungrab_all_buttons(inner_window);
        self.subscribe_to_events(inner_window);
        self.grab_buttons(inner_window);
        self.grab_keys(inner_window);
        self.lib.map_window(inner_window);
        self.lib.raise_window(inner_window);
        self.clients.insert(w, ww.clone());
    }

    pub fn run(&mut self) {


        // manage windows created before wm
        self.lib.grab_server();
        let _ = self.lib.get_top_level_windows()
            .iter()
            .map(|w| {
                self.setup_window(*w)
            });
        self.lib.ungrab_server();

        loop {
            let event = self.lib.next_event();
            //println!("{:?}", &event);
            match event {
                Event::ConfigurationRequest(window, window_changes, value_mask) => self.on_configure_request(window, window_changes, value_mask),
                Event::WindowCreated(window) => self.on_map_request(window),
                Event::ButtonPressed(window, sub_window, button, x_root, y_root, state) => {
                    println!("Button pressed");
                    self.on_button_pressed(window, sub_window, button, x_root, y_root, state);
                },
                Event::KeyPress(window, state, keycode) => {
                    println!("keypress");
                    self.on_key_pressed(window, state, keycode)
                },
                Event::MotionNotify(window, x_root, y_root, state) => {
                    println!("On motion notify");
                    self.on_motion_notify(
                        window,
                        x_root,
                        y_root,
                        state
                    )
                },
                Event::EnterNotify(window) => self.on_enter(window),
                Event::LeaveNotify(window) => self.on_leave(window),
                _ => {}//println!("Unknown event")
            }
        }
    }

    fn on_map_request(&mut self, w: Window) {
        self.setup_window(w);
        //self.lib.map_window(w);
    }

    fn on_configure_request(&mut self, w: Window, window_changes: WindowChanges, value_mask: u64) {
        //println!("on_configure_request");
        let changes = WindowChanges {
            x: window_changes.x,
            y: window_changes.y,
            width: window_changes.width,
            height: window_changes.height,
            border_width: window_changes.border_width,
            sibling: window_changes.sibling,
            stack_mode: window_changes.stack_mode,
        };

        if self.clients.contains_key(&w) {
            let frame = self.clients.get(&w).expect("ConfigureWindow: No such window in client list");
            self.lib.configure_window(
                frame.window(),
                value_mask as i64,
                changes.clone()
            );
        }
        self.lib.configure_window(
            w,
            value_mask as i64,
            window_changes
        );
    }

    fn on_button_pressed(&mut self, window: Window, sub_window: Window, button: u32, x_root: u32, y_root: u32, state: u32) {
        if !self.clients.contains_key(&window) && window == self.lib.get_root() {
            return
        }

        let ww = self.clients.get(&window).expect("ButtonPressed: No such window in client list");
        let geometry = match self.lib.get_geometry(ww.window()) {
            Ok(g) => g,
            Err(err) => panic!(format!("Shit went south: {:?}", err))
        };

        self.drag_start_pos = (x_root as i32 , y_root as i32);
        self.drag_start_frame_pos = (geometry.x,geometry.y);
        self.drag_start_frame_size = (geometry.width, geometry.height);

        match ww.get_dec() {
            Some(dec) => {
                self.lib.raise_window(dec);
                self.lib.raise_window(ww.window());
            },
            None => self.lib.raise_window(ww.window())
        }
    }

    fn on_enter(&self, w: Window) {

        let ww = self.clients.get(&w).expect("OnEnter: No such window in client list");

        match ww.get_dec() {
            Some(dec) => {
                self.lib.set_border_color(dec, Color::SolarizedCyan);
            },
            None => {
                self.lib.set_border_color(w, Color::SolarizedPurple);
            }
        }
        self.lib.set_input_focus(w, RevertToParent, CurrentTime);
    }

    fn on_leave(&self, w: Window) {
        // this check is an ugly hack to not crash when decorations window gets destroyed before
        // client and client recieves an "OnLeave"-event
        if !self.clients.contains_key(&w) {
            return;
        }

        let ww = self.clients.get(&w).expect("OnLeave: No such window in client list");

        match ww.get_dec() {
            Some(dec) => self.lib.set_border_color(dec, Color::SolarizedPurple),
            None => self.lib.set_border_color(ww.window(), Color::SolarizedPurple)
        }
    }

    fn on_key_pressed(&mut self, w: Window, state: u32, keycode: u32) {
        if self.lib.key_sym_to_keycode(keysym_lookup::into_keysym("q").unwrap() as u64 ) == keycode as u8
            && (state & (Mod4Mask | Shift)) != 0 {
                self.kill_window(w);
            }
    }

    fn on_motion_notify(&mut self, w: Window, x_root: i32, y_root: i32, state: u32) {
        let frame = self.clients.get(&w).expect("MotionNotify: No such window in client list").window();

        let drag_pos = Position { x: x_root, y: y_root };
        let (delta_x, delta_y) =  (drag_pos.x - self.drag_start_pos.0,
                                   drag_pos.y - self.drag_start_pos.1);
        let dest_pos = Position{ x: self.drag_start_frame_pos.0 + delta_x,
        y: self.drag_start_frame_pos.1 + delta_y};

        if (state & Button1Mask) != 0 {
            let ww = *self.clients.get(&w).unwrap();
            self.move_window(ww, dest_pos.x, dest_pos.y);
        }
    }

    fn move_window(&self, ww: WindowWrapper, x: i32, y: i32) {
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
                    y + DecorationHeight + BorderWidth + DecorationHeight
                );
            },
            None => self.lib.move_window(
                ww.window(),
                x,
                y
            )
        }
    }

    fn kill_window(&mut self, w: Window) {
        if !self.clients.contains_key(&w) {
            return;
        }

        let frame = self.clients.get(&w).expect("KillWindow: No such window in client list");


        match frame.get_dec() {
            Some(dec) => {
                self.lib.unmap_window(dec);
                self.lib.destroy_window(dec);
                self.lib.kill_client(frame.window())
            },
            None => self.lib.kill_client(frame.window())
        }
        self.clients.remove(&w);
    }

    fn subscribe_to_events(&mut self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask
        );
    }

    fn grab_keys(&self, w: Window) {

        let keys: Vec<u32> =
            vec!["q"]
            .into_iter()
            .map(|key| {
                keysym_lookup::into_keysym(key).unwrap()
            }).map(|keysym| {
                self.lib.key_sym_to_keycode(keysym as u64) as u32
            }).collect();


        let q_sym = keysym_lookup::into_keysym("q").expect("Apparently q is not a keysym");
        self.lib.grab_key(
            self.lib.key_sym_to_keycode(q_sym as u64) as u32,
            Mod4Mask | Shift,
            w,
            false,
            GrabModeAsync,
            GrabModeAsync);
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
