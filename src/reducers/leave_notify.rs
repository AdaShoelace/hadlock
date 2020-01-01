use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
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


impl Reducer<action::LeaveNotify> for State {
    fn reduce(&mut self, action: action::LeaveNotify) {
        //debug!("LeaveNotify");
        if let Some(w) = self.monitors.get_mut(&self.current_monitor).expect("LeaveNotify - monitor - get_mut").get_client_mut(action.win) {
                w.handle_state = HandleState::Unfocus.into();
        }
    }
}

