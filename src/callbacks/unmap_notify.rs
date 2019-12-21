use {
    crate::windowmanager::*,
    crate::xlibwrapper::action::Action,
    crate::xlibwrapper::core::*,
    std::rc::Rc,
};

pub fn unmap_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let w = match action {
        Action::UnmapNotify{win} => win,
        _ => { return; }
    };

    wm.hide_decoration(w);
    //wm.lib.unmap_window(w);
}

