use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;


pub fn destroy_window(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event {
        Event {
            event_type: EventType::DestroyWindow,
            payload: Some(EventPayload::DestroyWindow(w))
        } => w,
        _ => { return; }
    };

    wm.kill_window(w);
}
