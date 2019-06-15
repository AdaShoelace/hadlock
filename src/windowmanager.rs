
use std::ptr;
use x11_dl::xlib;
// use x11_dl::xlib::{Display, Window};
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CString, CStr};
use std::mem;
use std::collections::HashMap;

use crate::xlibwrapper::*;
use crate::xlibwrapper::util::{Color, Position, Size};

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
            println!("{:?}", event);
            match event {
                Event::ConfigurationRequest(window, window_changes, value_mask) => self.on_configure_request(window, window_changes, value_mask),
                Event::WindowCreated(window) => self.on_map_request(window),
                // xlib::ButtonPress => self.on_button_pressed(event.button),
                // xlib::MotionNotify => self.on_motion_notify(event.motion),
                _ => println!("Unknown event")
            }
        }
    }

    fn on_map_request(&mut self, w: Window) {
        println!("on_map_request");
        self.frame(w);
        self.lib.map_window(w);
    }

    fn on_configure_request(&mut self, w: Window, window_changes: WindowChanges, value_mask: u64) {
        println!("on_configure_request");
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


    /*unsafe fn on_button_pressed(&mut self, event: xlib::XButtonPressedEvent) {
        let lib: xlib::Xlib = xlib::Xlib::open().unwrap();
        println!("On button pressed");
        if !self.clients.contains_key(&event.window) {
            return
        }

        unsafe {
            let frame = self.clients.get(&event.window).unwrap();
            self.drag_start_pos = (event.x_root, event.y_root);

            let mut returned_root: xlib::Window = mem::uninitialized();
            let mut x: c_int = 0;
            let mut y: c_int = 0;
            let (mut width, mut height, mut border_width, mut depth) = (0 as c_uint, 0 as c_uint, 0 as c_uint, 0 as c_uint);

            let check = (lib.XGetGeometry)(
                self.display,
                *frame,
                &mut returned_root,
                &mut x, &mut y,
                &mut width, &mut height,
                &mut border_width,
                &mut depth);

            if check == 0 {
                panic!("XGetGeometry");
            }
            self.drag_start_frame_pos = (x,y);
            self.drag_start_frame_size = (width, height);
            (lib.XRaiseWindow)(self.display, *frame);
        }
    }*/

    unsafe fn on_motion_notify(&mut self, event: xlib::XMotionEvent) {
        unimplemented!()
    }

    fn frame(&mut self, w: xlib::Window) {
        const BORDER_WIDTH: c_uint = 3;
        const BORDER_COLOR: Color = Color::RED;
        const BG_COLOR: Color = Color::BLUE;

            let attributes = self.lib.get_window_attributes(w);
            let parent = self.lib.get_root();
            let frame = self.lib.create_simple_window(
                parent,
                Position { x: attributes.x, y: attributes.y },
                Size { width: attributes.width as u32, height: attributes.height as u32 },
                BORDER_WIDTH,
                BORDER_COLOR,
                BG_COLOR);
            let mask = (SubstructureRedirectMask | SubstructureNotifyMask) as Mask;
            self.lib.select_input(
                frame,
                mask
            );
            self.lib.add_to_save_set(w);
            self.lib.reparent_window(w, frame);
            self.lib.map_window(frame);
            self.clients.insert(w, frame);

        unsafe {
            // move window with alt + right button
            /*(lib.XGrabButton)(
              self.display,
              xlib::Button3,
              xlib::Mod1Mask,
              w,
              0,
              (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::ButtonMotionMask) as u32,
              xlib::GrabModeAsync,
              xlib::GrabModeAsync,
              0,0);*/
            //(lib.XGrabButton)(...)
            //(lib.XGrabKey)(...)
            //(lib.XGrabKey)(...)
        }

        // TODO - see framing existing Top-Level windows section below

        //create frame
    }

}
