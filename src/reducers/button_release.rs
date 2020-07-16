#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::{rect::*, window_type::WindowType, windowwrapper::*},
        state::State,
        wm,
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
    fn reduce(&mut self, action: action::ButtonRelease) {
        let old_mon_id =
            wm::get_mon_by_window(&self, action.win).expect("It has to come from some mon?");

        if old_mon_id != self.current_monitor {
            let old_mon = self
                .monitors
                .get_mut(&old_mon_id)
                .expect("Apparently this monitor does not exist");

            let action_ww = old_mon
                .remove_window(action.win)
                .expect("Window must be in this monitor");

            let current_mon = self.monitors.get_mut(&self.current_monitor).expect("How!?");
            let windows = current_mon.place_window(action.win);
            for (win, rect) in windows {
                if win == action.win {
                    current_mon.add_window(
                        win,
                        WindowWrapper {
                            window_rect: rect,
                            ..action_ww
                        },
                    );
                } else {
                    current_mon.swap_window(win, |_mon, ww| WindowWrapper {
                        window_rect: rect,
                        ..ww
                    });
                }
            }
        }
    }
}
