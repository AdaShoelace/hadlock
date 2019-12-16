use crate::windowmanager::WindowManager;
use crate::xlibwrapper::{
    core::*,
    event::*,
    masks::*
};
use crate::models::Direction;
use crate::config::*;

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

    let ws_keys: Vec<u8> = (1..=9).map(|x| {
        xlib.str_to_keycode(&x.to_string()).expect("key_press 1")
    }).collect();

    let focus_w = wm.focus_w;

    match wm.current_monitor().expect("key_press: current_monitor 1").get_client(focus_w) {
        Some(ww) => {
            let keycode = keycode as u8;

            if mod_not_shift && xlib.str_to_keycode("Return").expect("key_press: 2") == keycode {
                spawn_process(CONFIG.term.as_str(), vec![]);
            }

            if mod_and_shift {
                let w = ww.window();
                let width = ww.get_width();
                let height = ww.get_height();

                if xlib.str_to_keycode("Right").expect("key_press: 3") == keycode {
                    wm.resize_window(w, width + 10, height);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Left").expect("key_press: 4") == keycode {
                    wm.resize_window(w, width - 10, height);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Down").expect("key_press: 5") == keycode {
                    wm.resize_window(w, width, height + 10);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Up").expect("key_press: 6") == keycode {
                    wm.resize_window(w, width, height - 10);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("q").expect("key_press: 7") == keycode {
                    wm.kill_window(w);
                }
                if xlib.str_to_keycode("e").expect("key_press: 8") == keycode {
                    xlib.exit();
                }
                if xlib.str_to_keycode("f").expect("key_press: 8") == keycode {
                    //wm.toggle_monocle(w);
                }

                match ws_keys.contains(&keycode) {
                    true  => {
                        let ws_num = keycode_to_ws(keycode);
                        wm.move_to_ws(w, ws_num as u8);
                        wm.set_current_ws(ws_num);
                    },
                    _ => {}
                }

            }

            if mod_not_shift {
                println!("Number pressed");

                if xlib.str_to_keycode("f").expect("Dafuq?!?!") == keycode {
                    wm.toggle_maximize(wm.focus_w);
                }
                if xlib.str_to_keycode("Right").expect("key_press: 9") == keycode || xlib.str_to_keycode("l").expect("key_press: 10") == keycode {
                    wm.shift_window(wm.focus_w, Direction::East);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Left").expect("key_press: 11") == keycode || xlib.str_to_keycode("h").expect("key_press: 12") == keycode {
                    wm.shift_window(wm.focus_w, Direction::West);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Down").expect("key_press: 13") == keycode || xlib.str_to_keycode("j").expect("key_press: 14") == keycode  {
                    wm.shift_window(wm.focus_w, Direction::South);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("Up").expect("key_press: \"Up\"") == keycode || xlib.str_to_keycode("k").expect("key_press: 16") == keycode  {
                    debug!("Snap up");
                    wm.shift_window(wm.focus_w, Direction::North);
                    wm.center_cursor(w);
                    return;
                }
                if xlib.str_to_keycode("c").expect("key_press: \"c\"") == keycode {
                    debug!("Center window");
                    wm.place_window(wm.focus_w);
                    wm.center_cursor(wm.focus_w);
                    return;
                }
                if xlib.str_to_keycode("d").expect("key_press: \"d\"") == keycode {
                    debug!("dmenu_run");
                    spawn_process("dmenu_recency", vec![]);
                    return;
                }
                if ws_keys.contains(&keycode) {
                    let ws_num = keycode_to_ws(keycode);
                    wm.set_current_ws(ws_num);
                }
            }
        },
        None if w == xlib.get_root() => {
            let keycode = keycode as u8;
            if mod_not_shift {
                if xlib.str_to_keycode("Return").expect("key_press: 17") == keycode {
                    spawn_process(CONFIG.term.as_str(), vec![]);
                }
                if xlib.str_to_keycode("d").expect("key_press: \"d\"") == keycode {
                    debug!("dmenu_run");
                    spawn_process("dmenu_recency", vec![]);
                    return;
                }

                match ws_keys.contains(&keycode) {
                    true  => {
                        let ws_num = keycode_to_ws(keycode);
                        wm.set_current_ws(ws_num);
                    },
                    _ => {}
                }
            }
            if mod_and_shift {
                if xlib.str_to_keycode("e").expect("key_press: 18") == keycode {
                    xlib.exit();
                }
            }

        }
        None => { return; }
    };
}

fn keycode_to_ws(keycode: u8) -> u32 {
    ((keycode - 10) % 10) as u32
}

fn spawn_process(bin_name: &str, args: Vec<&str>) {
    let mut cmd = Command::new(bin_name);
    args
        .into_iter()
        .for_each(|arg| {cmd.arg(arg);});
    let _ = cmd.spawn();
}
