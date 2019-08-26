use crate::windowmanager::WindowManager;
use crate::xlibwrapper::core::XlibWrapper;
use crate::xlibwrapper::event::Event;
use std::rc::Rc;

pub mod map_request;
pub mod configure_request;
pub mod motion_notify;
pub mod leave_notify;
pub mod enter_notify;
pub mod button_press;
pub mod button_release;
pub mod key_press;
pub mod key_release;
pub mod destroy_window;
pub mod expose;

pub type Callback = Box<fn(Rc<XlibWrapper>, &mut WindowManager, Event)>;

