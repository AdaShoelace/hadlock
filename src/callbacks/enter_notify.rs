use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Color;
use crate::config::*;
use std::rc::Rc;

pub fn enter_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, _sub_w) = match event {
        Event {
            event_type: EventType::EnterNotify,
            payload: Some(EventPayload::EnterNotify(w, sub_w))
        } => (w, sub_w),
        _ => { return; }
    };


    if !wm.clients.contains_key(&w) {
        println!("Calling window {} not in client list", w);
        return;
    }

    let ww = wm.clients.get(&w).expect("OnEnter: No such window in client list");

    xlib.remove_focus(wm.focus_w);
    wm.focus_w = ww.window();

    xlib.ungrab_all_buttons(w);
    wm.grab_buttons(w);
    xlib.ungrab_keys(w);
    wm.grab_keys(w);

    match ww.get_dec() {
        Some(dec) => {
            xlib.set_border_color(dec, CONFIG.border_color);
        },
        None => {
            xlib.set_border_color(w, CONFIG.background_color);
        }
    }
    // need to rethink focus for non floating modes
    xlib.take_focus(w);
}
