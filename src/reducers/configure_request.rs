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
        debug!("ConfigurationRequest");
        self.lib.configure_window(action.win, action.value_mask as i64, action.win_changes);
    }
}

