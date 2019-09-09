use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
    masks::*,
};
use std::rc::Rc;


pub fn button_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, x_root, y_root, _state, button) =
        match event {
            Event {
                event_type: EventType::ButtonPress,
                payload: Some(EventPayload::ButtonPress(window, _sub_window, button, x_root, y_root, state))
            } => (window, x_root, y_root, state, button),
            _ => { return; }
        };

    if !wm.clients.contains_key(&window) || window == xlib.get_root() {
        return
    }

    println!("Button pressed from: {}", window);

    let ww = wm.clients.get(&window).expect("ButtonPressed: No such window in client list");
    let geometry = xlib.get_geometry(ww.window());

    wm.drag_start_pos = (x_root as i32 , y_root as i32);
    wm.drag_start_frame_pos = (geometry.x,geometry.y);
    wm.drag_start_frame_size = (geometry.width, geometry.height);

    if button == Button1 {
        println!("Button1 pressed");
        wm.raise_window(&ww);
    }
}
