use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::masks::*;

use std::rc::Rc;
use std::process::Command;

pub fn key_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {
    println!("keypress registered");
    let (w, state, keycode) =
        match event {
            Event {
                event_type: EventType::KeyPress,
                payload: Some(EventPayload::KeyPress(w, state, keycode))
            } => (w, state, keycode),
            _ => { return; }
        };

    match wm.clients.get(&wm.focus_w) {
        Some(ww) => {
            let keycode = keycode as u8;
            if ((state & !Shift) & Mod4Mask) != 0 && xlib.str_to_keycode("Return").unwrap() == keycode {
                println!("just mod and enter");
                spawn_terminal();
            }

            if (state & (Mod4Mask | Shift)) != 0 {
                println!("For some godforsaken reason we are here to...");
                let w = ww.window();
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
            }          },
            None if w == xlib.get_root() => {
                if (state & Mod4Mask) != 0 {
                    let keycode = keycode as u8;
                    if xlib.str_to_keycode("Return").unwrap() == keycode {
                        spawn_terminal();
                    }
                }
            }
        None => { return; }
    };
}

fn spawn_terminal() {
    match Command::new("alacritty").spawn() {
        Ok(_) => {},
        Err(e) => eprintln!("Failed to open terminal. Error: {}", e)
    }
}
