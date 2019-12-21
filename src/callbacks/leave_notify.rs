use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    std::rc::Rc
};


pub fn leave_notify(xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let w = match action {
        Action::LeaveNotify{win} => win,
        _ => { return; }
    };

    // this check is an ugly hack to not crash when decorations window gets destroyed before
    // client and client recieves an "OnLeave"-event
    if !wm.current_monitor().expect("leave_notify: current_monitor 1").contains_window(w) || w == xlib.get_root() {
        return;
    }

    wm.unset_focus(w);
}
