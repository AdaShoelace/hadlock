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

impl Reducer<action::ConfigurationRequest> for State {
    fn reduce(&mut self, action: action::ConfigurationRequest) {
        debug!(
            "ConfigurationRequest for window: {} - {:?}",
            action.win, action.win_changes
        );

        let mon_id = match wm::get_mon_by_window(&self, action.win) {
            Some(mon_id) => mon_id,
            None => {
                self.lib
                    .configure_window(action.win, action.value_mask as i64, action.win_changes);
                return;
            }
        };

        let mon = self
            .monitors
            .get_mut(&mon_id)
            .expect("ConfigurationRequest - monitor - get_mut");

        if mon.contains_window(action.win) {
            let ws = mon.get_ws_by_window(action.win).unwrap();
            let ww = mon
                .remove_window_non_current(action.win, ws)
                .expect("ConfigurationRequest - monitor - remove_window");
            self.lib.configure_window(
                action.win,
                action.value_mask as i64,
                WindowChanges {
                    x: ww.window_rect.get_position().x,
                    y: ww.window_rect.get_position().y,
                    width: ww.window_rect.get_size().width,
                    height: ww.window_rect.get_size().height,
                    ..action.win_changes
                },
            );
            mon.add_window_non_current(ww.window(), ww, ws);
        }
    }
}
