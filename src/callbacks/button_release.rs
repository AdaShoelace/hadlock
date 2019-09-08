use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;

use std::rc::Rc;

pub fn button_release(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, _x_root, _y_root, _state) =
        match event {
            Event {
                event_type: EventType::ButtonRelease,
                payload: Some(EventPayload::ButtonRelease(window, _sub_window, _button, x_root, y_root, state))
            } => (window, x_root, y_root, state),
            _ => { return; }
        };


    if !wm.clients.contains_key(&window) || window == xlib.get_root() {
        return
    }
    
    println!("Focused window: {}", wm.focus_w);

    //println!("Button released at: {}", window);

    let _ww = wm.clients.get(&window).expect("ButtonPressed: No such window in client list");
}
