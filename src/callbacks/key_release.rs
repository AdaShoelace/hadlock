use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;

pub fn key_release(_xlib: Rc<XlibWrapper>, _wm: &mut WindowManager, event: Event) {
    //println!("keyrelease registered");
    let (_w, _state, _keycode) =
        match event {
            Event::KeyRelease{win, state, keycode} => (win, state, keycode),
            _ => { return; }
        };
}
