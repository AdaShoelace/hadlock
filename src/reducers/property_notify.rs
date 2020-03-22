#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*},
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::PropertyNotify> for State {
    fn reduce(&mut self, action: action::PropertyNotify) {
        let name = self.lib.atom_name(action.atom).unwrap();
        //debug!("PropertyNotify, Atom name: {}", name);
    }
}
