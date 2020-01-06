#![allow(unused_imports)]
use {
    crate::{
        wm,
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
            monitor::Monitor
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


impl Reducer<action::PropertyNotify> for State {
    fn reduce(&mut self, _action: action::PropertyNotify) {
        debug!("PropertyNotify");
    }
}

