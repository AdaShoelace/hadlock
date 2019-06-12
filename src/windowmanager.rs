
use std::ptr;
use x11_dl::xlib;
use x11_dl::xlib::{Display, Window};
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CString, CStr};
use std::mem;
use std::collections::HashMap;

unsafe extern "C" fn error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int {
    println!("Unknown error occured!");
    return 0;
}

unsafe extern "C" fn on_wm_detected(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    if (*e).error_code == xlib::BadAccess {
        panic!("Other wm registered!")
    }
    return 0;
}


pub struct WindowManager {
    display: *mut Display,
    root : Window,
    clients: HashMap<u64, xlib::Window>,
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
        unsafe {
            let xlib: xlib::Xlib = xlib::Xlib::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null_mut());

            if display == std::ptr::null_mut() {
                // Log with simplelog in the future
                let disp_name = CString::from_raw((xlib.XDisplayName)(display as *const c_char));
                println!("Failed to open display: {}", disp_name.to_str().unwrap());
            }

            // let screen = (xlib.XDefaultScreenOfDisplay)(display);
            // let root = (xlib.XRootWindowOfScreen)(screen);
            let root = (xlib.XDefaultRootWindow)(display);
            Self {
                display: display,
                root: root,
                clients: HashMap::new(),
                drag_start_pos: (0, 0),
                drag_start_frame_pos: (0, 0),
                drag_start_frame_size: (0, 0)
            }
        }
    }

    pub fn run(&mut self) {
        unsafe {
            let lib: xlib::Xlib = xlib::Xlib::open().unwrap();

            let cstring = CString::from_raw((lib.XDisplayName)(self.display as *const c_char));
            // println!("Display name: {}\nroot: {}", cstring.to_str().unwrap(), self.root);

            (lib.XSetErrorHandler)(Some(on_wm_detected));

            (lib.XSelectInput)(
                self.display,
                self.root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask
            );

            (lib.XSync)(self.display, 0);
            (lib.XSetErrorHandler)(Some(error_handler));

            loop {
                let mut event = xlib::XEvent { pad: [0;24] };
                (lib.XNextEvent)(self.display, &mut event);

                match event.get_type() {
                    xlib::CreateNotify => Self::on_create_notify(event.create_window),
                    xlib::ConfigureRequest => self.on_configure_request(event.configure_request),
                    xlib::MapRequest => self.on_map_request(event.map_request),
                    xlib::ButtonPress => self.on_button_pressed(event.button),
                    xlib::MotionNotify => self.on_motion_notify(event.motion),
                    _ => println!("Unknown event")
                }
            }

        }
    }

    unsafe fn on_map_request(&mut self, event: xlib::XMapRequestEvent) {
        let xlib: xlib::Xlib = xlib::Xlib::open().unwrap();
        self.frame(event.window);
        (xlib.XMapWindow)(self.display, event.window);
    }

    unsafe fn on_configure_request(&mut self, event: xlib::XConfigureRequestEvent) {
        let lib: xlib::Xlib = xlib::Xlib::open().unwrap();

        unsafe {
            let mut changes: xlib::XWindowChanges = mem::uninitialized();

            changes.x = event.x;
            changes.y = event.y;
            changes.width = event.width;
            changes.height = event.height;
            changes.border_width = event.border_width;
            changes.sibling = event.above;
            changes.stack_mode = event.detail;

            if self.clients.contains_key(&event.window) {
                let frame = self.clients.get(&event.window);
                (lib.XConfigureWindow)(self.display, *frame.unwrap(), event.value_mask as u32, &mut changes);
            }

            (lib.XConfigureWindow)(self.display, event.window, event.value_mask as u32, &mut changes);
        }
    }

    unsafe fn on_create_notify(event: xlib::XCreateWindowEvent) {
        println!("on_create_notify: called");
    }

    unsafe fn on_button_pressed(&mut self, event: xlib::XButtonPressedEvent) {
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
    }
    
    unsafe fn on_motion_notify(&mut self, event: xlib::XMotionEvent) {
        unimplemented!()
    }

    fn frame(&mut self, w: xlib::Window) {
        let lib: xlib::Xlib = xlib::Xlib::open().unwrap();
        const BORDER_WIDTH: c_uint = 3;
        const BORDER_COLOR: c_ulong = 0xff0000;
        const BG_COLOR: c_ulong = 0x0000ff;

        unsafe {
            let mut attributes: xlib::XWindowAttributes = std::mem::uninitialized();
            (lib.XGetWindowAttributes)(self.display, w, &mut attributes);
            let frame: xlib::Window = (lib.XCreateSimpleWindow)(
                self.display,
                self.root,
                attributes.x,
                attributes.y,
                attributes.width as u32,
                attributes.height as u32,
                BORDER_WIDTH,
                BORDER_COLOR,
                BG_COLOR);

            (lib.XSelectInput)(
                self.display,
                frame,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);
            (lib.XAddToSaveSet)(self.display, w);
            (lib.XReparentWindow)(
                self.display,
                w,
                frame,
                0,0);

            (lib.XMapWindow)(self.display, frame);
            self.clients.insert(w, frame);

            // move window with alt + right button
            (lib.XGrabButton)(
                self.display,
                xlib::Button3,
                xlib::Mod1Mask,
                w,
                0,
                (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::ButtonMotionMask) as u32,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                0,0);
            //(lib.XGrabButton)(...)
            //(lib.XGrabKey)(...)
            //(lib.XGrabKey)(...)
        }

        // TODO - see framing existing Top-Level windows section below

        //create frame
    }

}
