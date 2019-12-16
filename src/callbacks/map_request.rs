use crate::windowmanager::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::core::*;
use std::rc::Rc;

pub fn map_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event.event_type {
        EventType::MapRequest => match event.payload {
            Some(EventPayload::MapRequest(w)) => w,
            _ => { return; }
        },
        _ => { return; }
    };
    let (class, name) = xlib.get_class_hint(w);
    debug!("Mapped class: {}, name: {}", class, name);
    wm.setup_window(w);
    xlib.map_window(w);
}

