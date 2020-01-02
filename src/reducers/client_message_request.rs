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


impl Reducer<action::ClientMessageRequest> for State {
    fn reduce(&mut self, action: action::ClientMessageRequest) {
        debug!("ClientMessageRequest");
        let data_zero = *action.data.get(0).expect("client_message_request: cleanupt");
        let data_one = *action.data.get(1).expect("client_message_request: cleanupt");
        let data_two = *action.data.get(2).expect("client_message_request: cleanupt");

        if action.message_type == self.lib.xatom.NetWMState
            && (data_one == self.lib.xatom.NetWMStateFullscreen as i64
                || data_two == self.lib.xatom.NetWMStateFullscreen as i64) {
                debug!("Actually fullscreen");
                let set_fullscreen = data_zero == 1;
                let toggle_fullscreen = data_zero == 2;


                let mut states = self.lib.get_window_states_atoms(action.win);

                //determine what to change the state to
                let fullscreen = if toggle_fullscreen {
                    !states.contains(&self.lib.xatom.NetWMStateFullscreen)
                } else {
                    set_fullscreen
                };

                //update the list of states
                if fullscreen {
                    states.push(self.lib.xatom.NetWMStateFullscreen);
                } else {
                    states.retain(|x| x != &self.lib.xatom.NetWMStateFullscreen);
                }
                states.sort();
                states.dedup();

                //set the windows state
                self.lib.set_window_states_atoms(action.win, states);
                let mon = self.monitors.get_mut(&self.current_monitor).expect("ClientMessageRequest - monitor - get_mut");
                let old_ww = mon.remove_window(action.win);
                let new_ww = wm::toggle_maximize(self, old_ww);
                self
                    .monitors
                    .get_mut(&self.current_monitor)
                    .expect("ClientMessageRequest - monitor - get_mut")
                    .add_window(action.win, new_ww);
            }
    }
}

