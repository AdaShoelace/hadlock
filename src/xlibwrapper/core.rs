#![allow(unused_variables, deprecated, dead_code)]
use std::ffi::CString;
use std::mem::{self, MaybeUninit};
use std::os::raw::*;
use std::ptr::null;
pub use x11_dl::xlib;

use super::{masks::*, util::*, xatom::*, xlibmodels::*, DisplayServer};

use super::cursor::Cursor;
use super::util::Position;
use crate::config::*;

use crate::models::{dockarea::DockArea, screen::Screen, window_type::WindowType};

pub(crate) unsafe extern "C" fn error_handler(
    _: *mut xlib::Display,
    e: *mut xlib::XErrorEvent,
) -> i32 {
    let err = *e;
    if err.error_code == xlib::BadWindow {
        return 0;
    }
    1
}

pub(crate) unsafe extern "C" fn on_wm_detected(
    _: *mut xlib::Display,
    e: *mut xlib::XErrorEvent,
) -> i32 {
    if (*e).error_code == xlib::BadAccess {
        error!("Other wm registered!");
        return 1;
    }
    0
}

pub struct XlibWrapper {
    lib: xlib::Xlib,
    pub xatom: XAtom,
    display: *mut Display,
    root: Window,
    cursors: Cursor,
}

impl XlibWrapper {
    pub fn new() -> Self {
        let (disp, root, lib, xatom, cursors) = unsafe {
            let lib = xlib::Xlib::open().expect("xlibwrapper::core: new");
            let disp = (lib.XOpenDisplay)(std::ptr::null_mut());

            if disp.is_null() {
                panic!("Failed to load display in xlib::XlibWrapper");
            }

            let root = (lib.XDefaultRootWindow)(disp);
            (lib.XSetErrorHandler)(Some(on_wm_detected));
            (lib.XSync)(disp, 0);
            (lib.XSetErrorHandler)(Some(error_handler));
            let xatom = XAtom::new(&lib, disp);

            let screen_id = (lib.XDefaultScreen)(disp);
            let display_width = (lib.XDisplayWidth)(disp, screen_id);
            let display_height = (lib.XDisplayHeight)(disp, screen_id);
            let cursors = Cursor::new(&lib, disp);

            (disp, root, lib, xatom, cursors)
        };

        let mut ret = Self {
            lib,
            xatom,
            display: disp,
            root,
            cursors,
        };
        ret.init();
        ret.init_desktops_hints();
        ret
    }

