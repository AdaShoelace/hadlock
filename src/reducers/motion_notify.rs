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
    reducer::*,
    std::cell::RefCell,
};


impl Reducer<action::MotionNotify> for State {
    fn reduce(&mut self, action: action::MotionNotify) {
        let drag_pos = Position { x: action.x_root, y: action.y_root };
        let (delta_x, delta_y) =  (drag_pos.x - self.drag_start_pos.0,
            drag_pos.y - self.drag_start_pos.1);

        let dest_pos = Position{ x: self.drag_start_frame_pos.0 + delta_x,
        y: self.drag_start_frame_pos.1 + delta_y};

        if let Some(w) = self.windows.get_mut(&action.win) {
            if (action.state & (Button1Mask | Mod4Mask)) == Button1Mask | Mod4Mask {
                let new_pos = Position{x: dest_pos.x, y: dest_pos.y};
                w.set_position(new_pos.clone());
                w.handle_state = HandleState::Move.into();
            }
            if (action.state & (Button3Mask | Mod4Mask)) == Button3Mask | Mod4Mask {
                debug!("shift move");
                let sign_x = if delta_x >= 0 { 1 } else { -1 };
                let sign_y = if delta_y >= 0 { 1 } else { -1 };
                let new_size = Size{ width: w.get_size().width  + sign_x, height: w.get_size().height + sign_y};
                w.set_size(new_size);
                w.handle_state = HandleState::Resize.into();
            }
        }
    }
}

