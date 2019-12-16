use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
};
use std::rc::Rc;


pub fn property_notify(_xlib: Rc<XlibWrapper>, _wm: &mut WindowManager, event: Event) {

    let (_window, _atom) = match event {
            Event {
                event_type: EventType::PropertyNotify,
                payload: Some(EventPayload::PropertyNotify(window, atom))
            } => (window, atom),
            _ => { return; }
        };

}
