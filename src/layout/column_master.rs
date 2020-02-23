#![allow(dead_code, unused_variables)]
use super::*;
use crate::{
    config::*,
    models::{
        dockarea::DockArea, rect::Rect, screen::Screen, windowwrapper::WindowWrapper, Direction,
    },
    xlibwrapper::{
        util::{Position, Size},
        xlibmodels::*,
    },
};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct ColumnMaster {
    layout_type: LayoutTag,
}

impl ColumnMaster {
    fn get_size(ww: &WindowWrapper, size: (Size, Size)) -> Size {
        if ww.is_decorated() {
            size.0
        } else {
            size.1
        }
    }

    fn column_maximize(
        &self,
        w: Window,
        screen: &Screen,
        dock_area: &DockArea,
    ) -> (Size, Position) {
        match dock_area.as_rect(screen) {
            Some(dock) => (
                Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: screen.height - dock.get_size().height - 2 * CONFIG.border_width,
                },
                Position {
                    x: screen.x,
                    y: screen.y + dock.get_size().height,
                },
            ),
            None => (
                Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: screen.height + 2 * CONFIG.border_width,
                },
                Position {
                    x: screen.x,
                    y: screen.y,
                },
            ),
        }
    }

    fn column_height(
        &self,
        screen: &Screen,
        dock: &DockArea,
        column: &VecDeque<&WindowWrapper>,
    ) -> i32 {
        let dock_height = match dock.as_rect(screen) {
            Some(dock_rect) => dock_rect.get_size().height,
            None => 0,
        };

        ((screen.height - dock_height)
            / if column.len() > 1 {
                column.len() as i32
            } else {
                1
            })
            - 2 * CONFIG.border_width
    }
}

impl Default for ColumnMaster {
    fn default() -> Self {
        Self {
            layout_type: LayoutTag::ColumnMaster,
        }
    }
}

impl std::fmt::Display for ColumnMaster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.layout_type)
    }
}

impl Layout for ColumnMaster {
    fn place_window(
        &mut self,
        dock_area: &DockArea,
        screen: &Screen,
        w: Window,
        windows: Vec<&WindowWrapper>,
    ) -> Vec<(Window, Rect)> {
        debug!("Incoming window vector in column_master: {:#?}", windows);
        let windows = windows.into_iter().collect::<VecDeque<&WindowWrapper>>();

        let dock_height = match dock_area.as_rect(screen) {
            Some(dock_rect) => dock_rect.get_size().height,
            None => 0,
        };

        let mut ret_vec = Vec::<(Window, Rect)>::new();

        let column_width = (screen.width / 2) - 2 * CONFIG.border_width;
        let column_x = screen.width / 2;

        if windows.is_empty() {
            let (size, pos) = self.column_maximize(w, &screen, &dock_area);
            ret_vec.push((w, Rect::new(pos, size)));
            return ret_vec;
        } else {
            let size = Size {
                width: column_width,
                height: screen.height - dock_height - 2 * CONFIG.border_width,
            };
            let pos = Position {
                x: screen.x,
                y: screen.y + dock_height,
            };
            for (index, win) in windows.iter().enumerate() {
                let pos = Position {
                    x: column_x,
                    y: ((screen.y + dock_height)
                        + self.column_height(&screen, &dock_area, &windows) * index as i32)
                        + (2 * CONFIG.border_width) * index as i32,
                };
                let size = Size {
                    width: column_width,
                    height: self.column_height(&screen, &dock_area, &windows),
                };
                debug!(
                    "Pushing w: {}, at pos: {:#?} with size: {:#?}",
                    win.window(),
                    pos,
                    size
                );
                ret_vec.push((win.window(), Rect::new(pos, size)))
            }
            ret_vec.push((w, Rect::new(pos, size)));
        }
        ret_vec
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
        let mut y = y;
        match dock_area.as_rect(&screen) {
            Some(dock) if respect_dock => {
                if y < dock.get_size().height as i32 {
                    y = dock.get_size().height as i32 - CONFIG.border_width;
                }
            }
            Some(_) | None => {}
        }

        (
            Position { x, y },
            Position {
                x: x + CONFIG.border_width,
                y: y + CONFIG.decoration_height + CONFIG.border_width,
            },
        )
    }

