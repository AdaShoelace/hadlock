use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
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


impl Reducer<action::EnterNotify> for State {
    fn reduce(&mut self, action: action::EnterNotify) {
        //debug!("EnterNotify");
        self.focus_w = action.win;
        if let Some(w) = self.monitors.get_mut(&self.current_monitor).expect("EnterNotify - monitor - get_mut").get_client_mut(action.win) {
            w.handle_state = HandleState::Focus.into();
        }
    }
}

