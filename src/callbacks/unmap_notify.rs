use crate::windowmanager::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::core::*;
use std::rc::Rc;

pub fn unmap_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event {
        Event::UnmapNotify{win} => win,
        _ => { return; }
    };

    wm.hide_decoration(w);
    //wm.lib.unmap_window(w);
}

