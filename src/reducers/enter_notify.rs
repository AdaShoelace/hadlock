use {
    crate::{layout::*, state::State, wm, xlibwrapper::action},
    reducer::*,
};

impl Reducer<action::EnterNotify> for State {
    fn reduce(&mut self, action: action::EnterNotify) {
        // debug!("EnterNotify");
        if self.latest_cursor_pos == self.lib.pointer_pos(self.lib.get_root()) {
            return;
        }

        self.latest_cursor_pos = self.lib.pointer_pos(self.lib.get_root());

        let window_mon = wm::get_mon_by_window(&self, action.win);
        if let Some(mon_id) = window_mon {
            if mon_id != self.current_monitor {
                self.current_monitor = mon_id;
            }
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("EnterNotify - monitor - get_mut");

        if action.win == self.lib.get_root() && mon.get_current_layout() != LayoutTag::Floating {
            return;
        }

        self.focus_w = action.win;
        self.monitors
            .get_mut(&self.current_monitor)
            .unwrap()
            .get_current_ws_mut()
            .unwrap()
            .focus_w = self.focus_w;
    }
}
