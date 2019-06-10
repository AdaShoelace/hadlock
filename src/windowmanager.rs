
use std::ptr;
use x11_dl::xlib;
use x11_dl::xlib::{Display, Window};
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CString, CStr};
use std::mem;
use std::process::exit;

unsafe extern "C" fn error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int {
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
            }
        }
    }

    pub fn run(&mut self) {
        unsafe {
            let xlib: xlib::Xlib = xlib::Xlib::open().unwrap();

            let cstring = CString::from_raw((xlib.XDisplayName)(self.display as *const c_char));
            println!("Display name: {}\nroot: {}", cstring.to_str().unwrap(), self.root);
            
            (xlib.XSetErrorHandler)(Some(on_wm_detected));

            (xlib.XSelectInput)(
                self.display,
                //self.root + 1,
                self.root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask
            );

            (xlib.XSync)(self.display, 0);
            (xlib.XSetErrorHandler)(Some(error_handler));

            println!("Pre loop");
            loop {
            println!("entered loop");
                let e: *mut xlib::XEvent = mem::uninitialized();
                (xlib.XNextEvent)(self.display, e);

                match (*e).get_type() {
                    xlib::CreateNotify => Self::on_create_notify((*e).create_window),
                    xlib::ConfigureRequest => Self::on_configure_request((*e).configure_request),
                    _ => println!("Unknown event")
                }
            }

        }
    }

    unsafe fn on_configure_request(event: xlib::XConfigureRequestEvent) {
        println!("Time to configure!");
    }

    unsafe fn on_create_notify(event: xlib::XCreateWindowEvent) {
        println!("on_create_notify: called");
    }

}
