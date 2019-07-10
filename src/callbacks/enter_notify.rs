use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Color;
use std::rc::Rc;

pub fn enter_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event {
        Event {
            event_type: EventType::EnterNotify,
            payload: Some(EventPayload::EnterNotify(w))
        } => w,
        _ => { return; }
    };

    if !wm.clients.contains_key(&w) {
        return;
    }

    let ww = wm.clients.get(&w).expect("OnEnter: No such window in client list");

    match ww.get_dec() {
        Some(dec) => {
            xlib.set_border_color(dec, Color::SolarizedCyan);
        },
        None => {
            xlib.set_border_color(w, Color::SolarizedPurple);
        }
    }
    // need to rethink focus for non floating modes
    xlib.take_focus(w);
}
