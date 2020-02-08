#![allow(unused_variables)]

pub mod floating;
pub mod column_master;

use crate::models::{dockarea::DockArea, screen::Screen, windowwrapper::WindowWrapper, Direction, rect::Rect};
use crate::xlibwrapper::util::{Position, Size};
use crate::xlibwrapper::xlibmodels::Window;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LayoutTag {
    Floating,
    ColumnMaster
}

impl std::fmt::Display for LayoutTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tag = match *self {
            Self::Floating => "Floating",
            Self::ColumnMaster => "ColumnMaster"
        };
        write!(f, "{}", tag)
    }
}

pub trait Layout: std::fmt::Debug + std::fmt::Display {

    fn place_window(&mut self, dock_area: &DockArea, screen: &Screen, w: Window, windows: Vec<&WindowWrapper>) -> (Size, Position) {
        unimplemented!();
    }

    fn place_window_relative(
        &self,
        dock_area: &DockArea,
        screen: &Screen,
        w: Window,
        ww: &WindowWrapper,
    ) -> (Size, Position) {
        unimplemented!();
    }

    fn move_window(
        &self,
        screen: &Screen,
        dock_area: &DockArea,
        w: Window,
        respect_dock: bool,
        x: i32,
        y: i32,
    ) -> (Position, Position) {
        unimplemented!();
    }
    
    fn reorder(&self, screen: &Screen, dock_area: &DockArea, windows: Vec<WindowWrapper>) -> Vec<Rect> {
        unimplemented!()
    }

    fn resize_window(
        &self,
        ww: &WindowWrapper,
        w: Window,
        width: i32,
        height: i32,
    ) -> (Size, Size) {
        unimplemented!();
    }

    fn maximize(
        &self,
        screen: &Screen,
        dock_area: &DockArea,
        ww: &WindowWrapper,
        w: Window,
    ) -> (Position, Size) {
        unimplemented!();
    }

    fn monocle(
        &self,
        screen: &Screen,
        dock_area: &DockArea,
        ww: &WindowWrapper,
        w: Window,
    ) -> (Position, Size) {
        unimplemented!();
    }

    fn shift_window(
        &self,
        screen: &Screen,
        ww: &WindowWrapper,
        dock_area: &DockArea,
        w: Window,
        direction: Direction,
    ) -> (Position, Size) {
        unimplemented!();
    }
}
