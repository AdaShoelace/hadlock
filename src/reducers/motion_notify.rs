#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, HandleState, WindowState},
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::masks::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::MotionNotify> for State {
    fn reduce(&mut self, action: action::MotionNotify) {
        debug!("MotionNotify");
        let actual_mon = wm::get_monitor_by_point(self, action.x_root, action.y_root);
        let old_mon = self.current_monitor;

        if self.current_monitor != actual_mon {
            self.current_monitor = actual_mon;
            self.monitors
                .get(&self.current_monitor)
                .expect("MotionNotify - monitor - get_mut - change handle state")
                .handle_state
                .replace(HandleState::Focus);
        }



        let drag_pos = Position {
            x: action.x_root,
            y: action.y_root,
        };
        let (delta_x, delta_y) = (
            drag_pos.x - self.drag_start_pos.0,
            drag_pos.y - self.drag_start_pos.1,
        );
        let dest_pos = Position {
            x: self.drag_start_frame_pos.0 + delta_x,
            y: self.drag_start_frame_pos.1 + delta_y,
        };

        if (action.state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
            if action.win == self.lib.get_root() { return }

            if action.win != self.lib.get_root() {
                let ww = self.monitors
                    .get_mut(&old_mon)
                    .expect("MotionNotify - old_mon - get_mut")
                    .remove_window(action.win);

                let _ = self.monitors
                    .get_mut(&actual_mon)
                    .expect("MotionNotify - old_mon - get_mut")
                    .add_window(action.win, ww);
            }
            let new_pos = Position {
                x: dest_pos.x,
                y: dest_pos.y,
            };
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
                w.set_position(pos);
                w.handle_state = HandleState::Move.into();
            }
        }
    }
}
