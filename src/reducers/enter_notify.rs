#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, HandleState},
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::EnterNotify> for State {
    fn reduce(&mut self, action: action::EnterNotify) {
        //debug!("EnterNotify");
        let window_mon = wm::get_mon_by_window(&self, action.win);
        if let Some(mon_id) = window_mon {
            if mon_id != self.current_monitor {
                self.current_monitor = mon_id;
                self.monitors
                    .get_mut(&self.current_monitor)
                    .unwrap()
                    .handle_state
                    .replace(HandleState::Focus);
            }
        }

        self.focus_w = action.win;
        if let Some(w) = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("EnterNotify - monitor - get_mut")
            .get_client_mut(action.win)
        {
            w.handle_state = HandleState::Focus.into();
        }
    }
}
