#![allow(unused_variables)]

pub mod column_master;
pub mod floating;

use crate::models::{
    dockarea::DockArea, rect::Rect, screen::Screen, windowwrapper::WindowWrapper, Direction,
};
use crate::xlibwrapper::util::{Position, Size};
use crate::xlibwrapper::xlibmodels::Window;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum LayoutTag {
    Floating,
    ColumnMaster,
}

impl std::fmt::Display for LayoutTag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let tag = match *self {
            Self::Floating => "Floating",
            Self::ColumnMaster => "ColumnMaster",
        };
        write!(f, "{}", tag)
    }
}

pub trait LayoutClone {
    fn clone_layout(&self) -> Box<dyn Layout>;
}

impl <T> LayoutClone for T
where T: Layout + Clone + 'static {
    fn clone_layout(&self) -> Box<dyn Layout> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Layout> {
    fn clone(&self) -> Box<dyn Layout> {
        self.clone_layout()
    }
}

pub fn layout_from_tag(tag: LayoutTag) -> Box<dyn Layout> {
    match tag {
        LayoutTag::Floating => Box::new(floating::Floating::default()),
        LayoutTag::ColumnMaster => Box::new(column_master::ColumnMaster::default()),
    }
}

pub trait Layout: std::fmt::Debug + std::fmt::Display + LayoutClone {
    fn get_type(&self) -> LayoutTag;

    fn place_window(
        &mut self,
        dock_area: &DockArea,
        screen: &Screen,
        w: Window,
        windows: Vec<&WindowWrapper>,
    ) -> Vec<(Window, Rect)> {
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

    fn reorder(
        &mut self,
        focus: Window,
        screen: &Screen,
        dock_area: &DockArea,
        windows: Vec<WindowWrapper>,
    ) -> Vec<(Window, Rect)> {
        unimplemented!()
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
    ) -> Vec<WindowWrapper> {
        unimplemented!();
    }
}
