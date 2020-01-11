#![allow(unused_imports)]
use {
    crate::{
        wm,
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
            monitor::Monitor,
            HandleState
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*,
    std::cell::RefCell,
};


impl Reducer<action::ConfigurationRequest> for State {
    fn reduce(&mut self, action: action::ConfigurationRequest) {
        debug!("ConfigurationRequest for window: {} - {:?}", action.win, action.win_changes);
        let mon = self.monitors.get_mut(&self.current_monitor).expect("ConfigurationRequest - monitor - get_mut");


        if mon.contains_window(action.win) {
            if action.value_mask & (xlib::CWX | xlib::CWY) as u64 == (xlib::CWX | xlib::CWY) as u64 { return }
            let ww = mon.remove_window(action.win);
            self.lib.configure_window(
                action.win,
                action.value_mask as i64,
                WindowChanges {
                    x: ww.window_rect.get_position().x,
                    y: ww.window_rect.get_position().y,
                    width: ww.window_rect.get_size().width,
                    height: ww.window_rect.get_size().height,
                    ..action.win_changes
            }
            );
            mon.add_window(ww.window(), ww);
        } else {
            self.lib.configure_window(action.win, action.value_mask as i64, action.win_changes);
        }
    }
}

