#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::{rect::*, window_type::WindowType, windowwrapper::*, WindowState},
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
        if action.win == self.lib.get_root() {
            return;
        }
        if (action.state & (Button1Mask | CONFIG.mod_key)) == Button1Mask | CONFIG.mod_key {
            if let Some(dir) = self
                .monitors
                .get_mut(&self.current_monitor)
                .expect("Should have a current mon...")
                .inside_snapping_region(self.lib.pointer_pos(self.lib.get_root()))
            {
                wm::shift_window(self, dir);
            }
        }
        let old_mon_id =
            wm::get_mon_by_window(&self, action.win).expect("It has to come from some mon?");

        if old_mon_id != self.current_monitor {
            let old_mon = self
                .monitors
                .get_mut(&old_mon_id)
                .expect("Apparently this monitor does not exist");
            if old_mon.get_current_layout() != LayoutTag::Floating {
                return;
            }

            let action_ww = old_mon
                .remove_window(action.win)
                .expect("Window must be in this monitor");
            let current_mon = self.monitors.get_mut(&self.current_monitor).expect("How!?");
            let windows = current_mon.place_window(action.win);
            let current_state =
                if windows.len() == 1 && current_mon.get_current_layout() != LayoutTag::Floating {
                    WindowState::Maximized
                } else {
                    WindowState::Free
                };
            for (win, rect) in windows {
                if win == action.win {
                    current_mon.add_window(
                        win,
                        WindowWrapper {
                            window_rect: rect,
                            current_state,
                            ..action_ww
                        },
                    );
                } else {
                    current_mon.swap_window(win, |_mon, ww| WindowWrapper {
                        window_rect: rect,
                        current_state,
                        ..ww
                    });
                }
            }
            current_mon.get_current_ws_mut().unwrap().focus_w = action.win;
            self.focus_w = action.win;
        }
    }
}
