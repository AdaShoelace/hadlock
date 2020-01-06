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
        xlibwrapper::masks::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*
};


impl Reducer<action::ButtonRelease> for State {
    fn reduce(&mut self, _action: action::ButtonRelease) {
        debug!("ButtonRelease");
    }
}

