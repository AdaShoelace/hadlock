use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
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
    reducer::*
};


impl Reducer<action::MotionNotify> for State {
    fn reduce(&mut self, action: action::MotionNotify) {
        debug!("MotionNotify");
        let drag_pos = Position { x: action.x_root, y: action.y_root };
        let (delta_x, delta_y) =  (drag_pos.x - self.drag_start_pos.0,
            drag_pos.y - self.drag_start_pos.1);
        let dest_pos = Position{ x: self.drag_start_frame_pos.0 + delta_x,
        y: self.drag_start_frame_pos.1 + delta_y};
        if let Some(w) = self.windows.get_mut(&action.win) {
            if (action.state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
                w.handle_state = HandleState::Move(Position{x: dest_pos.x, y: dest_pos.y});
            }
            if (action.state & (Button3Mask | Mod4Mask)) == Button3Mask | Mod4Mask {
                debug!("shift move");
                w.handle_state = HandleState::ResizeRelative(Size{width: dest_pos.x, height: dest_pos.y});
            }
        }
    }
}

