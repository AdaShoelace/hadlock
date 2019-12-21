use crate::windowmanager::*;
use crate::xlibwrapper::event::*;
use crate::xlibwrapper::core::*;
use std::rc::Rc;

pub fn map_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let w = match event{
        Event::MapRequest{win} => win,
        _ => { return; }
    };

    let (class, name) = xlib.get_class_hint(w);
    debug!("Mapped class: {}, name: {}", class, name);
    if wm.client_exist(w) {
        wm.show_client(w);
    } else {
        wm.setup_window(w);
    }
    xlib.map_window(w);
}

