
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::collections::HashMap;
use crate::xlibwrapper::masks::*;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::util::*;


pub struct WindowManager {
    lib:  XlibWrapper,
    clients: HashMap<u64, Window>,
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

    pub fn run(&mut self) {
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
        //println!("on_map_request");
        self.setup_window(w);
        self.lib.map_window(w);
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
            let frame = self.clients.get(&w);
            self.lib.configure_window(
                *frame.unwrap(),
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
        //println!("On button pressed");
        if !self.clients.contains_key(&window) {
            return
        }

        let frame = self.clients.get(&window).unwrap();
        let geometry = match self.lib.get_geometry(*frame) {
            Ok(g) => g,
            //Err(err) => panic!(err)
            Err(err) => panic!(format!("Shit went south: {:?}", err))
        };

        self.drag_start_pos = (x_root as i32 , y_root as i32);
        self.drag_start_frame_pos = (geometry.x,geometry.y);
        self.drag_start_frame_size = (geometry.width, geometry.height);


        if let Err(e) = self.lib.raise_window(*frame) {
            println!("Ole dole doff");
            println!("{}", e);
        }
    }

    fn on_enter(&self, w: Window) {
        //self.lib.unmap_window(w);
        self.lib.set_border_width(w, 3);
        self.lib.set_border_color(w);
        //self.lib.map_window(w);
    }

    fn on_leave(&self, w: Window) {
        self.lib.set_border_width(w, 0);
    }

    fn on_key_pressed(&mut self, w: Window, state: u32, keycode: u32) {
        if self.lib.key_sym_to_keycode(keysym_lookup::into_keysym("q").unwrap() as u64 ) == keycode as u8 && (state & (Mod4Mask | Shift)) != 0 {
            self.kill_window(w);
        }
    }

    fn on_motion_notify(&mut self, w: Window, x_root: i32, y_root: i32, state: u32) {
        let frame = self.clients.get(&w).unwrap();

        let drag_pos = Position { x: x_root, y: y_root };
        let (delta_x, delta_y) =  (drag_pos.x - self.drag_start_pos.0,
                                   drag_pos.y - self.drag_start_pos.1);
        let dest_pos = Position{ x: self.drag_start_frame_pos.0 + delta_x,
        y: self.drag_start_frame_pos.1 + delta_y};

        if (state & Button1Mask) != 0 {
            self.lib.move_window(
                *frame,
                dest_pos.x,
                dest_pos.y
            );
        }
    }

    fn kill_window(&mut self, w: Window) {
        if !self.clients.contains_key(&w) {
            return;
        }

        let frame = self.clients.get(&w).unwrap();

        self.lib.kill_client(*frame);
        // self.lib.destroy_window(*frame);
        self.clients.remove(&w);
        let clients = self.clients.len();
    }

    fn setup_window(&mut self, w: Window) {
        const BORDER_WIDTH: c_uint = 3;
        const BORDER_COLOR: Color = Color::RED;
        const BG_COLOR: Color = Color::BLUE;

        let attributes = self.lib.get_window_attributes(w);
        let parent = self.lib.get_root();

        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask 
        );

        //self.lib.configure_window()
        self.lib.add_to_save_set(w);
        self.lib.map_window(w);
        self.lib.change_frame_property(w);
        self.clients.insert(w, w);

        // move window super + mouse1
        self.lib.grab_button(
            Button1,
            Mod4Mask,
            w,
            false,
            (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,0);

        //test implementation (redo with alt + f4)
        //kill window with super + mouse2
        self.lib.grab_button(
            Button3,
            Mod4Mask,
            w,
            false,
            (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
            GrabModeAsync,
            GrabModeAsync,
            0,0);

        let q_sym = keysym_lookup::into_keysym("q").expect("Apparently q is not a keysym");
        self.lib.grab_key(
            self.lib.key_sym_to_keycode(q_sym as u64) as u32,
            Mod4Mask | Shift,
            w,
            false,
            GrabModeAsync,
            GrabModeAsync);
        //self.lib.grab_key

        //(lib.XGrabKey)(...)
        // TODO - implement keygrabbing/keyevent
        // TODO - implement closing of windows by protocols/atom else by KillClient


        //create frame
    }
}