    fn init(&mut self) {
        let root_event_mask: i64 = xlib::SubstructureRedirectMask
            | xlib::SubstructureNotifyMask
            | xlib::ButtonPressMask
            | xlib::KeyPressMask
            | xlib::PointerMotionMask
            | xlib::EnterWindowMask
            | xlib::LeaveWindowMask
            | xlib::StructureNotifyMask
            | xlib::PropertyChangeMask;

        let mut attrs: xlib::XSetWindowAttributes = unsafe { mem::uninitialized() };
        attrs.cursor = self.cursors.normal_cursor;
        attrs.event_mask = root_event_mask;

        unsafe {
            (self.lib.XChangeWindowAttributes)(
                self.display,
                self.root,
                xlib::CWEventMask | xlib::CWCursor,
                &mut attrs,
            );
        }
        self.select_input(self.root, root_event_mask);
        unsafe {
            let supported = self.xatom.net_supported();
            let supp_ptr: *const xlib::Atom = supported.as_ptr();
            (self.lib.XInternAtom)(
                self.display,
                self.xatom.get_name(self.xatom.NetActiveWindow).as_ptr() as *const i8,
                to_c_bool(false),
            );
            (self.lib.XInternAtom)(self.display, supp_ptr as *const i8, to_c_bool(false));
            let size = supported.len() as i32;
            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                self.xatom.NetSupported,
                xlib::XA_ATOM,
                32,
                xlib::PropModeReplace,
                supp_ptr as *const u8,
                size,
            );
            mem::forget(supported);
            (self.lib.XUngrabKey)(self.display, xlib::AnyKey, xlib::AnyModifier, self.root);
            (self.lib.XDeleteProperty)(self.display, self.root, self.xatom.NetClientList);
            let key_list = CONFIG
                .key_bindings
                .iter()
                .filter(|binding| match binding.key {
                    Key::Letter(_) => true,
                    _ => false,
                })
                .cloned()
                .map(|binding| match binding.key {
                    Key::Letter(x) => x,
                    _ => "".to_string(),
                })
                .filter(|key| !key.is_empty())
                .collect::<Vec<String>>();

            for mod_key in mod_masks_vec() {
                for key in &key_list {
                    if let Some(key_sym) = keysym_lookup::into_keysym(&key) {
                        self.grab_keys(self.get_root(), key_sym, CONFIG.mod_key | mod_key);
                    }
                }
            }
        }
        self.sync(false);
    }

    fn get_non_xinerama_screens(&self) -> Vec<xlib::Screen> {
        let mut screens = Vec::new();
        let count = unsafe { (self.lib.XScreenCount)(self.display) };
        (0..count).enumerate().for_each(|(i, x)| {
            let screen = unsafe { *(self.lib.XScreenOfDisplay)(self.display, i as i32) };
            screens.push(screen);
        });
        screens
    }

    fn get_roots(&self) -> Vec<Window> {
        self.get_non_xinerama_screens()
            .into_iter()
            .map(|mut screen| unsafe { (self.lib.XRootWindowOfScreen)(&mut screen) })
            .collect()
    }

    fn ewmh_current_desktop(&self, desktop: u32) {
        let data = vec![desktop, xlib::CurrentTime as u32];
        self.set_desktop_prop(&data, self.xatom.NetCurrentDesktop);
        self.sync(false);
    }

    fn support_wm_check(&self) {
        if let Ok(cstring) = CString::new("Hadlock") {
            unsafe {
                (self.lib.XChangeProperty)(
                    self.display,
                    self.root,
                    self.xatom.NetWMName,
                    self.xatom.NetUtf8String,
                    8,
                    xlib::PropModeReplace,
                    cstring.as_ptr() as *const u8,
                    "Hadlock".len() as i32,
                );
                mem::forget(cstring);
            }
        }
        self.set_desktop_prop_u64(self.root, self.xatom.NetSupportingWmCheck, xlib::XA_WINDOW);
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
            mem::forget(xdata);
        }
    }

    fn get_atom(&self, s: &str) -> u64 {
        unsafe {
            match CString::new(s) {
                Ok(b) => {
                    (self.lib.XInternAtom)(self.display, b.as_ptr() as *const c_char, 0) as u64
                }
                _ => panic!("Invalid atom! {}", s),
            }
        }
    }

    fn send_xevent_atom(&self, window: Window, atom: xlib::Atom) -> bool {
        if self.expects_xevent_atom(window, atom) {
            let mut msg: xlib::XClientMessageEvent = unsafe { mem::uninitialized() };
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

    fn set_desktop_prop_string(&self, value: &str, atom: c_ulong) {
        if let Ok(cstring) = CString::new(value) {
            unsafe {
                (self.lib.XChangeProperty)(
                    self.display,
                    self.root,
                    atom,
                    xlib::XA_CARDINAL,
                    32,
                    xlib::PropModeReplace,
                    cstring.as_ptr() as *const u8,
                    value.len() as i32,
                );
                mem::forget(cstring);
            }
        }
    }

    fn get_window_strut_array_strut_partial(&self, window: Window) -> Option<DockArea> {
        let mut format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut type_return: xlib::Atom = 0;
        let mut bytes_after_return: xlib::Atom = 0;
        let mut prop_return: *mut c_uchar = unsafe { mem::uninitialized() };
        unsafe {
            let status = (self.lib.XGetWindowProperty)(
                self.display,
                window,
                self.xatom.NetWMStrutPartial,
                0,
                4096,
                xlib::False,
                xlib::XA_CARDINAL,
                &mut type_return,
                &mut format_return,
                &mut nitems_return,
                &mut bytes_after_return,
                &mut prop_return,
            );
            if status == i32::from(xlib::Success) {
                #[allow(clippy::cast_ptr_alignment)]
                let array_ptr = prop_return as *const i64;
                let slice = std::slice::from_raw_parts(array_ptr, nitems_return as usize);
                if slice.len() == 12 {
                    return Some(DockArea::from(slice));
                }
                None
            } else {
                None
            }
        }
    }

    //old way to get strut
    fn get_window_strut_array_strut(&self, window: xlib::Window) -> Option<DockArea> {
        let mut format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut type_return: xlib::Atom = 0;
        let mut bytes_after_return: xlib::Atom = 0;
        let mut prop_return: *mut c_uchar = unsafe { std::mem::uninitialized() };
        unsafe {
            let status = (self.lib.XGetWindowProperty)(
                self.display,
                window,
                self.xatom.NetWMStrut,
                0,
                4096,
                xlib::False,
                xlib::XA_CARDINAL,
                &mut type_return,
                &mut format_return,
                &mut nitems_return,
                &mut bytes_after_return,
                &mut prop_return,
            );
            if status == i32::from(xlib::Success) {
                #[allow(clippy::cast_ptr_alignment)]
                let array_ptr = prop_return as *const i64;
                let slice = std::slice::from_raw_parts(array_ptr, nitems_return as usize);
                if slice.len() == 12 {
                    return Some(DockArea::from(slice));
                }
                None
            } else {
                None
            }
        }
    }

    fn reparent(&self, w: Window, new_parent: Window) {
        unsafe {
            (self.lib.XReparentWindow)(
                self.display,
                w,
                new_parent,
                CONFIG.border_width,
                CONFIG.decoration_height + CONFIG.border_width,
            );
        }
    }

    pub fn exit(&self) {
        unsafe {
            (self.lib.XCloseDisplay)(self.display);
        }
    }
}

impl DisplayServer for XlibWrapper {
    fn get_screens(&self) -> Vec<Screen> {
        use x11_dl::xinerama::XineramaScreenInfo;
        use x11_dl::xinerama::Xlib;

        let xlib = Xlib::open().expect("xlibwrapper::core: get_screens");
        let xinerama = unsafe { (xlib.XineramaIsActive)(self.display) } > 0;

        if xinerama {
            let mut screen_count = 0;
            let info_array_raw =
                unsafe { (xlib.XineramaQueryScreens)(self.display, &mut screen_count) };
            info!("screen count: {}", screen_count);
            let xinerama_infos: &[XineramaScreenInfo] =
                unsafe { std::slice::from_raw_parts(info_array_raw, screen_count as usize) };
            xinerama_infos
                .iter()
                .map(|info| {
                    let mut screen = Screen::from(info);
                    screen.root = self.root;
                    screen
                })
                .collect::<Vec<Screen>>()
        } else {
            let roots: Vec<WindowAttributes> = self
                .get_roots()
                .iter()
                .map(|w| self.get_window_attributes(*w))
                .collect();
            roots.iter().map(Screen::from).collect()
        }
    }

    fn update_desktops(&self, current_ws: u32, num_of_ws: Option<u32>) {
        if let Some(num) = num_of_ws {
            let data = vec![num];
            self.set_desktop_prop(&data, self.xatom.NetNumberOfDesktops);
        }
        self.sync(false);
        self.ewmh_current_desktop(current_ws);
    }

    fn init_desktops_hints(&self) {
        //set the number of desktop
        let data = vec![CONFIG.workspaces.len() as u32];
        //let num_ws = self.get_screens().len();
        //let data = vec![num_ws as u32];
        self.set_desktop_prop(&data, self.xatom.NetNumberOfDesktops);
        //set a current desktop
        let data = vec![0 as u32, xlib::CurrentTime as u32];
        self.set_desktop_prop(&data, self.xatom.NetCurrentDesktop);
        //set desktop names
        let mut text: xlib::XTextProperty = unsafe { mem::uninitialized() };
        unsafe {
            let mut clist_tags: Vec<*mut c_char> = CONFIG
                .workspaces
                .values()
                .map(|x| {
                    CString::new(x.to_string())
                        .expect("xlibwrapper::core: init_desktops_hints")
                        .into_raw()
                })
                .collect();
            let ptr = clist_tags.as_mut_ptr();
            (self.lib.Xutf8TextListToTextProperty)(
                self.display,
                ptr,
                clist_tags.len() as i32,
                xlib::XUTF8StringStyle,
                &mut text,
            );
            mem::forget(clist_tags);
            (self.lib.XSetTextProperty)(
                self.display,
                self.root,
                &mut text,
                self.xatom.NetDesktopNames,
            );
            self.support_wm_check();
        }

        //set a viewport
        let data = vec![0 as u32, 0 as u32];
        self.set_desktop_prop(&data, self.xatom.NetDesktopViewport);
    }

    fn get_window_states_atoms(&self, window: xlib::Window) -> Vec<xlib::Atom> {
        let mut format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut bytes_remaining: c_ulong = 0;
        let mut type_return: xlib::Atom = 0;
        let mut prop_return: *mut c_uchar = unsafe { std::mem::uninitialized() };
        unsafe {
            let status = (self.lib.XGetWindowProperty)(
                self.display,
                window,
                self.xatom.NetWMState,
                0,
                4096i64 / 4,
                xlib::False,
                xlib::XA_ATOM,
                &mut type_return,
                &mut format_return,
                &mut nitems_return,
                &mut bytes_remaining,
                &mut prop_return,
            );
            if status == i32::from(xlib::Success) && !prop_return.is_null() {
                #[allow(clippy::cast_lossless, clippy::cast_ptr_alignment)]
                //let result = *(prop_return as *const u32);
                let ptr = prop_return as *const u64;
                let results: &[xlib::Atom] =
                    std::slice::from_raw_parts(ptr, nitems_return as usize);
                return results.to_vec();
            }
            vec![]
        }
    }

    fn set_window_states_atoms(&self, window: xlib::Window, states: Vec<xlib::Atom>) {
        let data: Vec<u32> = states.iter().map(|x| *x as u32).collect();
        unsafe {
            (self.lib.XChangeProperty)(
                self.display,
                window,
                self.xatom.NetWMState,
                xlib::XA_ATOM,
                32,
                xlib::PropModeReplace,
                data.as_ptr() as *const u8,
                data.len() as i32,
            );
            std::mem::forget(data);
        }
    }

    fn set_desktop_prop_u64(&self, value: u64, atom: c_ulong, type_: c_ulong) {
        let data = vec![value as u32];
        unsafe {
            (self.lib.XChangeProperty)(
                self.display,
                self.root,
                atom,
                type_,
                32,
                xlib::PropModeReplace,
                data.as_ptr() as *const u8,
                1 as i32,
            );
            mem::forget(data);
        }
    }

    fn atom_name(&self, atom: xlib::Atom) -> Result<String, Box<dyn std::error::Error>> {
        use std::ffi::CStr;
        unsafe {
            let ret = (self.lib.XGetAtomName)(self.display, atom);
            Ok(CStr::from_ptr(ret).to_str()?.to_string())
        }
    }

    fn add_to_save_set(&self, w: Window) {
        unsafe {
            (self.lib.XAddToSaveSet)(self.display, w);
        }
    }

    fn set_input_focus(&self, w: Window) {
        unsafe {
            (self.lib.XSetInputFocus)(
                self.display,
                w,
                xlib::RevertToPointerRoot,
                xlib::CurrentTime,
            );
        }
    }

    fn take_focus(&self, w: Window) {
        unsafe {
            (self.lib.XSetInputFocus)(
                self.display,
                w,
                xlib::RevertToPointerRoot,
                xlib::CurrentTime,
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
                1,
            );
            mem::forget(list);
            self.flush();
        }
        //self.send_xevent_atom(w, self.xatom.WMTakeFocus);
        self.sync(false);
    }

    fn set_window_background_color(&self, w: Window, color: Color) {
        if w == self.root {
            return;
        }
        let color = color.value();
        unsafe {
            let res = (self.lib.XSetWindowBackground)(self.display, w, color);
            self.unmap_window(w);
            self.map_window(w);
            (self.lib.XSync)(self.display, 0);
        }
    }

    fn set_border_color(&self, w: Window, color: Color) {
        if w == self.root {
            return;
        }

        let color = color.value();

        unsafe {
            (self.lib.XSetWindowBorder)(self.display, w, color);
            (self.lib.XSync)(self.display, 0);
        }
    }

    fn window_under_pointer(&self) -> Option<Window> {
        unsafe {
            let mut root_return = 0;
            let mut child_return = 0;
            let mut root_x = 0i32;
            let mut root_y = 0i32;
            let mut win_x = 0i32;
            let mut win_y = 0i32;
            let mut mask = 0u32;
            if (self.lib.XQueryPointer)(
                self.display,
                self.root,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            ) == 0
            {
                warn!("Query pointer retured false")
            }

            if child_return != 0 {
                Some(child_return)
            } else {
                None
            }
        }
    }

    fn pointer_pos(&self, w: Window) -> Position {
        unsafe {
            let mut root_return = 0;
            let mut child_return = 0;
            let mut root_x = 0i32;
            let mut root_y = 0i32;
            let mut win_x = 0i32;
            let mut win_y = 0i32;
            let mut mask = 0u32;
            if (self.lib.XQueryPointer)(
                self.display,
                //self.root,
                w,
                &mut root_return,
                &mut child_return,
                &mut root_x,
                &mut root_y,
                &mut win_x,
                &mut win_y,
                &mut mask,
            ) == 0
            {
                warn!("Query pointer retured false")
            }
            Position {
                x: root_x,
                y: root_y,
            }
        }
    }

    fn move_cursor(&self, pos: Position) {
        unsafe {
            (self.lib.XWarpPointer)(self.display, 0, self.root, 0, 0, 0, 0, pos.x, pos.y);
            (self.lib.XFlush)(self.display);
        }
    }

    fn center_cursor(&self, w: Window) {
        let geom = self.get_geometry(w);
        let pos = Position {
            x: (geom.width / 2) as i32,
            y: (geom.height / 2) as i32,
        };
        unsafe {
            (self.lib.XWarpPointer)(self.display, 0, w, 0, 0, 0, 0, pos.x, pos.y);
            (self.lib.XFlush)(self.display);
        }
    }

    fn configure_window(&self, window: Window, value_mask: Mask, changes: WindowChanges) {
        unsafe {
            let mut raw_changes = xlib::XWindowChanges {
                x: changes.x,
                y: changes.y,
                width: changes.width,
                height: changes.height,
                border_width: changes.border_width,
                sibling: changes.sibling,
                stack_mode: changes.stack_mode,
            };

            (self.lib.XConfigureWindow)(self.display, window, value_mask as u32, &mut raw_changes);
        }
    }

    fn add_to_root_net_client_list(&self, w: Window) {
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
                1,
            );
            mem::forget(list);
        }
    }

    fn update_net_client_list(&self, clients: Vec<Window>) {
        unsafe {
            (self.lib.XDeleteProperty)(self.display, self.root, self.xatom.NetClientList);
            clients.iter().for_each(|c| {
                let list = vec![c];
                (self.lib.XChangeProperty)(
                    self.display,
                    self.root,
                    self.xatom.NetClientList,
                    xlib::XA_WINDOW,
                    32,
                    xlib::PropModeAppend,
                    list.as_ptr() as *const u8,
                    1,
                );
                mem::forget(list);
            })
        }
    }

    fn create_simple_window(
        &self,
        w: Window,
        pos: Position,
        size: Size,
        border_width: u32,
        border_color: Color,
        bg_color: Color,
    ) -> Window {
        unsafe {
            (self.lib.XCreateSimpleWindow)(
                self.display,
                w,
                pos.x,
                pos.y,
                size.width as u32,
                size.height as u32,
                border_width,
                border_color.value(),
                bg_color.value(),
            )
        }
    }

    fn destroy_window(&self, w: Window) {
        unsafe {
            (self.lib.XDestroyWindow)(self.display, w);
        }
    }

    fn get_geometry(&self, w: Window) -> Geometry {
        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            let _status = (self.lib.XGetWindowAttributes)(self.display, w, &mut attr);

            Geometry {
                x: attr.x,
                y: attr.y,
                width: attr.width as u32,
                height: attr.height as u32,
            }
        }
    }

    fn get_root(&self) -> Window {
        self.root
    }

    fn get_window_attributes(&self, w: Window) -> WindowAttributes {
        unsafe {
            let mut attr: xlib::XWindowAttributes = mem::uninitialized();
            (self.lib.XGetWindowAttributes)(self.display, w, &mut attr);
            WindowAttributes::from(attr)
        }
    }

    fn grab_server(&self) {
        unsafe {
            (self.lib.XGrabServer)(self.display);
        }
    }

    fn ungrab_server(&self) {
        unsafe {
            (self.lib.XUngrabServer)(self.display);
        }
    }

    fn ungrab_keys(&self, _w: Window) {
        unsafe {
            (self.lib.XUngrabKey)(self.display, xlib::AnyKey, xlib::AnyModifier, self.root);
        }
    }

    fn ungrab_all_buttons(&self, w: Window) {
        unsafe {
            (self.lib.XUngrabButton)(self.display, xlib::AnyButton as u32, xlib::AnyModifier, w);
        }
    }

    fn grab_button(
        &self,
        button: u32,
        modifiers: u32,
        grab_window: Window,
        owner_events: bool,
        event_mask: u32,
        pointer_mode: i32,
        keyboard_mode: i32,
        confine_to: Window,
        cursor: u64,
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
                cursor,
            );
        }
    }

    fn str_to_keycode(&self, key: &str) -> Option<KeyCode> {
        match keysym_lookup::into_keysym(key) {
            Some(key) => Some(self.key_sym_to_keycode(key.into())),
            None => None,
        }
    }

    fn keycode_to_key_sym(&self, keycode: KeyCode) -> Result<String, Box<dyn std::error::Error>> {
        use std::ffi::CStr;
        unsafe {
            let ret = (self.lib.XKeycodeToKeysym)(self.display, keycode, 0);
            if ret == xlib::NoSymbol as u64 {
                return Err("NoSymbol".into());
            }
            let ret = (self.lib.XKeysymToString)(ret);
            let c_str: &CStr = CStr::from_ptr(ret);
            let str_slice: &str = c_str.to_str()?;
            Ok(str_slice.to_owned())
        }
    }

    fn key_sym_to_keycode(&self, keysym: u64) -> KeyCode {
        unsafe { (self.lib.XKeysymToKeycode)(self.display, keysym) }
    }

    fn get_window_type_atom(&self, w: Window) -> Option<xlib::Atom> {
        self.get_atom_prop_value(w, self.xatom.NetWMWindowType)
    }

    fn get_class_hint(&self, w: Window) -> Result<(String, String), Box<dyn std::error::Error>> {
        unsafe {
            let mut hint_return = MaybeUninit::<xlib::XClassHint>::zeroed();

            (self.lib.XGetClassHint)(self.display, w, hint_return.as_mut_ptr());

            if std::ptr::eq((*hint_return.as_ptr()).res_class, null::<i8>())
                || std::ptr::eq((*hint_return.as_ptr()).res_class, null::<i8>())
            {
                return Err("XClassHint uninitialized".into());
            }

            let class = CString::from_raw(hint_return.assume_init().res_class).into_string();

            let name = CString::from_raw(hint_return.assume_init().res_name).into_string();

            match (class, name) {
                (Ok(class), Ok(name)) => Ok((class, name)),
                _ => Err("Failed to create string from raw string".into()),
            }
        }
    }

    fn get_atom_prop_value(&self, window: xlib::Window, prop: xlib::Atom) -> Option<xlib::Atom> {
        // Shamelessly stolen from lex148/leftWM
        let mut format_return: i32 = 0;
        let mut nitems_return: c_ulong = 0;
        let mut type_return: xlib::Atom = 0;
        let mut prop_return: *mut c_uchar = unsafe { mem::uninitialized() };
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

    fn grab_keys(&self, _w: Window, keysym: u32, modifiers: u32) {
        let code = self.key_sym_to_keycode(keysym as u64);

        let mods: Vec<u32> = vec![
            modifiers,
            modifiers | xlib::Mod2Mask,
            modifiers | xlib::LockMask,
        ];

        mods.into_iter().for_each(|m| {
            self.grab_key(
                code as u32,
                m,
                self.root,
                true,
                GrabModeAsync,
                GrabModeAsync,
            )
        });
    }

    fn grab_key(
        &self,
        key_code: u32,
        modifiers: u32,
        grab_window: Window,
        owner_event: bool,
        pointer_mode: i32,
        keyboard_mode: i32,
    ) {
        unsafe {
            // add error handling.. Like really come up with a strategy!
            (self.lib.XGrabKey)(
                self.display,
                key_code as i32,
                modifiers,
                grab_window,
                to_c_bool(owner_event),
                pointer_mode,
                keyboard_mode,
            );
        }
    }

    fn kill_client(&self, w: Window) -> bool {
        if !self.send_xevent_atom(w, self.xatom.WMDelete) {
            unsafe {
                (self.lib.XGrabServer)(self.display);
                (self.lib.XSetCloseDownMode)(self.display, xlib::DestroyAll);
                (self.lib.XKillClient)(self.display, w);
                (self.lib.XSync)(self.display, xlib::False);
                (self.lib.XUngrabServer)(self.display);
            }
        }

        !self.get_top_level_windows().contains(&w)
    }

    fn map_window(&self, window: Window) {
        unsafe {
            (self.lib.XMapWindow)(self.display, window);
        }
    }

    fn move_window(&self, w: Window, position: Position) {
        let Position { x, y } = position;
        unsafe {
            (self.lib.XMoveWindow)(self.display, w, x, y);
        }
    }

    fn next_event(&self) -> xlib::XEvent {
        unsafe {
            let mut event: xlib::XEvent = mem::uninitialized();
            (self.lib.XNextEvent)(self.display, &mut event);
            event
        }
    }

    fn raise_window(&self, w: Window) {
        unsafe {
            (self.lib.XRaiseWindow)(self.display, w);
        }
    }

    fn resize_window(&self, w: Window, size: Size) {
        unsafe {
            (self.lib.XResizeWindow)(self.display, w, size.width as u32, size.height as u32);
        }
    }

    fn select_input(&self, window: xlib::Window, masks: Mask) {
        unsafe {
            (self.lib.XSelectInput)(self.display, window, masks);
        }
    }

    fn set_border_width(&self, w: Window, border_width: u32) {
        if w == self.root {
            return;
        }
        unsafe {
            (self.lib.XSetWindowBorderWidth)(self.display, w, border_width);
        }
    }

    fn sync(&self, discard: bool) {
        unsafe {
            (self.lib.XSync)(self.display, discard as i32);
        }
    }

    fn flush(&self) {
        unsafe {
            (&self.lib.XFlush)(self.display);
        }
    }

    fn get_window_strut_array(&self, window: Window) -> Option<DockArea> {
        if let Some(d) = self.get_window_strut_array_strut_partial(window) {
            return Some(d);
        }
        if let Some(d) = self.get_window_strut_array_strut(window) {
            return Some(d);
        }
        None
    }

    //new way to get strut

    fn get_window_type(&self, window: xlib::Window) -> WindowType {
        if let Some(value) = self.get_atom_prop_value(window, self.xatom.NetWMWindowType) {
            if value == self.xatom.NetWMWindowTypeDesktop {
                return WindowType::Desktop;
            }
            if value == self.xatom.NetWMWindowTypeDock {
                return WindowType::Dock;
            }
            if value == self.xatom.NetWMWindowTypeToolbar {
                return WindowType::Toolbar;
            }
            if value == self.xatom.NetWMWindowTypeMenu {
                return WindowType::Menu;
            }
            if value == self.xatom.NetWMWindowTypeUtility {
                return WindowType::Utility;
            }
            if value == self.xatom.NetWMWindowTypeSplash {
                return WindowType::Splash;
            }
            if value == self.xatom.NetWMWindowTypeDialog {
                return WindowType::Dialog;
            }
        }
        WindowType::Normal
    }

    fn get_upmost_window(&self) -> Option<Window> {
        match self.get_top_level_windows().last() {
            Some(x) => Some(*x),
            None => None,
        }
    }

    fn reparent_client(&self, w: Window, size: Size, pos: Position) -> Window {
        let attr = self.get_window_attributes(w);
        //let pos = Position {x: attr.x, y: attr.y};
        //let size = Size {width: attr.width, height: attr.height};
        let frame = self.create_simple_window(
            self.get_root(),
            pos,
            size,
            0,
            CONFIG.border_color,
            CONFIG.background_color,
        );
        self.select_input(frame, SubstructureRedirectMask | SubstructureNotifyMask);
        self.add_to_save_set(w);
        self.reparent(w, frame);
        //self.map_window(frame);
        frame
    }

    fn get_top_level_windows(&self) -> Vec<Window> {
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
                &mut num_top_level_windows,
            );

            let windows =
                std::slice::from_raw_parts(top_level_windows, num_top_level_windows as usize);
            Vec::from(windows)
        }
    }

    fn top_level_window_count(&self) -> u32 {
        self.get_top_level_windows().len() as u32
    }

    fn transient_for_hint(&self, w: Window) -> Option<Window> {
        unsafe {
            let mut prop_window = mem::uninitialized();
            let res = (self.lib.XGetTransientForHint)(self.display, w, &mut prop_window);

            if from_c_bool(res) {
                Some(prop_window)
            } else {
                None
            }
        }
    }

    fn unmap_window(&self, w: Window) {
        unsafe {
            (self.lib.XUnmapWindow)(self.display, w);
            self.sync(false);
        }
    }

    fn should_be_managed(&self, w: Window) -> bool {
        if let Some(prop_val) = self.get_window_type_atom(w) {
            if vec![
                self.xatom.NetWMWindowTypeDock,
                self.xatom.NetWMWindowTypeToolbar,
                self.xatom.NetWMWindowTypeUtility,
                self.xatom.NetWMWindowTypeDialog,
                self.xatom.NetWMWindowTypeSplash,
                self.xatom.NetWMWindowTypeMenu,
            ]
            .contains(&prop_val)
            {
                return false;
            }
        }

        if self.get_window_attributes(w).override_redirect {
            return false;
        }
        true
    }

    fn xatom(&self) -> &XAtom {
        &self.xatom
    }
}
