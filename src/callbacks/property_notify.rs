use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
    masks::*,
};
use std::rc::Rc;


pub fn property_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, atom) = match event {
            Event {
                event_type: EventType::PropertyNotify,
                payload: Some(EventPayload::PropertyNotify(window, atom))
            } => (window, atom),
            _ => { return; }
        };
    if atom == xlib.xatom.WMNormalHints {
    }

}
