pub mod floating;
pub mod tiled;

use crate::windowmanager::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};
use crate::xlibwrapper::xlibmodels::Window;
use crate::models::{
    windowwrapper::WindowWrapper
};

pub trait Layout {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {
        unimplemented!();
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {
        unimplemented!();
    }

    fn resize_window(&self) -> Position {
        unimplemented!();
    }
}
