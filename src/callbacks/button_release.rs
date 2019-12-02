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


    if !wm.current_monitor().expect("button_release: current_monitor 1").contains_window(window) || window == xlib.get_root() {
        return
    }
    
    println!("Focused window: {}", wm.focus_w);

    //println!("Button released at: {}", window);

    let _ww = wm.current_monitor().expect("button_release: current_monitor 2").get_client(window).expect("ButtonPressed: No such window in client list");
}
