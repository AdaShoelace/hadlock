#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*},
        state::State,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::masks::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::rc::Rc,
};

impl Reducer<action::ButtonRelease> for State {
    fn reduce(&mut self, _action: action::ButtonRelease) {
        debug!("ButtonRelease");
    }
}
