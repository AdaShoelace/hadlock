#![allow(non_upper_case_globals)]

use x11::xlib;
use libc::{c_char, c_uchar, c_int, c_uint, c_long, c_ulong};
use std::ffi::{CStr, CString};
use std::mem;

#[macro_use]
use bitflags;
use util::*;

use super::xlibwrappererror::{ Result as XResult, XlibWrapperError as XlibError};
use super::masks::*;

pub(crate) type Mask = c_long;
pub(crate) type Window = xlib::Window;
pub(crate) type Display = xlib::Display;
pub(crate) type Drawable = xlib::Drawable;



pub(crate) unsafe extern "C" fn error_handler(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    let error = CString::from_raw((*e).error_code as *mut c_char);
    println!("Error: {}", error.to_str().unwrap());
    return 0;
}

pub(crate) unsafe extern "C" fn on_wm_detected(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    if (*e).error_code == xlib::BadAccess {
        panic!("Other wm registered!")
    }
    return 0;
}

pub struct XlibWrapper {
    display: *mut Display,
    root: xlib::Window,
}

impl XlibWrapper {
    pub fn new() -> Self {
        let (disp, root) = unsafe {
            let disp = xlib::XOpenDisplay(std::ptr::null_mut());

            if disp == std::ptr::null_mut() {
                panic!("Failed to load display in Xxlib::rapper");
            }

            let root = (xlib::XDefaultRootWindow)(disp);
            xlib::XSetErrorHandler(Some(on_wm_detected));
            xlib::XSelectInput(
                disp,
                root,
                xlib::SubstructureNotifyMask | xlib::SubstructureRedirectMask
            );
            xlib::XSync(disp, 0);
            xlib::XSetErrorHandler(Some(error_handler));
            (disp, root)
        };

        Self {
            display: disp,
            root: root,
        }
    }

    pub fn add_to_save_set(&self, w: Window) {
        unsafe {
            xlib::XAddToSaveSet(self.display, w);
        }
    }

    pub fn create_simple_window(&self, w: Window, pos: Position, size: Size, border_width: u32, border_color: Color, bg_color: Color) -> Window {
        unsafe {
            xlib::XCreateSimpleWindow(
                self.display,
                w,
                pos.x,
                pos.y,
                size.width,
                size.height,
                border_width,
                border_color as u64,
                bg_color as u64
            )
        }
    }

    pub fn configure_window(&mut self,
                            window: Window,
                            value_mask: Mask,
                            changes: WindowChanges) {
        unsafe {
            let mut raw_changes = xlib::XWindowChanges {
                x: changes.x,
                y: changes.y,
                width: changes.width,
                height: changes.height,
                border_width: changes.border_width,
                sibling: changes.sibling,
                stack_mode: changes.stack_mode
            };

            xlib::XConfigureWindow(self.display, window, value_mask as u32, &mut raw_changes);
        }
    }

