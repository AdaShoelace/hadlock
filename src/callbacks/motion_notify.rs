use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::util::Position;
use crate::xlibwrapper::masks::*;
use std::rc::Rc;

pub fn motion_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, x_root, y_root, state) =
        match event.event_type {
            EventType::MotionNotify => match event.payload {
                Some(EventPayload::MotionNotify(w, x_root, y_root, state)) => (w, x_root, y_root, state),
                _ => { return; }
            },
            _ => { return; }
        };
    let root = xlib.get_root();
    if !wm.clients.contains_key(&w) && w != root {
        return
    }

    if w == root && wm.focus_screen != wm.get_focused_screen() {
        wm.focus_screen = wm.get_focused_screen();
        let screen = wm.focus_screen.clone();
        wm.set_current_ws_by_screen(&screen);
        {
            use std::fs::{File, OpenOptions};
            use std::io::{Write};
            let path = "/home/pierre/hadlock.log";
            match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                {
                    Ok(mut x) => {
                        let pointer_pos = xlib.pointer_pos();
                        let output_text = &format!("setting new focus screen: {:?}", wm.focus_screen);
                        write!(x, "{}\n", output_text);
                    },
                    Err(e) => println!("{}", e)
                };
        }
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
