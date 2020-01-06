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


impl Reducer<action::UnmapNotify> for State {
    fn reduce(&mut self, _action: action::UnmapNotify) {
        debug!("UnmapNotify");
    }
}

