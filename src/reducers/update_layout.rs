#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{
            monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, HandleState, WindowState
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
        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("UpdateLayout - reducer - monitor - get_mut");
        if self.focus_w == self.lib.get_root() {
            return;
        }
        let windows = mon.place_window(self.focus_w);
        for win in windows.into_iter() {
            let ww = mon.remove_window(win.0).expect("to shit it went!");
            let new_ww = WindowWrapper {
                window_rect: win.1,
                previous_state: ww.current_state,
                current_state: WindowState::Free,
                handle_state: HandleState::Center.into(),
                ..ww
            };
            mon.add_window(new_ww.window(), new_ww);
        }
    }
}
