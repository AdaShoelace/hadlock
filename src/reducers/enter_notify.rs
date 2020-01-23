#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, HandleState},
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

impl Reducer<action::EnterNotify> for State {
    fn reduce(&mut self, action: action::EnterNotify) {
        //debug!("EnterNotify");
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
