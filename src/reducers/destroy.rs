use {
    crate::{
        layout::LayoutTag,
        state::State,
        wm,
        xlibwrapper::action,
    },
    reducer::*,
};

impl Reducer<action::Destroy> for State {
    fn reduce(&mut self, action: action::Destroy) {
        debug!("DestroyNotify");
        if action.win == self.lib.get_root() {
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("DestroyNotify - get_mut");
        if mon.contains_window(action.win) {
            mon.remove_window(action.win);
            if mon.get_current_layout() != LayoutTag::Floating {
                wm::reorder(self);
            }
        }
    }
}
