pub mod floating;
pub mod tiled;

use crate::windowmanager::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};
use crate::xlibwrapper::xlibmodels::Window;


pub trait Layout {
    fn place_window(&self, _wm: &WindowManager, _w: Window) -> Position {
        unimplemented!();
    }

    fn move_window(&self, _wm: &WindowManager, _w: Window, _x: i32, _y: i32) -> (Position, Position) {
        unimplemented!();
    }

    fn resize_window(&self, wm: &WindowManager, w: Window, width: u32, height: u32) -> (Size, Size) {
        unimplemented!();
    }
}
