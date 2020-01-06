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
        if self.current_monitor != wm::get_monitor_by_point(self, action.x_root, action.y_root) {
            self.current_monitor = wm::get_monitor_by_point(self, action.x_root, action.y_root);
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
