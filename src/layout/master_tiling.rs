use super::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};

use crate::xlibwrapper::xlibmodels::Window;


pub struct MasterTiling;

impl Layout for MasterTiling {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {
        let w_list = wm.get_windows_in_current_ws();

        Position{x: 0, y: 0}
    }

    fn move_window(&self, _wm: &WindowManager, _w: Window, _x: i32, _y: i32) -> (Position, Position) {
        (Position{x: 0, y: 0},
        Position{x: 0, y: 0})
    }

    fn resize_window(&self, _wm: &WindowManager, _w: Window, _width: u32, _height: u32) -> (Size, Size) {
        unimplemented!();
    }

    fn maximize(&self, wm: &WindowManager, w: Window) -> Size {
        unimplemented!();
    }
}
