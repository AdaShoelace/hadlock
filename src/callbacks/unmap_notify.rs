use crate::windowmanager::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::core::*;
use std::rc::Rc;

pub fn unmap_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event.event_type {
        EventType::UnmapNotify => match event.payload {
            Some(EventPayload::UnmapNotify(w)) => w,
            _ => { return; }
        },
        _ => { return; }
    };
    wm.hide_client(w);
    //wm.lib.unmap_window(w);
}

