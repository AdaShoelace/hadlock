#![allow(unused_imports)]
use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*,
    std::cell::RefCell,
};


impl Reducer<action::DestroyNotify> for State {
    fn reduce(&mut self, action: action::DestroyNotify) {
        //debug!("DestroyNotify");
        if action.win == self.lib.get_root() { return }

        let mon = self.monitors.get_mut(&self.current_monitor).expect("DestroyNotify - get_mut");
        if mon.contains_window(action.win) {
            mon.remove_window(action.win);
        }
    }
}

