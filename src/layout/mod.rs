#![allow(unused_variables)]

pub mod floating;
pub mod tiled;
pub mod master_tiling;

use crate::windowmanager::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};
use crate::models::{
    screen::Screen,
    dockarea::DockArea,
    windowwrapper::WindowWrapper,
    Direction
};
use crate::xlibwrapper::xlibmodels::Window;

pub trait Layout : std::fmt::Debug {
    fn place_window(&self, dock_area: &DockArea, screen: &Screen, w: Window, ww: &WindowWrapper) -> Position {
        unimplemented!();
    }

    fn move_window(&self, screen: &Screen, dock_area: &DockArea, w: Window, x: i32, y: i32) -> (Position, Position) {
        unimplemented!();
    }

    fn resize_window(&self, ww: &WindowWrapper, w: Window, width: u32, height: u32) -> (Size, Size) {
        unimplemented!();
    }

    fn maximize(&self, screen: &Screen, dock_area: &DockArea, ww: &WindowWrapper, w: Window) -> Size {
        unimplemented!();
    }

    fn shift_window(&self, screen: &Screen, ww: &WindowWrapper, dock_area: &DockArea, w: Window, direction: Direction) -> (Position, Size) {
        unimplemented!();
    }
}