    pub fn get_geometry(&self, w: Window) -> super::xlibwrappererror::Result<Geometry> {

        /* 
         * Because of xlib being designed as most c libraries it takes pointers and mutates them
         * instead of returning a new value.
         * Here we instead return a struct representing the changes in pointers sent to
         * xlib::XGetGeometry
         */

        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            let status = xlib::XGetWindowAttributes(self.display, w, &mut attr);

            match status as u8 {
                xlib::BadValue => return Err(XlibError::BadValueError),
                xlib::BadWindow => return Err(XlibError::BadWindowError),
                _=> {}
            } 

            Ok(Geometry {
                x: attr.x,
                y: attr.y,
                width: attr.width as u32,
                height: attr.height as u32,
            })
        }
    }

    pub fn get_root(&self) -> Window {
        self.root
    }

    pub fn get_window_attributes(&self, w: Window) -> WindowAttributes {
        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            xlib::XGetWindowAttributes(self.display, w, &mut attr);
            WindowAttributes::from(attr)
        }
    }

    pub fn grab_button(&self,
                       button: u32,
                       modifiers: u32,
                       grab_window: Window,
                       owner_events: bool,
                       event_mask: u32,
                       pointer_mode: i32,
                       keyboard_mode: i32,
                       confine_to: Window,
                       cursor: u64
    ) {
        unsafe {
            xlib::XGrabButton(
                self.display,
                button,
                modifiers,
                grab_window,
                to_c_bool(owner_events),
                event_mask,
                pointer_mode,
                keyboard_mode,
                confine_to,
                cursor
            );
        }
    }

    pub fn map_window(&mut self, window: Window) {
        unsafe {
            xlib::XMapWindow(self.display, window);
        }
    }

    pub fn move_window(&self, w: Window, dest_x: i32, dest_y: i32) {
        unsafe {
            xlib::XMoveWindow(self.display, w, dest_x, dest_y);
        }
    }

    pub fn next_event(&self) -> Event {
        let ret_event = unsafe {
            let mut event: xlib::XEvent = mem::uninitialized();
            xlib::XNextEvent(self.display, &mut event);
            println!("Event: {:?}", event);
            println!("Event type: {:?}", event.get_type());
            match event.get_type() {
                xlib::ConfigureRequest => {
                    let event = xlib::XConfigureRequestEvent::from(event);
                    let window_changes = WindowChanges {
                        x: event.x,
                        y: event.y,
                        width: event.width,
                        height: event.height,
                        border_width: event.border_width,
                        sibling: event.above,
                        stack_mode: event.detail
                    };
                    Event::ConfigurationRequest(
                        event.window,
                        window_changes,
                        event.value_mask
                    )
                },
                xlib::MapRequest => {
                    let event = xlib::XMapRequestEvent::from(event);
                    Event::WindowCreated(event.window)
                },
                xlib::ButtonPress => {
                    println!("Button press");
                    let event = xlib::XButtonEvent::from(event);
                   Event::ButtonPressed(event.window, event.subwindow, event.button, event.x_root as u32, event.y_root as u32) 
                },
                xlib::MotionNotify => {
                    let event = xlib::XMotionEvent::from(event);
                    Event::MotionNotify(
                        event.window,
                        event.x_root,
                        event.y_root,
                        event.state
                    )
                }
                _ => Event::UnknownEvent
            }
        };
        // println!("ret_event: {:?}", ret_event);
        ret_event
    }
    
    pub fn raise_window(&self, w: Window) -> XResult<()> {
        unsafe {
            match xlib::XRaiseWindow(self.display, w) as u8 {
                xlib::BadValue => Err(XlibError::BadValueError),
                xlib::BadWindow => Err(XlibError::BadWindowError),
                _ => Err(XlibError::UnknownError) 
            }
        }
    }

    pub fn reparent_window(&self, w: Window, parent: Window) {
        unsafe {
            xlib::XReparentWindow(
                self.display,
                w,
                parent,
                0,0
            );
        }
    }

    pub fn select_input(&mut self, window: xlib::Window, masks: Mask) {
        unsafe {
            xlib::XSelectInput(
                self.display,
                window,
                masks
            );
        }
    }

    pub fn sync(&mut self, discard: bool) {
        unsafe {
            xlib::XSync(self.display, discard as i32);
        }
    }

    pub fn generic_error_handler(&mut self) {
        unsafe {
            xlib::XSetErrorHandler(Some(error_handler));
        }
    }
    pub fn other_wm_error_handler(&mut self) {
        unsafe {
            xlib::XSetErrorHandler(Some(on_wm_detected));
        }
    }

}

#[derive(Debug)]
pub enum Event {
    ConfigurationNotification(Window),
    ConfigurationRequest(Window, WindowChanges, u64),
    WindowCreated(Window),
    ButtonPressed(Window, Window, u32, u32, u32),
    MotionNotify(Window, i32, i32, u32),
    ButtonReleased,
    // CreateNotify(CreateWindowEvent),
    UnknownEvent
}

