#![allow(unused_imports)]
use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::WindowWrapper,
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*
};


impl Reducer<action::UnknownEvent> for State {
    fn reduce(&mut self, _action: action::UnknownEvent) {
    }
}

