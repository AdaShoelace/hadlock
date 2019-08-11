use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Color;
use std::rc::Rc;

pub fn button_release(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, x_root, y_root) =
        match event {
            Event {
                event_type: EventType::ButtonRelease,
                payload: Some(EventPayload::ButtonRelease(window, _sub_window, _button, x_root, y_root, _state))
            } => (window, x_root, y_root),
            _ => { return; }
        };


    if !wm.clients.contains_key(&window) || window == xlib.get_root() {
        return
    }

    println!("Button released at: {}", window);

    let ww = wm.clients.get(&window).expect("ButtonPressed: No such window in client list");

    match ww.get_dec() {
        Some(dec) => {
            xlib.raise_window(dec);
            xlib.raise_window(ww.window());
        },
        None => xlib.raise_window(ww.window())
    }

    //xlib.take_focus(ww.window());
}