    fn reorder(
        &mut self,
        focus: Window,
        screen: &Screen,
        dock_area: &DockArea,
        windows: Vec<WindowWrapper>,
    ) -> Vec<(Window, Rect)> {
        let mut windows = windows.iter().collect::<Vec<&WindowWrapper>>();

        if windows.is_empty() {
            return vec![];
        } else {
            let focus = windows.remove(0).window();
            self.place_window(&dock_area, &screen, focus, windows)
        }
    }

    fn resize_window(
        &self,
        ww: &WindowWrapper,
        w: Window,
        width: i32,
        height: i32,
    ) -> (Size, Size) {
        println!("width: {}", width);

        println!("is window decorated: {}", ww.is_decorated());
        if let Some(dec_rect) = ww.get_dec_rect() {
            let dec_size = Size { width, height };
            let window_size = Size {
                width,
                height: height - CONFIG.decoration_height,
            };
            (dec_size, window_size)
        } else {
            let dec_size = Size {
                width: 0,
                height: 0,
            };
            let window_size = Size { width, height };
            (dec_size, window_size)
        }
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
                let size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(
                        &ww,
                        w,
                        screen.width,
                        screen.height - dock.get_size().height,
                    ),
                );
                (pos.0, size)
            }
            None => {
                let size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(&ww, w, screen.width, screen.height),
                );
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
        // TODO: implement for decorated windows
        let pos = self.move_window(screen, dock_area, w, false, screen.x, screen.y);
        let size =
            ColumnMaster::get_size(&ww, self.resize_window(&ww, w, screen.width, screen.height));
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
        let origin = Position {
            x: screen.x,
            y: screen.y,
        };
        match direction {
            Direction::North => {
                let pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.y)
                    .0;
                let size = if ww.is_decorated() {
                    Size {
                        width: screen.width - 2 * CONFIG.border_width,
                        height: (screen.height) / 2 - 2 * CONFIG.border_width,
                    }
                } else {
                    Size {
                        width: screen.width - 2 * CONFIG.border_width,
                        height: (screen.height) / 2 - 2 * CONFIG.border_width,
                    }
                };
                let mut size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(&ww, w, size.width, size.height),
                );
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height / 2
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::East => {
                let pos = self
                    .move_window(
                        &screen,
                        &dock_area,
                        w,
                        true,
                        screen.x + screen.width / 2,
                        screen.y,
                    )
                    .0;
                let size = if ww.is_decorated() {
                    Size {
                        width: (screen.width) / 2 - 2 * CONFIG.border_width,
                        height: screen.height - CONFIG.border_width,
                    }
                } else {
                    Size {
                        width: (screen.width) / 2 - 2 * CONFIG.border_width,
                        height: screen.height - CONFIG.border_width,
                    }
                };
                let mut size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(&ww, w, size.width, size.height),
                );
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::West => {
                let pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.y)
                    .0;
                let size = if ww.is_decorated() {
                    Size {
                        width: (screen.width) / 2 - 2 * CONFIG.border_width,
                        height: (screen.height) - CONFIG.border_width,
                    }
                } else {
                    Size {
                        width: (screen.width) / 2 - 2 * CONFIG.border_width,
                        height: (screen.height) - CONFIG.border_width,
                    }
                };
                let mut size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(&ww, w, size.width, size.height),
                );
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::South => {
                let mut pos = self
                    .move_window(
                        &screen,
                        &dock_area,
                        w,
                        true,
                        screen.x,
                        screen.height / 2 - CONFIG.border_width,
                    )
                    .0;
                let size = if ww.is_decorated() {
                    Size {
                        width: screen.width - 2 * CONFIG.border_width,
                        height: (screen.height) / 2 - 2 * CONFIG.border_width,
                    }
                } else {
                    Size {
                        width: screen.width - 2 * CONFIG.border_width,
                        height: (screen.height) / 2 - CONFIG.border_width,
                    }
                };
                let mut size = ColumnMaster::get_size(
                    &ww,
                    self.resize_window(&ww, w, size.width, size.height),
                );
                if let Some(dock) = dock_area.as_rect(&screen) {
                    let offset = dock.get_size().height / 2;
                    size.height -= offset;
                    pos.y += offset as i32;
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
        }
    }
}
