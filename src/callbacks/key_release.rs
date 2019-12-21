use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    std::rc::Rc,
};

pub fn key_release(_xlib: Rc<XlibWrapper>, _wm: &mut WindowManager, action: Action) {
    let (_w, _state, _keycode) =
        match action {
            Action::KeyRelease{win, state, keycode} => (win, state, keycode),
            _ => { return; }
        };
}