#[derive(Clone, Copy, Debug)]
pub struct WindowChanges {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub sibling: Window,
    pub stack_mode: c_int,
}


pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct WindowAttributes<'a> {
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub depth: c_int,
    pub visual: &'a mut xlib::Visual,
    pub root: Window,
    pub class: c_int,
    pub bit_gravity: c_int,
    pub win_gravity: c_int,
    pub backing_store: c_int,
    pub backing_planes: c_ulong,
    pub backing_pixel: c_ulong,
    pub save_under: bool,
    pub colormap: xlib::Colormap,
    pub map_installed: bool,
    pub map_state: c_int,
    pub all_event_masks: c_long,
    pub your_event_mask: c_long,
    pub do_not_propagate_mask: c_long,
    pub override_redirect: bool,
    pub screen: &'a mut xlib::Screen,
}

impl <'a> From<xlib::XWindowAttributes> for WindowAttributes<'a> {
    fn from(attr: xlib::XWindowAttributes) -> Self {
        unsafe {
            Self {
                x: attr.x,
                y: attr.y,
                width: attr.width,
                height: attr.height,
                border_width: attr.border_width,
                depth: attr.depth,
                visual: &mut *attr.visual,
                root: attr.root,
                class: attr.class,
                bit_gravity: attr.bit_gravity,
                win_gravity: attr.win_gravity,
                backing_store: attr.backing_store,
                backing_planes: attr.backing_planes,
                backing_pixel: attr.backing_pixel,
                save_under: from_c_bool(attr.save_under),
                colormap: attr.colormap,
                map_installed: from_c_bool(attr.map_installed),
                map_state: attr.map_state,
                all_event_masks: attr.all_event_masks,
                your_event_mask: attr.your_event_mask,
                do_not_propagate_mask: attr.do_not_propagate_mask,
                override_redirect: from_c_bool(attr.override_redirect),
                screen: &mut *attr.screen,
            }
        }
    }
}

impl <'a> Into<xlib::XWindowAttributes> for WindowAttributes<'a> {
    fn into(self) -> xlib::XWindowAttributes {
        unsafe {
            let mut ret: xlib::XWindowAttributes = mem::uninitialized();
            ret.x = self.x;
            ret.y = self.y;
            ret.width = self.width;
            ret.height =  self.height;
            ret.border_width = self.border_width;
            ret.depth = self.depth;
            ret.visual = &mut *self.visual;
            ret.root = self.root;
            ret.class = self.class;
            ret.bit_gravity = self.bit_gravity;
            ret.win_gravity = self.win_gravity;
            ret.backing_store = self.backing_store;
            ret.backing_planes = self.backing_planes;
            ret.backing_pixel = self.backing_pixel;
            ret.save_under = to_c_bool(self.save_under);
            ret.colormap = self.colormap;
            ret.map_installed = to_c_bool(self.map_installed);
            ret.map_state = self.map_state;
            ret.all_event_masks = self.all_event_masks;
            ret.your_event_mask = self.your_event_mask;
            ret.do_not_propagate_mask = self.do_not_propagate_mask;
            ret.override_redirect = to_c_bool(self.override_redirect);
            ret.screen = &mut *self.screen;
            ret
        }
    }
}

pub mod util {
    pub fn from_c_bool(b: i32) -> bool {
        if b > 0 || b < 0{
            true
        } else {
            false
        }
    }

    pub fn to_c_bool(b: bool) -> i32 {
        if b || !b {
            1
        } else {
            0
        }
    }

    pub struct Position { pub x: i32, pub y: i32 }
    pub struct Size { pub width: u32, pub height: u32 }
    pub enum Color {
        RED = 0xff0000,
        BLUE = 0x0000ff,
    }
}

pub struct EventWrapper {
    pub event: xlib::XEvent
}
