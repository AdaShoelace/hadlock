
use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::config::*;
use std::rc::Rc;

pub fn leave_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event {
        Event {
            event_type: EventType::LeaveNotify,
            payload: Some(EventPayload::LeaveNotify(w))
        } => w,
        _ => { return; }
    };

    // this check is an ugly hack to not crash when decorations window gets destroyed before
    // client and client recieves an "OnLeave"-event
    if !wm.clients.contains_key(&w) || w == xlib.get_root() {
        return;
    }

    let ww = wm.clients.get(&w).expect("OnLeave: No such window in client list");

    match ww.get_dec() {
        Some(dec) => xlib.set_border_color(dec, CONFIG.background_color),
        None => xlib.set_border_color(ww.window(), CONFIG.background_color)
    }
}
