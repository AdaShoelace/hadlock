use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    x11_dl::xlib,
    std::rc::Rc,
};


pub fn configure_request(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let (w, window_changes, value_mask) =
        match action {
            Action::ConfigurationRequest{win, win_changes, value_mask} => (win, win_changes, value_mask),
            _ => { return; }
        };

    if value_mask & (xlib::CWX | xlib::CWY) as u64 == (xlib::CWX | xlib::CWY) as u64 { return }

    if wm.current_monitor().expect("configure_request: current_monitor 1").contains_window(w) {
        return;
    }

    xlib.configure_window(
        w,
        value_mask as i64,
        window_changes
    );
}
