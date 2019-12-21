use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    std::rc::Rc,
};

pub fn button_release(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let (window, _sub_win, _x_root, _y_root, button, _state) =
        match action {
            Action::ButtonRelease{win, sub_win, x_root, y_root, button, state} => (win, sub_win, x_root, y_root, button, state),
            _ => { return; }
        };


    if !wm.current_monitor().expect("button_release: current_monitor 1").contains_window(window) || window == xlib.get_root() {
        return
    }

    debug!("Focused window: {}", wm.focus_w);

    let _ww = wm.current_monitor().expect("button_release: current_monitor 2").get_client(window).expect("ButtonPressed: No such window in client list");
}
