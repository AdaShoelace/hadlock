#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::*,
        models::{rect::*, window_type::WindowType, windowwrapper::*},
        state::State,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::LeaveNotify> for State {
    fn reduce(&mut self, action: action::LeaveNotify) {
        let _mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("LeaveNotify - monitor - get_mut");

        debug!("LeaveNotify for: 0x{:x}", action.win);
    }
}
