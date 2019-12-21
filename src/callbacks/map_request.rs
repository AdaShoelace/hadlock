use {
    crate::windowmanager::*,
    crate::xlibwrapper::action::Action,
    crate::xlibwrapper::core::*,
    std::rc::Rc,
};

pub fn map_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let w = match action {
        Action::MapRequest{win} => win,
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

