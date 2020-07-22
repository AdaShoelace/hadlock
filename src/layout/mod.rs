#![allow(unused_variables)]

pub mod column_master;
pub mod floating;

use crate::config::Axis;
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

impl<T> LayoutClone for T
where
    T: Layout + Clone + 'static,
{
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

    fn resize(
        &self,
        win: Window,
        axis: &Axis,
        delta: i32,
        windows: &[&WindowWrapper],
    ) -> Vec<WindowWrapper> {
        windows
            .into_iter()
            .map(|ww| {
                if ww.window() == win {
                    let old_size = ww.get_size();
                    let size = match axis {
                        Axis::Vertical => Size {
                            width: old_size.width,
                            height: old_size.height + delta,
                        },
                        Axis::Horizontal => Size {
                            width: old_size.width + delta,
                            height: old_size.height,
                        },
                    };
                    WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        ..**ww
                    }
                } else {
                    WindowWrapper { ..**ww }
                }
            })
            .collect::<Vec<WindowWrapper>>()
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
        let pos = self.move_window(screen, dock_area, w, true, screen.x, screen.y);
        match dock_area.as_rect(&screen) {
            Some(dock) => {
                let size = Size {
                    width: screen.width,
                    height: screen.height - dock.get_size().height,
                };
                (pos.0, size)
            }
            None => {
                let size = Size {
                    width: screen.width,
                    height: screen.height,
                };
                (pos.0, size)
            }
        }
    }

    fn monocle(
        &self,
        screen: &Screen,
        dock_area: &DockArea,
        ww: &WindowWrapper,
        w: Window,
    ) -> (Position, Size) {
        let pos = self.move_window(screen, dock_area, w, false, screen.x, screen.y);
        let size = Size {
            width: screen.width,
            height: screen.height,
        };
        (pos.0, size)
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
