use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::masks::*;

use std::rc::Rc;
use std::process::Command;

pub fn key_press(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {
    //println!("keypress registered");
    let (w, state, keycode) =
        match event {
            Event {
                event_type: EventType::KeyPress,
                payload: Some(EventPayload::KeyPress(w, state, keycode))
            } => (w, state, keycode),
            _ => { return; }
        };


    let mod_not_shift = (state & (Mod4Mask | Shift)) == Mod4Mask;
    let mod_and_shift = (state & (Mod4Mask | Shift)) == Mod4Mask | Shift;

    match wm.clients.get(&wm.focus_w) {
        Some(ww) => {
            let keycode = keycode as u8;
            if mod_not_shift && xlib.str_to_keycode("Return").unwrap() == keycode {
                println!("just mod and enter");
                spawn_terminal();
            }

            if mod_and_shift {
                println!("For some godforsaken reason we are here to...");
                let w = ww.window();
                let width = ww.get_width();
                let height = ww.get_height();
                wm.center_cursor(w);

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

            if mod_not_shift {
                println!("Number pressed");
                let ws_keys: Vec<u8> = (1..=9).map(|x| {
                    xlib.str_to_keycode(&x.to_string()).unwrap()
                }).collect();

                match ws_keys.contains(&keycode) {
                    true  => {
                        let ws_num = ((keycode - 10) % 10) + 1;
                        wm.set_current_ws(ws_num as u32);
                    },
                    _ => {}
                }
            }
        },
        None if w == xlib.get_root() => {
            if mod_not_shift {
                let keycode = keycode as u8;
                if xlib.str_to_keycode("Return").unwrap() == keycode {
                    spawn_terminal();
                }

                let ws_keys: Vec<u8> = (1..=9).map(|x| {
                    xlib.str_to_keycode(&x.to_string()).unwrap()
                }).collect();

                match ws_keys.contains(&keycode) {
                    true  => {
                        let ws_num = ((keycode - 10) % 10) + 1;
                        wm.set_current_ws(ws_num as u32);
                    },
                    _ => {}
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
