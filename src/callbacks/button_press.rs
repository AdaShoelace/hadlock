use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
    masks::*,
};
use std::rc::Rc;


pub fn button_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (window, sub_win, x_root, y_root, _state, button) =
        match event {
            Event::ButtonPress{win, sub_win, x_root, y_root, state, button}  => (win, sub_win, x_root, y_root, state, button),
            _ => { return; }
        };

    if !wm.current_monitor().expect("button_press: current_monitor 1").contains_window(window) || window == xlib.get_root() {
        return
    }

    debug!("Button pressed from: {}", window);

    let geometry = xlib.get_geometry(window);

    wm.drag_start_pos = (x_root as i32 , y_root as i32);
    wm.drag_start_frame_pos = (geometry.x,geometry.y);
    wm.drag_start_frame_size = (geometry.width, geometry.height);

    if button == Button1 {
        debug!("Button1 pressed");
        let ww = wm.current_monitor().expect("button_press: current_monitor 2").get_client(window).expect(&format!("Button press no client: {}", window)).clone();
        debug!("Pointer location: {:?}", xlib.pointer_pos());
        match xlib.get_upmost_window() {
            Some(x) if x != window => wm.raise_window(&ww),
            _ => { return }
        }
    }
}
