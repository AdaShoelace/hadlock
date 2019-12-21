use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;


pub fn destroy_window(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event {
        Event::DestroyNotify{win} => win,
        _ => { return; }
    };

    wm.kill_window(w);
}
