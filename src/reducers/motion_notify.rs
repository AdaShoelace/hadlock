use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::WindowState,
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::masks::*,
        xlibwrapper::util::*,
    },
    reducer::Reducer,
};

impl Reducer<action::MotionNotify> for State {
    fn reduce(&mut self, action: action::MotionNotify) {
        let actual_mon = wm::get_monitor_by_point(self, action.x_root, action.y_root);
        let old_mon = self.current_monitor;

        if self.current_monitor != actual_mon {
            if let Some(mon) = self.monitors.get_mut(&old_mon) {
                mon.mouse_follow.replace(false);
            }

            self.current_monitor = actual_mon;

            let mon = self
                .monitors
                .get_mut(&self.current_monitor)
                .expect("MotionNotify - monitor - get_mut - change handle state");
            mon.mouse_follow.replace(false);
        }

        let layout = self
            .monitors
            .get(&self.current_monitor)
            .expect("MotionNotify - monitor - get - check layout")
            .get_current_layout();

        let is_trans = match self
            .monitors
            .get(&self.current_monitor)
            .expect("MotionNotify - monitor - get - action.win is_trans")
            .get_client(action.win)
        {
            Some(client) => client.is_trans,
            None => return,
        };

        if layout != LayoutTag::Floating && !is_trans {
            return;
        }

        let new_pos = calculcate_destination(self, &action);

        if (action.state & (Button1Mask | CONFIG.mod_key)) == Button1Mask | CONFIG.mod_key {
            if action.win == self.lib.get_root() {
                return;
            }

            let ww = self
                .monitors
                .get_mut(&old_mon)
                .expect("MotionNotify - old_mon - get_mut")
                .remove_window(action.win)
                .expect("Trying to remove window in motion_notify");

            self.monitors
                .get_mut(&actual_mon)
                .expect("MotionNotify - old_mon - get_mut")
                .add_window(action.win, ww);

            let (pos, _) = self
                .monitors
                .get_mut(&self.current_monitor)
                .expect("MotionNotify - monitor - get_mut")
                .move_window(action.win, new_pos.x, new_pos.y);
            let w = self
                .monitors
                .get_mut(&self.current_monitor)
                .expect("MotionNotify - monitor - get_mut")
                .get_client_mut(action.win)
                .expect("motion_notify some window");
            if w.current_state != WindowState::Monocle {
                w.save_restore_position();
                w.set_position(pos);
                w.set_window_state(WindowState::Free);
            }
            return;
        }
    }
}

fn calculcate_destination(state: &State, action: &action::MotionNotify) -> Position {
    let drag_pos = Position {
        x: action.x_root,
        y: action.y_root,
    };
    let (delta_x, delta_y) = (
        drag_pos.x - state.drag_start_pos.0,
        drag_pos.y - state.drag_start_pos.1,
    );
    Position {
        x: state.drag_start_frame_pos.0 + delta_x,
        y: state.drag_start_frame_pos.1 + delta_y,
    }
}
