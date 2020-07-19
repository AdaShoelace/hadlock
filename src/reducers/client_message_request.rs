#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, internal_action::InternalAction},
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

#[allow(clippy::collapsible_if)]
impl Reducer<action::ClientMessageRequest> for State {
    // Full credit for this solution goes to lex148
    fn reduce(&mut self, action: action::ClientMessageRequest) {
        let _name = self.lib.xatom().get_name(action.message_type);

        // debug!("client message: {}", name);

        let data_zero = *action
            .data
            .get(0)
            .expect("client_message_request: cleanupt");
        // debug!("data_zero: {:?}", data_zero);
        let data_one = *action
            .data
            .get(1)
            .expect("client_message_request: cleanupt");
        // debug!("data_one: {:?}", data_one);
        let data_two = *action
            .data
            .get(2)
            .expect("client_message_request: cleanupt");
        // debug!("data_two: {:?}", data_two);

        if action.message_type == self.lib.xatom().NetWMState {
            if data_zero == self.lib.xatom().NetWMStateHidden as i64
                || data_one == self.lib.xatom().NetWMStateHidden as i64
                || data_two == self.lib.xatom().NetWMStateHidden as i64
            {
                debug!("window: 0x{:x} sent a hidden message", action.win);
                let mon_id =
                    wm::get_mon_by_window(&self, action.win).unwrap_or(self.current_monitor);
                let mon = self
                    .monitors
                    .get_mut(&mon_id)
                    .expect("No monitor was found?!");
                mon.remove_window(action.win);
                let _ = self.tx.send(InternalAction::UpdateLayout);
            }
        }

        if action.message_type == self.lib.xatom().NetCurrentDesktop {
            wm::set_current_ws(self, data_zero as u32);
        }

        if action.message_type == self.lib.xatom().NetWMState
            && (data_one == self.lib.xatom().NetWMStateFullscreen as i64
                || data_two == self.lib.xatom().NetWMStateFullscreen as i64)
        {
            //debug!("Actually fullscreen");
            let set_fullscreen = data_zero == 1;
            let toggle_fullscreen = data_zero == 2;

            let mut states = self.lib.get_window_states_atoms(action.win);

            //determine what to change the state to
            let fullscreen = if toggle_fullscreen {
                !states.contains(&self.lib.xatom().NetWMStateFullscreen)
            } else {
                set_fullscreen
            };

            //update the list of states
            if fullscreen {
                states.push(self.lib.xatom().NetWMStateFullscreen);
            } else {
                states.retain(|x| x != &self.lib.xatom().NetWMStateFullscreen);
            }
            states.sort();
            states.dedup();

            //set the windows state
            self.lib.set_window_states_atoms(action.win, states);
            let mon = self
                .monitors
                .get_mut(&self.current_monitor)
                .expect("ClientMessageRequest - monitor - get_mut");
            mon.swap_window(action.win, |mon, ww| {
                let ww = wm::toggle_monocle(mon, ww);
                debug!("client_message_request fullscreen: {:#?}", ww);
                ww
            });
        }
    }
}
