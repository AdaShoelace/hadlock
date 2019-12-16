use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;

use x11_dl::xlib;
use std::rc::Rc;


pub fn configure_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, window_changes, value_mask) =
        match event {
            Event {
                event_type: EventType::ConfigurationRequest,
                payload: Some(EventPayload::ConfigurationRequest(w, window_changes, value_mask))
            } => (w, window_changes, value_mask),
            _ => { return; }
        };

    if value_mask & (xlib::CWX | xlib::CWY) as u64 == (xlib::CWX | xlib::CWY) as u64 { return }

    if wm.current_monitor().expect("configure_request: current_monitor 1").contains_window(w) {
        return;
    }

    xlib.configure_window(
        w,
        value_mask as i64,
        window_changes
    );
}
