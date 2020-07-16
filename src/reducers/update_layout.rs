#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::{
            monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, WindowState,
        },
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

impl Reducer<action::UpdateLayout> for State {
    fn reduce(&mut self, _action: action::UpdateLayout) {
        if self
            .monitors
            .get(&self.current_monitor)
            .expect("")
            .get_current_ws()
            .expect("")
            .get_current_layout()
            != LayoutTag::Floating
        {
            wm::reorder(self);
        }
    }
}
