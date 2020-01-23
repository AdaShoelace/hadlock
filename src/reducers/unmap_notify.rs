#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*},
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

impl Reducer<action::UnmapNotify> for State {
    fn reduce(&mut self, _action: action::UnmapNotify) {
        //debug!("UnmapNotify");
        //self.lib.unmap_window(action.win);
    }
}
