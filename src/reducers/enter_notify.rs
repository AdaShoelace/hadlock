#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::*,
        models::{rect::*, window_type::WindowType, windowwrapper::*, HandleState},
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

impl Reducer<action::EnterNotify> for State {
    fn reduce(&mut self, action: action::EnterNotify) {
        //debug!("EnterNotify");
        if self.ws_switch {
            self.ws_switch = false;
            return
        }
        let window_mon = wm::get_mon_by_window(&self, action.win);
        if let Some(mon_id) = window_mon {
            if mon_id != self.current_monitor {
                self.current_monitor = mon_id;
                self.monitors
                    .get_mut(&self.current_monitor)
                    .unwrap()
                    .handle_state
                    .replace(HandleState::Focus.into());
            }
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("EnterNotify - monitor - get_mut");

        if action.win == self.lib.get_root()
            && mon.get_current_layout() != Some(LayoutTag::Floating)
        {
            return;
        }

        if let Some(w) = mon.get_client_mut(self.focus_w) {
            w.handle_state = HandleState::Unfocus.into();
        }

        self.focus_w = action.win;
        if let Some(w) = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("EnterNotify - monitor - get_mut")
            .get_client_mut(action.win)
        {
            w.handle_state = HandleState::Focus.into();
        }
        self
            .monitors
            .get_mut(&self.current_monitor)
            .unwrap()
            .get_current_ws_mut()
            .unwrap()
            .focus_w = self.focus_w;
    }
}
