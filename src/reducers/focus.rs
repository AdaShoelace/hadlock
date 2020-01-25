#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{
            monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, HandleState,
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

impl Reducer<action::Focus> for State {
    fn reduce(&mut self, action: action::Focus) {
        if action.win == self.lib.get_root() {
            return;
        }

        match wm::get_mon_by_window(self, action.win) {
            Some(mon) => {
                let (class, something) = self.lib.get_class_hint(action.win);
                debug!(
                    "Sending clients top window is not root. Win ({},{}) is in mon {}",
                    class, something, mon
                );
                let curr_mon = self
                    .monitors
                    .get_mut(&self.current_monitor)
                    .expect("Focus - get_monitor");

                //unset focus
                if self.focus_w != self.lib.get_root() {
                    let mut old_focus = match curr_mon.remove_window(self.focus_w) {
                        Some(win) => win,
                        None => return,
                    };
                    old_focus = WindowWrapper {
                        handle_state: HandleState::Unfocus.into(),
                        ..old_focus
                    };
                    curr_mon.add_window(self.focus_w, old_focus);
                }

                //set focus
                self.focus_w = action.win;
                let mut new_focus = match curr_mon.remove_window(self.focus_w) {
                    Some(win) => win,
                    None => return,
                };
                new_focus = WindowWrapper {
                    handle_state: HandleState::Focus.into(),
                    ..new_focus
                };
                curr_mon.add_window(self.focus_w, new_focus);
            }
            None => (),
        }
    }
}
