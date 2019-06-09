
use std::ptr;
use x11::xlib;
use x11::xlib::{Display, Window};
use x11::xlib::{XOpenDisplay, XDefaultScreenOfDisplay, XRootWindowOfScreen};
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::process::exit;

unsafe extern "C" fn error_handler(_: *mut xlib::Display, _: *mut xlib::XErrorEvent) -> c_int {
    return 0;
}

unsafe extern "C" fn on_wm_detected(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    return 0;
}

pub struct WindowManager {
    display: *mut Display,
    root : Window,
    wm_detected: bool
}

impl WindowManager {
    pub fn new () -> Self {
        unsafe {

            let display = XOpenDisplay(ptr::null_mut());
            let screen = XDefaultScreenOfDisplay(display);
            let root = XRootWindowOfScreen(screen);

            Self {
                display: display,
                root: root,
                wm_detected: false
            }
        }
    }

    pub fn run(&mut self) {
        unsafe {
            println!("Display name: {}\nDisplay height: {}", xlib::XDisplayName(ptr::null()) as i8, xlib::XDisplayHeight(self.display, 0) as u8);
            xlib::XSetErrorHandler(Some(on_wm_detected));
            xlib::XSelectInput(
                self.display,
                self.root,
                xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask
            );
            xlib::XSync(self.display, 0);
            if self.wm_detected {
                return;
            }
            xlib::XSetErrorHandler(Some(error_handler));
        }
    }


}
