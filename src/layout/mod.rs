#![allow(unused_variables)]

pub mod floating;

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
    fn place_window(&self, dock_area: &DockArea, screen: &Screen, w: Window) -> (Size, Position) {
        unimplemented!();
    }

    fn place_window_relative(&self, dock_area: &DockArea, screen: &Screen, w: Window, ww: &WindowWrapper) -> (Size, Position) {
        unimplemented!();
    }

    fn move_window(&self, screen: &Screen, dock_area: &DockArea, w: Window, respect_dock: bool, x: i32, y: i32) -> (Position, Position) {
        unimplemented!();
    }

    fn resize_window(&self, ww: &WindowWrapper, w: Window, width: i32, height: i32) -> (Size, Size) {
        unimplemented!();
    }

    fn maximize(&self, screen: &Screen, dock_area: &DockArea, ww: &WindowWrapper, w: Window) -> (Position, Size) {
        unimplemented!();
    }

    fn monocle(&self, screen: &Screen, dock_area: &DockArea, ww: &WindowWrapper, w: Window) -> (Position, Size) {
        unimplemented!();
    }

    fn shift_window(&self, screen: &Screen, ww: &WindowWrapper, dock_area: &DockArea, w: Window, direction: Direction) -> (Position, Size) {
        unimplemented!();
    }
}
