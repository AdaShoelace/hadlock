#![allow(unused_variables)]

pub mod floating;
pub mod tiled;

use crate::windowmanager::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};
use crate::xlibwrapper::xlibmodels::Window;


pub trait Layout {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {
        unimplemented!();
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {
        unimplemented!();
    }

    fn resize_window(&self, wm: &WindowManager, w: Window, width: u32, height: u32) -> (Size, Size) {
        unimplemented!();
    }

    fn maximize(&self, wm: &WindowManager, w: Window) -> Size {
        unimplemented!();
    }
}
