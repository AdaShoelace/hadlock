use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::masks::*;
use std::rc::Rc;

pub fn key_release(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {
    //println!("keyrelease registered");
    let (w, state, keycode) =
        match event {
            Event {
                event_type: EventType::KeyRelease,
                payload: Some(EventPayload::KeyRelease(w, state, keycode))
            } => (w, state, keycode),
            _ => { return; }
        };

    match wm.clients.get(&wm.focus_w) {
        Some(_ww) => {
            let keycode = keycode as u8;

            if (state & (Mod4Mask | Shift)) == Mod4Mask | Shift {
                //println!("For some godforsaken reason we are here to...");
                if xlib.str_to_keycode("q").unwrap() == keycode {
                    wm.kill_window(w);
                }
            }
        },

        None => { return; }
    };
}
