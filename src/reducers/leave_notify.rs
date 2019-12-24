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


impl Reducer<action::LeaveNotify> for State {
    fn reduce(&mut self, action: action::LeaveNotify) {
        debug!("LeaveNotify");
        if let Some(w) = self.windows.get_mut(&action.win) {
                w.handle_state = HandleState::Unfocus.into();
        }
    }
}

