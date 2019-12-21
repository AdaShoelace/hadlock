use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Position;
use crate::xlibwrapper::masks::*;
use std::rc::Rc;

pub fn motion_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, x_root, y_root, state) = match event {
        Event::MotionNotify{win, x_root, y_root, state} => (win, x_root, y_root, state),
        _ => { return; }
    };

    let root = xlib.get_root();
    wm.set_current_monitor_by_mouse();
    let current = wm.current_monitor().expect("motion_notify: current_monitor 1").get_current_ws_tag();
    wm.lib.update_desktops(current, None);
    if !wm.current_monitor().expect("motion_notify: current_monitor 2").contains_window(w) && w == root {
        return
    }

    let drag_pos = Position { x: x_root, y: y_root };
    let (delta_x, delta_y) =  (drag_pos.x - wm.drag_start_pos.0,
        drag_pos.y - wm.drag_start_pos.1);
    let dest_pos = Position{ x: wm.drag_start_frame_pos.0 + delta_x,
    y: wm.drag_start_frame_pos.1 + delta_y};

    if (state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
        wm.move_window(w, dest_pos.x, dest_pos.y);
    }
}
