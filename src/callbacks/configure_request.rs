use crate::windowmanager::WindowManager;
use crate::xlibwrapper::xlibmodels::*;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;


pub fn configure_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, window_changes, value_mask) =
        match event.event_type {
            EventType::ConfigurationRequest => match event.payload {
                Some(EventPayload::ConfigurationRequest(w, window_changes, value_mask)) => (w, window_changes, value_mask),
                _ => { return; }
            },
            _ => { return; }
        };


    //println!("on_configure_request");
    let changes = WindowChanges {
        x: window_changes.x,
        y: window_changes.y,
        width: window_changes.width,
        height: window_changes.height,
        border_width: window_changes.border_width,
        sibling: window_changes.sibling,
        stack_mode: window_changes.stack_mode,
    };

    if wm.clients.contains_key(&w) {
        let frame = wm.clients.get(&w).expect("ConfigureWindow: No such window in client list");
        xlib.configure_window(
            frame.window(),
            value_mask as i64,
            changes.clone()
        );
    }
    xlib.configure_window(
        w,
        value_mask as i64,
        window_changes
    );
}
