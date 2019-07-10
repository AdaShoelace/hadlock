#![allow(non_upper_case_globals, dead_code)]

use x11_dl::xlib;
use std::os::raw::*;
use std::ffi::CString;
use std::mem;

use super::masks::*;
use super::util::*;
use super::xatom::*;
use super::xlibmodels::*;
use super::event::*;

pub(crate) unsafe extern "C" fn error_handler(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    let err = *e;
    if err.error_code == xlib::BadWindow {
        return 0;
    }
    1
}

pub(crate) unsafe extern "C" fn on_wm_detected(_: *mut xlib::Display, e: *mut xlib::XErrorEvent) -> c_int {
    if (*e).error_code == xlib::BadAccess {
        eprintln!("Other wm registered!");
        return 1;
    }
    0
}

pub struct XlibWrapper {
    lib: xlib::Xlib,
    pub xatom: XAtom,
    display: *mut Display,
    root: Window,
    check: Window
}

impl XlibWrapper {
    pub fn new() -> Self {
        let (disp, root, lib, xatom, check) = unsafe {
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
            let check = (lib.XCreateSimpleWindow)(disp, root, 0, 0, 1, 1, 0, 0, 0);
            (disp, root, lib, xatom, check)
        };

        let ret = Self {
            lib: lib,
            xatom: xatom,
            display: disp,
            root: root,
            check: check
        };

        ret
    }


    pub fn add_to_save_set(&self, w: Window) {
        unsafe {
            (self.lib.XAddToSaveSet)(self.display, w);
        }
    }

