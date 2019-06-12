
use std::ptr;
use x11_dl::xlib;
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CStr, CString};
use std::mem;

#[macro_use]
use bitflags;

type Mask = c_long;
type Window = xlib::Window;

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

pub struct XlibWrapper<'a> {
    lib: xlib::Xlib,
    display: &'a mut xlib::Display,
    root: xlib::Window,
    event: xlib::XEvent
}

impl<'a> XlibWrapper<'a> {

    pub fn new() -> Self {
        let lib: xlib::Xlib = xlib::Xlib::open().expect("Failed to load xlib in XlibWrapper");

        let (disp, root) = unsafe {
            let disp = (lib.XOpenDisplay)(std::ptr::null_mut());

            if disp == std::ptr::null_mut() {
                panic!("Failed to load display in XlibWrapper");
            }

            let root = (lib.XDefaultRootWindow)(disp);
            (&mut(*disp), root)
        };

        Self {
            lib: lib,
            display: disp,
            root: root,
            event: xlib::XEvent { pad: [0; 24] }
        }
    }

    pub fn next_event(&mut self) -> Event {
        let configure_event = |event: xlib::XConfigureRequestEvent| {
            let ret_event = ConfigureRequestEvent {
                type_: event.type_,
                serial: event.serial,
                send_event: if event.send_event > 0 { true } else { false },
                // display: *mut Display
                parent: event.parent,
                window: event.window,
                x: event.x,
                y: event.y,
                width: event.width,
                height: event.height,
                border_width: event.border_width,
                above: event.above,
                detail: event.detail,
                value_mask: event.value_mask,
            };
            Event::ConfigureRequest(ret_event)
        };


        unsafe {
            (self.lib.XNextEvent)(self.display, &mut self.event);
            match self.event.get_type() {
                xlib::ConfigureRequest => configure_event(self.event.configure_request),
                // xlib::MapRequest => 
                _ => unimplemented!()
            }
        }
    }

    pub fn select_input(&mut self, window: xlib::Window, masks: Mask) {
        unsafe {
            (self.lib.XSelectInput)(
                self.display,
                window,
                masks
            );
        }
    }

    pub fn sync(&mut self, discard: bool) {
        unsafe {
            (self.lib.XSync)(self.display, discard as i32);
        }
    }

    pub fn generic_error_handler(&mut self) {
        unsafe {
            (self.lib.XSetErrorHandler)(Some(error_handler));
        }
    }
    pub fn other_wm_error_handler(&mut self) {
        unsafe {
            (self.lib.XSetErrorHandler)(Some(on_wm_detected));
        }
    }

}

pub enum Event {
    ConfigureRequest(ConfigureRequestEvent),
}

pub struct ConfigureRequestEvent {
    type_: c_int,
    serial: c_ulong,
    send_event: bool,
    // display: *mut Display
    parent: Window,
    window: Window,
    x: c_int,
    y: c_int,
    width: c_int,
    height: c_int,
    border_width: c_int,
    above: Window,
    detail: c_int,
    value_mask: c_ulong,
}


bitflags! {
    pub struct Masks: c_long {
        const SubstructureRedirectMask = xlib::SubstructureRedirectMask;
        const SubstructureNotifyMask = xlib::SubstructureNotifyMask;
    }
}


