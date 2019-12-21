use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::*;
use crate::xlibwrapper::event::*;
use std::rc::Rc;

pub fn enter_notify(_xlib: Rc<XlibWrapper>, wm: &mut WindowManager, event: Event) {

    let (w, _sub_w) = match event {
        Event::EnterNotify{win, sub_win} => (win, sub_win),
        _ => { return; }
    };


    if !wm.current_monitor().expect("enter_notify: current_monitor 1").contains_window(w) && w != wm.lib.get_root() {
        println!("Calling window {} not in client list", w);
        return;
    }
    wm.unset_focus(wm.focus_w);
    wm.set_focus(w);
}
