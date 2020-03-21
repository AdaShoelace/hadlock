#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, internal_action::InternalAction},
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

impl Reducer<action::Destroy> for State {
    fn reduce(&mut self, action: action::Destroy) {
        debug!("DestroyNotify");
        if action.win == self.lib.get_root() {
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("DestroyNotify - get_mut");
        if mon.contains_window(action.win) {
            mon.remove_window(action.win);
            let _ = self.tx.send(InternalAction::UpdateLayout);
        }
    }
}
