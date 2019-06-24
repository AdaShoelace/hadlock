#![allow(non_upper_case_globals)]

use x11_dl::xlib;
use std::os::raw::*;
use std::ffi::{CStr, CString};
use std::mem;

#[macro_use]
use bitflags;

use super::xlibwrappererror::{ Result as XResult, XlibWrapperError as XlibError};
use super::masks::*;
use super::util::*;
use super::util::keysym_lookup::*;
use super::xatom::*;

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
    lib: xlib::Xlib,
    xatom: XAtom,
    display: *mut Display,
    root: xlib::Window,
}

impl XlibWrapper {
    pub fn new() -> Self {
        let (disp, root, lib, xatom) = unsafe {
            let lib = xlib::Xlib::open().unwrap();
            let disp = (lib.XOpenDisplay)(std::ptr::null_mut());

            if disp == std::ptr::null_mut() {
                panic!("Failed to load display in Xxlib::rapper");
            }

            let root = (lib.XDefaultRootWindow)(disp);
            (lib.XSetErrorHandler)(Some(on_wm_detected));
            (lib.XSelectInput)(
                disp,
                root,
                xlib::SubstructureNotifyMask | xlib::SubstructureRedirectMask
            );
            (lib.XSync)(disp, 0);
            (lib.XSetErrorHandler)(Some(error_handler));
            let xatom = XAtom::new(&lib, disp);
            (disp, root, lib, xatom)
        };

        Self {
            lib: lib,
            xatom: xatom,
            display: disp,
            root: root,
        }
    }

    pub fn add_to_save_set(&self, w: Window) {
        unsafe {
            (self.lib.XAddToSaveSet)(self.display, w);
        }
    }

    fn expects_xevent_atom(&self, window: Window, atom: xlib::Atom) -> bool {
        unsafe {
            let mut array: *mut xlib::Atom = mem::uninitialized();
            let mut length: c_int = mem::uninitialized();
            let status: xlib::Status =
                (self.lib.XGetWMProtocols)(self.display, window, &mut array, &mut length);
            let protocols: &[xlib::Atom] = std::slice::from_raw_parts(array, length as usize);
            status > 0 && protocols.contains(&atom)
        }
    }

