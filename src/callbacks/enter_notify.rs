use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::core::*,
    crate::xlibwrapper::action::Action,
    std::rc::Rc,
};

pub fn enter_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, action: Action) {

    let (w, _sub_w) = match action {
        Action::EnterNotify{win, sub_win} => (win, sub_win),
        _ => { return; }
    };


    if !wm.current_monitor().expect("enter_notify: current_monitor 1").contains_window(w) && w != wm.lib.get_root() {
        println!("Calling window {} not in client list", w);
        return;
    }
    wm.unset_focus(wm.focus_w);
    wm.set_focus(w);
}
