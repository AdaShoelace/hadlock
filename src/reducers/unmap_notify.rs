use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, internal_action::InternalAction},
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::rc::Rc,
};

impl Reducer<action::UnmapNotify> for State {
    fn reduce(&mut self, action: action::UnmapNotify) {
        //debug!("UnmapNotify");
        let mon_id = wm::get_mon_by_window(&self, action.win).unwrap_or(self.current_monitor);
        let mon = self.monitors.get_mut(&mon_id).expect("No monitor was found?!");
        mon.remove_window(action.win);
        let _ = self.tx.send(InternalAction::UpdateLayout);
    }
}
