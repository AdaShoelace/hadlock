use {
    crate::{
        layout::LayoutTag, state::State, wm, xlibwrapper::action, xlibwrapper::xlibmodels::Window,
    },
    reducer::*,
};

impl Reducer<action::Destroy> for State {
    fn reduce(&mut self, action: action::Destroy) {
        debug!("DestroyNotify action.win: 0x{:x}", action.win);
        if action.win == self.lib.get_root() {
            debug!("is root in destroy");
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("DestroyNotify - get_mut");
        debug!(
            "windows in current mon, ws: {:x?}",
            mon.get_current_ws()
                .unwrap()
                .clients
                .keys()
                .collect::<Vec<&Window>>()
        );
        if mon.contains_window(action.win) {
            let next_focus = mon
                .get_previous(action.win)
                .map(|ww| ww.window())
                .unwrap_or(
                    mon.get_newest()
                        .map(|(win, _)| *win)
                        .unwrap_or(self.lib.get_root()),
                );
            debug!(
                "in destroy - action.win: {}, next_focus: {}",
                action.win, next_focus
            );
            mon.remove_window(action.win);
            if let Some(ws) = mon.get_current_ws_mut() {
                ws.focus_w = next_focus;
            }
            self.focus_w = next_focus;
            if mon.get_current_layout() != LayoutTag::Floating {
                wm::reorder(self);
            }
        } else {
            debug!("destroy_notify - does not contain action.win");
        }
    }
}
