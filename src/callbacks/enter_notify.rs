use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;

pub fn enter_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, _sub_w) = match event {
        Event {
            event_type: EventType::EnterNotify,
            payload: Some(EventPayload::EnterNotify(w, sub_w))
        } => (w, sub_w),
        _ => { return; }
    };


    if !wm.current_monitor().contains_window(w) && w != wm.lib.get_root() {
        println!("Calling window {} not in client list", w);
        return;
    }
    wm.unset_focus(wm.focus_w);
    wm.set_focus(w);
}
