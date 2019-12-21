use {
    crate::windowmanager::WindowManager,
    crate::xlibwrapper::{
        core::*,
        action::Action,
    },
    std::rc::Rc,
};


pub fn property_notify(_xlib: Rc<XlibWrapper>, _wm: &mut WindowManager, action: Action) {

    let (_window, _atom) = match action {
        Action::PropertyNotify{win, atom} => (win, atom),
        _ => { return; }
    };

}