    pub fn take_focus(&self, w: Window) {
        unsafe {
            (self.lib.XSetInputFocus)(
                self.display,
                w,
                xlib::RevertToPointerRoot,
                xlib::CurrentTime
            );
            let list = vec![w];
            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                self.xatom.NetActiveWindow,
                xlib::XA_WINDOW,
                32,
                xlib::PropModeReplace,
                list.as_ptr() as *const u8,
                1
            );
            std::mem::forget(list);
        }
        self.send_xevent_atom(w, self.xatom.WMTakeFocus);
    }

    pub fn set_active(&self, w: Window) {
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

    pub fn set_border_color(&self, w: Window, color: Color) {
        if w == self.root {
            return;
        }

        let color = color.value();

        unsafe {
            (self.lib.XSetWindowBorder)(
                self.display,
                w,
                color
            );
            (self.lib.XSync)(
                self.display,
                0
            );
        }
    }

    pub fn configure_window(&self,
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

    pub fn grab_keyboard(&self, w: Window) {
        unsafe {
            (self.lib.XGrabKeyboard)(
                self.display,
                w,
                to_c_bool(false),
                GrabModeAsync,
                GrabModeAsync,
                xlib::CurrentTime
            );
        }
    }

    pub fn add_to_root_net_client_list(&self, w: Window) {
        unsafe {
            let list = vec![w];

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
                border_color.value(),
                bg_color.value()
            )
        }
    }

    pub fn destroy_window(&self, w: Window) {
        unsafe {
            (self.lib.XDestroyWindow)(self.display, w);
        }
    }

    pub fn focus_window(&self, w: Window) {
        unsafe {
            let list = vec![w];
            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                self.xatom.NetActiveWindow,
                xlib::XA_WINDOW,
                32,
                xlib::PropModeReplace,
                list.as_ptr() as *const u8,
                1
            );
            mem::forget(list);
        }
    }


    pub fn intern_atom(&self, s: &str) -> u64 {
        unsafe {
            match CString::new(s) {
                Ok(b) =>  (self.lib.XInternAtom)(self.display, b.as_ptr() as *const i8, 0) as u64,
                _ => panic!("Invalid atom {}", s)
            }
        }
    }

    pub fn get_geometry(&self, w: Window) -> Geometry {

        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            let status = (self.lib.XGetWindowAttributes)(self.display, w, &mut attr);

            Geometry {
                x: attr.x,
                y: attr.y,
                width: attr.width as u32,
                height: attr.height as u32,
            }
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

    pub fn grab_server(&self) {
        unsafe {
            (self.lib.XGrabServer)(
                self.display
            );
        }
    }

    pub fn ungrab_server(&self) {
        unsafe {
            (self.lib.XUngrabServer)(
                self.display
            );
        }
    }

    pub fn ungrab_all_buttons(&self, w: Window) {
        unsafe {
            (self.lib.XUngrabButton)(
                self.display,
                xlib::AnyButton as u32,
                xlib::AnyModifier,
                w
            );
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

    pub fn str_to_keycode(&self, key: &str) -> Option<KeyCode> {
        match keysym_lookup::into_keysym(key) {
            Some(key) => Some(self.key_sym_to_keycode(key.into())),
            None => None
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

    pub fn get_window_type_atom(&self, w: Window) -> Option<xlib::Atom> {
        self.get_atom_prop_value(w, self.xatom.NetWMWindowType)
    }

    pub fn get_atom_prop_value(
        &self,
        window: xlib::Window,
        prop: xlib::Atom,
        ) -> Option<xlib::Atom> {
        // Shamelessly stolen from lex148/leftWM
        let mut format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut type_return: xlib::Atom = 0;
        let mut prop_return: *mut c_uchar = unsafe { std::mem::uninitialized() };
        unsafe {
            let status = (self.lib.XGetWindowProperty)(
                self.display,
                window,
                prop,
                0,
                1024,
                xlib::False,
                xlib::XA_ATOM,
                &mut type_return,
                &mut format_return,
                &mut nitems_return,
                &mut nitems_return,
                &mut prop_return,
                );
            if status == i32::from(xlib::Success) && !prop_return.is_null() {
                #[allow(clippy::cast_lossless, clippy::cast_ptr_alignment)]
                let atom = *(prop_return as *const xlib::Atom);
                return Some(atom);
            }
            None
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

    pub fn map_window(&self, window: Window) {
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
        unsafe {
            let mut event: xlib::XEvent = mem::uninitialized();
            (self.lib.XNextEvent)(self.display, &mut event);
            //println!("Event: {:?}", event);
            //println!("Event type: {:?}", event.get_type());
            //println!("Pending events: {}", (self.lib.XPending)(self.display));

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
                    let payload = EventPayload::ConfigurationRequest(
                        event.window,
                        window_changes,
                        event.value_mask
                    );

                    Event::new(EventType::ConfigurationRequest, Some(payload))
                },
                xlib::MapRequest => {
                    //println!("MapRequest");
                    let event = xlib::XMapRequestEvent::from(event);
                    let payload = EventPayload::MapRequest(event.window);
                    Event::new(EventType::MapRequest, Some(payload))
                },
                xlib::ButtonPress => {
                    //println!("Button press");
                    let event = xlib::XButtonEvent::from(event);
                    let payload = EventPayload::ButtonPress(
                        event.window,
                        event.subwindow,
                        event.button,
                        event.x_root as u32,
                        event.y_root as u32,
                        event.state as u32
                    );
                    Event::new(EventType::ButtonPress, Some(payload))
                },
                xlib::KeyPress => {
                    //println!("Keypress\tEvent: {:?}", event);
                    let event = xlib::XKeyEvent::from(event);
                    let payload = EventPayload::KeyPress(event.window, event.state, event.keycode);
                    Event::new(EventType::KeyPress, Some(payload))
                },
                xlib::MotionNotify => {
                    let event = xlib::XMotionEvent::from(event);
                    let payload = EventPayload::MotionNotify(
                        event.window,
                        event.x_root,
                        event.y_root,
                        event.state
                    );
                    Event::new(EventType::MotionNotify, Some(payload))
                },
                xlib::EnterNotify => {
                    //println!("EnterNotify");
                    let event = xlib::XCrossingEvent::from(event);
                    let payload = EventPayload::EnterNotify(event.window);
                    Event::new(EventType::EnterNotify, Some(payload))
                },
                xlib::LeaveNotify => {
                    //println!("LeaveNotify");
                    let event = xlib::XCrossingEvent::from(event);
                    let payload = EventPayload::LeaveNotify(event.window);
                    Event::new(EventType::LeaveNotify, Some(payload))
                },
                xlib::Expose => {
                    let event = xlib::XExposeEvent::from(event);
                    let payload = EventPayload::Expose(event.window);
                    Event::new(EventType::Expose, Some(payload))
                },
                xlib::DestroyNotify => {
                    let event = xlib::XDestroyWindowEvent::from(event);
                    let payload = EventPayload::DestroyWindow(event.window);
                    Event::new(EventType::DestroyWindow, Some(payload))
                }
                _ => Event::new(EventType::UnknownEvent, None)
            }
        }
    }

    pub fn raise_window(&self, w: Window) {
        unsafe {
            (self.lib.XRaiseWindow)(self.display, w);
        }
    }


    pub fn resize_window(&self, w: Window, width: u32, height: u32) {
        unsafe {
            (self.lib.XResizeWindow)(
                self.display,
                w,
                width,
                height
            );
        }
    }

    pub fn remove_from_save_set(&self, w: Window) {
        unsafe {
            (self.lib.XRemoveFromSaveSet)(self.display, w);
        }
    }

    pub fn select_input(&self, window: xlib::Window, masks: Mask) {
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


    fn set_desktop_prop(&self, data: &[u32], atom: c_ulong) {
        let xdata = data.to_owned();
        unsafe {
            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                atom,
                xlib::XA_CARDINAL,
                32,
                xlib::PropModeReplace,
                xdata.as_ptr() as *const u8,
                data.len() as i32,
                );
            std::mem::forget(xdata);
        }
    }

    pub fn set_atom_number_of_desktops(&self, num: u32) {
        self.set_desktop_prop(&[num], self.xatom.NetNumberOfDesktops);
    }

    pub fn sync(&self, discard: bool) {
        unsafe {
            (self.lib.XSync)(self.display, discard as i32);
        }
    }

    pub fn get_top_level_windows(&self) -> Vec<Window> {
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

            let windows = std::slice::from_raw_parts(top_level_windows, num_top_level_windows as usize);
            Vec::from(windows)
        }
    }

    pub fn top_level_window_count(&self) -> u32 {
        self.get_top_level_windows().len() as u32
    }

    pub fn unmap_window(&self, w: Window) {
        unsafe {
            (self.lib.XUnmapWindow)(self.display, w);
        }
    }

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


