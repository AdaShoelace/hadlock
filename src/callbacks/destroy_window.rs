use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    std::rc::Rc,
};


pub fn destroy_window(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let w = match action {
        Action::DestroyNotify{win} => win,
        _ => { return; }
    };

    wm.kill_window(w);
}
