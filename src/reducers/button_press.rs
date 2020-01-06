#![allow(unused_imports)]
use {
    crate::{
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::masks::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*
};


impl Reducer<action::ButtonPress> for State {
    fn reduce(&mut self, action: action::ButtonPress) {
        debug!("ButtonPress");
        let geometry = self.lib.get_geometry(action.win);
        self.drag_start_pos = (action.x_root as i32 , action.y_root as i32);
        self.drag_start_frame_pos = (geometry.x,geometry.y);
        self.drag_start_frame_size = (geometry.width, geometry.height);

        if action.button == Button1 && (action.state & Mod4Mask) == Mod4Mask {
            debug!("should raise");
            self.lib.raise_window(action.win);
        }
    }
}

