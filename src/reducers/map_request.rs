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
    reducer::*
};


impl Reducer<action::MapRequest> for State {
    fn reduce(&mut self, action: action::MapRequest) {

        match self.windows.get_mut(&action.win) {
            Some(w) => w.handle_state = HandleState::Map,
            None => {
                if self.lib.should_be_managed(action.win) {
                    let pos = Position{x: 200, y: 300};
                    let size = Size{ width: 600, height: 400 };
                    let mut ww = WindowWrapper::new(action.win, Rect::new(pos, size));
                    self.windows.insert(action.win, ww);
                }
            }
        }


    }
}