    fn send_xevent_atom(&self, window: Window, atom: xlib::Atom) -> bool {
        if self.expects_xevent_atom(window, atom) {
            let mut msg: xlib::XClientMessageEvent = unsafe { std::mem::uninitialized() };
            msg.type_ = xlib::ClientMessage;
            msg.window = window;
            msg.message_type = self.xatom.WMProtocols;
            msg.format = 32;
            msg.data.set_long(0, atom as i64);
            msg.data.set_long(1, xlib::CurrentTime as i64);
            let mut ev: xlib::XEvent = msg.into();
            unsafe { (self.lib.XSendEvent)(self.display, window, 0, xlib::NoEventMask, &mut ev) };
            return true;
        }
        false
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

            (self.lib.XConfigureWindow)(self.display, window, value_mask as u32, &mut raw_changes);
        }
    }

    pub fn change_frame_property(&self, frame: Window) {
        unsafe {
            let list = vec![frame];

            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                self.xatom.NetClientList,
                xlib::XA_WINDOW,
                32,
                xlib::PropModeAppend,
                list.as_ptr() as *const u8,
                1
            );
            mem::forget(list);
        }

    }

    pub fn create_simple_window(&self, w: Window, pos: Position, size: Size, border_width: u32, border_color: Color, bg_color: Color) -> Window {
        unsafe {
            (self.lib.XCreateSimpleWindow)(
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

    pub fn destroy_window(&self, w: Window) -> XResult<()>{
        unsafe {
            let status = (self.lib.XDestroyWindow)(self.display, w);
            match status as u8 {
                xlib::BadWindow => Err(XlibError::BadWindow),
                _ => Ok(())
            }
        }
    }

    pub fn intern_atom(&self, s: &str) -> XResult<u64> {
        unsafe {
            match CString::new(s) {
                Ok(b) => {
                    let ret = (self.lib.XInternAtom)(self.display, b.as_ptr() as *const i8, 0) as u64;
                    match ret as u8 {
                        xlib::BadAlloc => Err(XlibError::BadAlloc),
                        xlib::BadAtom => Err(XlibError::BadAtom),
                        _ => Ok(ret)
                    }
                },
                _ => panic!("Invalid atom {}", s)
            }
        }
    }

    pub fn get_geometry(&self, w: Window) -> XResult<Geometry> {

        /*
         * Because of xlib being designed as most c libraries it takes pointers and mutates them
         * instead of returning a new value.
         * Here we instead return a struct representing the changes in pointers sent to
         * xlib::XGetGeometry
         */

        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            let status = (self.lib.XGetWindowAttributes)(self.display, w, &mut attr);

            match status as u8 {
                xlib::BadValue => return Err(XlibError::BadValue),
                xlib::BadWindow => return Err(XlibError::BadWindow),
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

    pub fn get_wm_protocols(&self, w: Window) -> Vec<u64> {
        unsafe {
            let mut protocols: *mut u64 = std::ptr::null_mut();
            let mut num = 0;
            (self.lib.XGetWMProtocols)(self.display, w, &mut protocols, &mut num);
            let slice = std::slice::from_raw_parts(protocols, num as usize);

            slice.iter()
                .map(|&x| x as u64)
                .collect::<Vec<u64>>()
        }
    }

    pub fn get_root(&self) -> Window {
        self.root
    }

    pub fn get_window_attributes(&self, w: Window) -> WindowAttributes {
        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            (self.lib.XGetWindowAttributes)(self.display, w, &mut attr);
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
            (self.lib.XGrabButton)(
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

    pub fn key_sym_to_keycode(&self, keysym: u64) -> KeyCode {
        unsafe {
            (self.lib.XKeysymToKeycode)(self.display, keysym)
        }
    }

    pub fn get_keycode_from_string(&self, key: &str) -> u64 {
        unsafe {
            match CString::new(key.as_bytes()) {
                Ok(b) => (self.lib.XStringToKeysym)(b.as_ptr()) as u64,
                _ => panic!("Invalid key string!"),
            }
        }
    }

    pub fn grab_key(&self,
                    key_code: u32,
                    modifiers: u32,
                    grab_window: Window,
                    owner_event: bool,
                    pointer_mode: i32,
                    keyboard_mode: i32) {
        unsafe {
            // add error handling.. Like really come up with a strategy!
            (self.lib.XGrabKey)(
                self.display,
                key_code as i32,
                modifiers,
                grab_window,
                to_c_bool(owner_event),
                pointer_mode,
                keyboard_mode
            );
        }
    }

    pub fn kill_client(&self, w: Window) {
        if !self.send_xevent_atom(w, self.xatom.WMDelete) {
            unsafe {
                (self.lib.XGrabServer)(self.display);
                (self.lib.XSetCloseDownMode)(self.display, xlib::DestroyAll);
                (self.lib.XKillClient)(self.display, w);
                (self.lib.XSync)(self.display, xlib::False);
                (self.lib.XUngrabServer)(self.display);
            }
        }
    }

    pub fn map_window(&mut self, window: Window) {
        unsafe {
            (self.lib.XMapWindow)(self.display, window);
        }
    }

    pub fn move_window(&self, w: Window, dest_x: i32, dest_y: i32) {
        unsafe {
            (self.lib.XMoveWindow)(self.display, w, dest_x, dest_y);
        }
    }

    pub fn next_event(&self) -> Event {
        let ret_event = unsafe {
            let mut event: xlib::XEvent = mem::uninitialized();
            (self.lib.XNextEvent)(self.display, &mut event);
            //println!("Event: {:?}", event);
            //println!("Event type: {:?}", event.get_type());
            unsafe {
                println!("Pending events: {}", (self.lib.XPending)(self.display))
            };
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
                    Event::ButtonPressed(event.window, event.subwindow, event.button, event.x_root as u32, event.y_root as u32, event.state as u32)
                },
                xlib::KeyPress => {
                    println!("Keypress\tEvent: {:?}", event);
                    let event = xlib::XKeyEvent::from(event);
                    Event::KeyPress(event.window, event.state, event.keycode)
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
            match (self.lib.XRaiseWindow)(self.display, w) as u8 {
                xlib::BadValue => Err(XlibError::BadValue),
                xlib::BadWindow => Err(XlibError::BadWindow),
                _ => Err(XlibError::Unknown)
            }
        }
    }

    pub fn remove_from_save_set(&self, w: Window) {
        unsafe {
            (self.lib.XRemoveFromSaveSet)(self.display, w);
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

    pub fn set_border_width(&self, w: Window, border_width: u32) {
        if w == self.root {
            return;
        }
        unsafe {
            (self.lib.XSetWindowBorderWidth)(self.display, w, border_width);
        }
    }

    pub fn set_input_focus(&self, w: Window, revert_to: i32, time: Time) -> XResult<()> {
        unsafe {
            match (self.lib.XSetInputFocus)(self.display, w, revert_to, time) as u8 {
                xlib::BadValue => Err(XlibError::BadValue),
                xlib::BadMatch => Err(XlibError::BadMatch),
                xlib::BadWindow => Err(XlibError::BadWindow),
                _ => Ok(())
            }
        }
    }

    pub fn sync(&mut self, discard: bool) {
        unsafe {
            (self.lib.XSync)(self.display, discard as i32);
        }
    }

    pub fn top_level_window_count(&self) -> u32 {
        unsafe {
            let mut returned_root: Window = mem::uninitialized();
            let mut returned_parent: Window = mem::uninitialized();
            let mut top_level_windows: *mut Window = mem::uninitialized();
            let mut num_top_level_windows: u32 = mem::uninitialized();
            (self.lib.XQueryTree)(
                self.display,
                self.root,
                &mut returned_root,
                &mut returned_parent,
                &mut top_level_windows,
                &mut num_top_level_windows
            );
            num_top_level_windows
        }
    }

    pub fn unmap_window(&self, w: Window) {
        unsafe {
            (self.lib.XUnmapWindow)(self.display, w);
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

#[derive(Debug)]
pub enum Event {
    ConfigurationNotification(Window),
    ConfigurationRequest(Window, WindowChanges, u64),
    WindowCreated(Window),
    ButtonPressed(Window, Window, u32, u32, u32, u32),
    KeyPress(Window, u32, u32),
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

