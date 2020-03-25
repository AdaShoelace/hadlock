#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{
            internal_action::InternalAction, rect::*, window_type::WindowType, windowwrapper::*,
        },
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
