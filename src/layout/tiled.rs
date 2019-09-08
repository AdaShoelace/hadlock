use super::*;
use crate::xlibwrapper::util::{
    Position
};
use crate::models::{
    windowwrapper::WindowWrapper,
};
use crate::xlibwrapper::xlibmodels::Window;


pub struct Tiled;

impl Layout for Tiled {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {

        

        Position{x: 0, y: 0}
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {
        (Position{x: 0, y: 0},
        Position{x: 0, y: 0})
    }
}
