use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::masks::*;

use std::rc::Rc;
use std::process::Command;

pub fn key_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, state, keycode) =
        match event {
            Event {
                event_type: EventType::KeyPress,
                payload: Some(EventPayload::KeyPress(w, state, keycode))
            } => (w, state, keycode),
            _ => { return; }
        };

    let ww = match wm.clients.get(&w) {
        Some(ww) => ww,
        None => { return; }
    };


    if (state & (Mod4Mask | Shift)) != 0 {
        let keycode = keycode as u8;

        let width = ww.get_width();
        let height = ww.get_height();

        if xlib.str_to_keycode("Right").unwrap() == keycode {
            wm.resize_window(w, width + 10, height);
            return;
        }
        if xlib.str_to_keycode("Left").unwrap() == keycode {
            wm.resize_window(w, width - 10, height);
            return;
        }
        if xlib.str_to_keycode("Down").unwrap() == keycode {
            wm.resize_window(w, width, height + 10);
            return;
        }
        if xlib.str_to_keycode("Up").unwrap() == keycode {
            wm.resize_window(w, width, height - 10);
            return;
        }
        if xlib.str_to_keycode("q").unwrap() == keycode {
            wm.kill_window(w);
        }

    }
    if (state | Mod4Mask) != 0 {
        let keycode = keycode as u8;
        if xlib.str_to_keycode("Return").unwrap() == keycode {
            println!("Start terminal");
            match Command::new("alacritty").spawn() {
                Ok(_) => {},
                Err(e) => eprintln!("Failed to open terminal. Error: {}", e)
            }
        }
    }
}
