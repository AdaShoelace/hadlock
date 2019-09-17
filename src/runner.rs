use std::collections::HashMap;
use super::windowmanager::WindowManager;
use super::callbacks::*;
use super::xlibwrapper::event::*;
use super::xlibwrapper::core::*;
use std::rc::Rc;
use std::sync::mpsc;

pub struct Runner {
    call_table: HashMap<EventType, Callback>,
    lib: Rc<XlibWrapper>,
    wm: WindowManager
}

impl Runner {
    pub fn new(lib: Rc<XlibWrapper>, wm: WindowManager) -> Self {
        let mut ret = Self {
            call_table: HashMap::new(),
            lib,
            wm
        };

        ret.call_table.insert(EventType::MapRequest, Box::new(map_request::map_request));
        ret.call_table.insert(EventType::ConfigurationRequest, Box::new(configure_request::configure_request));
        ret.call_table.insert(EventType::MotionNotify, Box::new(motion_notify::motion_notify));
        ret.call_table.insert(EventType::DestroyWindow, Box::new(destroy_window::destroy_window));
        ret.call_table.insert(EventType::Expose, Box::new(expose::expose));
        ret.call_table.insert(EventType::LeaveNotify, Box::new(leave_notify::leave_notify));
        ret.call_table.insert(EventType::EnterNotify, Box::new(enter_notify::enter_notify));
        ret.call_table.insert(EventType::ButtonPress, Box::new(button_press::button_press));
        ret.call_table.insert(EventType::KeyPress, Box::new(key_press::key_press));
        ret.call_table.insert(EventType::KeyRelease, Box::new(key_release::key_release));
        ret.call_table.insert(EventType::ButtonRelease, Box::new(button_release::button_release));


        ret
    }


    pub fn run(&mut self, tx: mpsc::Sender<bool>) {

        self.lib.grab_server();
        let _ = self.lib.get_top_level_windows()
            .iter()
            .map(|w| {
                self.wm.setup_window(*w)
            });
        self.lib.ungrab_server();
        
        tx.send(true);

        loop {
            let event = self.lib.next_event();
            // println!("{:?}", &event);

            match self.call_table.get(&event.event_type) {
                Some(func) => func(self.lib.clone(), &mut self.wm, event),
                None => { continue; }
            }
        }
    }
}
