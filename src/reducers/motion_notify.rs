use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
            HandleState
        },
        xlibwrapper::action,
        xlibwrapper::masks::*,
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


impl Reducer<action::MotionNotify> for State {
    fn reduce(&mut self, action: action::MotionNotify) {

        if self.current_monitor != self.get_monitor_by_mouse() {
            self.current_monitor = self.get_monitor_by_mouse();
            self.monitors
                .get(&self.current_monitor)
                .expect("MotionNotify - monitor - get_mut - change handle state")
                .handle_state.
                replace(HandleState::Focus);
        }

        let drag_pos = Position { x: action.x_root, y: action.y_root };
        let (delta_x, delta_y) =  (drag_pos.x - self.drag_start_pos.0,
            drag_pos.y - self.drag_start_pos.1);

        let dest_pos = Position{ x: self.drag_start_frame_pos.0 + delta_x,
        y: self.drag_start_frame_pos.1 + delta_y};


        if (action.state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
            let new_pos = Position{x: dest_pos.x, y: dest_pos.y};
            let (pos, _) = self.monitors.get_mut(&self.current_monitor).expect("MotionNotify - monitor - get_mut").move_window(action.win, new_pos.x, new_pos.y);
            let w = self.monitors.get_mut(&self.current_monitor).expect("MotionNotify - monitor - get_mut").get_client_mut(action.win).expect("motion_notify some window");
            w.set_position(pos);
            w.handle_state = HandleState::Move.into();
        }
    }
}

