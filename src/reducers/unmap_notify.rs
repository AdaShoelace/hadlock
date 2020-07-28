use crate::{layout::LayoutTag, models::WindowState, state::State, wm, xlibwrapper::action};
use reducer::*;

impl Reducer<action::UnmapNotify> for State {
    fn reduce(&mut self, action: action::UnmapNotify) {
        debug!("UnmapNotify");
        let mon_id = wm::get_mon_by_window(&self, action.win).unwrap_or(self.current_monitor);
        let mon = self
            .monitors
            .get_mut(&mon_id)
            .expect("No monitor was found?!");
        if let Some(client) = mon.get_client(action.win) {
            debug!("unmap - contains action.win: 0x{:x}", action.win);
            if client.current_state != WindowState::Destroy {
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
                if let Some(ws) = mon.get_current_ws_mut() {
                    ws.focus_w = next_focus;
                }
                self.focus_w = next_focus;
                mon.remove_window(action.win);
            }
        }
        if mon.get_current_layout() != LayoutTag::Floating {
            wm::reorder(self);
        }
    }
}
