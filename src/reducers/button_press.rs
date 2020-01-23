#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*},
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::masks::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::rc::Rc,
};

impl Reducer<action::ButtonPress> for State {
    fn reduce(&mut self, action: action::ButtonPress) {
        //debug!("ButtonPress");
        let geometry = self.lib.get_geometry(action.win);
        self.drag_start_pos = (action.x_root as i32, action.y_root as i32);
        self.drag_start_frame_pos = (geometry.x, geometry.y);
        self.drag_start_frame_size = (geometry.width, geometry.height);

        if action.button == Button1 && (action.state & Mod4Mask) == Mod4Mask {
            //debug!("should raise");
            self.lib.raise_window(action.win);
        }
    }
}
