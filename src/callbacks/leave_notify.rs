
use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
//use crate::config::*;
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
    if !wm.current_monitor().expect("leave_notify: current_monitor 1").contains_window(w) || w == xlib.get_root() {
        return;
    }

    wm.unset_focus(w);
}
