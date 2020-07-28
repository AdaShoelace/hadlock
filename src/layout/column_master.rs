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
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct ColumnMaster {
    offset: RefCell<i32>,
    pub layout_type: LayoutTag,
}

impl ColumnMaster {
    fn column_maximize(
        &self,
        w: Window,
        screen: &Screen,
        dock_area: &DockArea,
    ) -> (Size, Position) {
        let gap = if CONFIG.smart_gaps {
            0
        } else {
            CONFIG.outer_gap
        };
        let (mut size, mut pos) = (
            Size {
                width: screen.width - gap * 2,
                height: screen.height - gap * 2,
            },
            Position {
                x: screen.x + gap,
                y: screen.y + gap,
            },
        );
        match dock_area.as_rect(screen) {
            Some(dock) => {
                size.height -= dock.get_size().height;
                pos.y += dock.get_size().height;
                (size, pos)
            }
            None => (size, pos),
        }
    }

    fn column_height(&self, screen: &Screen, dock: &DockArea, column: &[&WindowWrapper]) -> i32 {
        let dock_height = match dock.as_rect(screen) {
            Some(dock_rect) => dock_rect.get_size().height,
            None => 0,
        };

        let mut ret = ((screen.height - dock_height - 2 * CONFIG.outer_gap)
            / if column.len() > 1 {
                column.len() as i32
            } else {
                1
            })
            - 2 * CONFIG.border_width;

        if column.len() > 2 {
            ret -= ((column.len() as i32 - 1).abs() * CONFIG.inner_gap) / column.len() as i32
        }
        ret
    }
}

impl Default for ColumnMaster {
    fn default() -> Self {
        Self {
            layout_type: LayoutTag::ColumnMaster,
            offset: RefCell::new(0),
        }
    }
}

impl std::fmt::Display for ColumnMaster {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.layout_type)
    }
}

impl Layout for ColumnMaster {
    fn get_type(&self) -> LayoutTag {
        self.layout_type
    }

    fn place_window(
        &mut self,
        dock_area: &DockArea,
        screen: &Screen,
        w: Window,
        windows: Vec<&WindowWrapper>,
    ) -> Vec<(Window, Rect)> {
        //debug!("Incoming window vector in column_master: {:#?}", windows);
        let windows = windows
            .into_iter()
            .filter(|ww| !ww.is_trans)
            .collect::<Vec<&WindowWrapper>>();

        let dock_height = match dock_area.as_rect(screen) {
            Some(dock_rect) => dock_rect.get_size().height,
            None => 0,
        };

        let mut ret_vec = Vec::<(Window, Rect)>::new();

        let column_width = ((screen.width / 2) - 2 * CONFIG.border_width)
            - (CONFIG.outer_gap + (CONFIG.inner_gap / 2));
        let column_x = (screen.x + screen.width / 2) + CONFIG.inner_gap + *self.offset.borrow();
        let offset = *self.offset.borrow();
        if windows.is_empty() {
            let (size, pos) = self.column_maximize(w, &screen, &dock_area);
            ret_vec.push((w, Rect::new(pos, size)));
            return ret_vec;
        } else {
            let size = Size {
                width: column_width + offset,
                height: screen.height
                    - dock_height
                    - 2 * CONFIG.border_width
                    - 2 * CONFIG.outer_gap,
            };
            let pos = Position {
                x: screen.x + CONFIG.outer_gap,
                y: screen.y + dock_height + CONFIG.outer_gap,
            };
            let windows = windows
                .into_iter()
                .filter(|win| w != win.window())
                .collect::<Vec<&WindowWrapper>>();
            for (index, win) in windows.iter().enumerate() {
                let pos = Position {
                    x: column_x,
                    y: ((screen.y + dock_height)
                        + self.column_height(&screen, &dock_area, &windows) * index as i32)
                        + (2 * CONFIG.border_width) * index as i32
                        + CONFIG.outer_gap
                        + (CONFIG.inner_gap * index as i32),
                };
                let size = Size {
                    width: column_width + (-offset),
                    height: self.column_height(&screen, &dock_area, &windows),
                };
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
                    y = dock.get_size().height as i32;
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

    fn resize(
        &self,
        win: Window,
        axis: &Axis,
        delta: i32,
        windows: &[&WindowWrapper],
    ) -> Vec<WindowWrapper> {
        // The new size always refer to master pane (left solo window)
        // Only X-axis is to be respected
        *self.offset.borrow_mut() += delta;
        use std::collections::HashMap;
        let trans = windows
            .into_iter()
            .filter(|ww| ww.is_trans)
            .map(|ww| (ww.window(), *ww))
            .collect::<HashMap<Window, &WindowWrapper>>();
        if let Some(win) = trans.get(&win) {
            let size = win.get_size();
            let size = if *axis == Axis::Vertical {
                Size {
                    width: size.width,
                    height: size.height + delta,
                }
            } else {
                Size {
                    width: size.width + delta,
                    height: size.height,
                }
            };
            return vec![WindowWrapper {
                window_rect: Rect::new(win.get_position(), size),
                ..**win
            }];
        }

        if windows.len() <= 1 || *axis == Axis::Vertical {
            return vec![];
        }
        let mut windows = windows
            .into_iter()
            .filter(|ww| !ww.is_trans)
            .map(|ww| *ww)
            .collect::<Vec<&WindowWrapper>>();
        let mut ret = vec![];
        if let Some(master) = windows.pop() {
            let size = master.get_size();
            ret.push(WindowWrapper {
                window_rect: Rect::new(
                    master.get_position(),
                    Size {
                        width: size.width + delta,
                        height: size.height,
                    },
                ),
                ..*master
            });
        }
        for ww in windows {
            let pos = ww.get_position();
            let size = ww.get_size();
            ret.push(WindowWrapper {
                window_rect: Rect::new(
                    Position::new(pos.x + delta, pos.y),
                    Size {
                        width: size.width + (-delta),
                        height: size.height,
                    },
                ),
                ..*ww
            });
        }
        ret
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
            let focus = match windows.pop() {
                Some(ww) => ww.window(),
                _ => focus,
            };
            self.place_window(&dock_area, &screen, focus, windows)
        }
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
                let size = Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
                let mut size = ww.get_size();
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
                let size = Size {
                    width: (screen.width) / 2 - 2 * CONFIG.border_width,
                    height: screen.height - CONFIG.border_width,
                };
                let mut size = ww.get_size();
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
                let size = Size {
                    width: (screen.width) / 2 - 2 * CONFIG.border_width,
                    height: (screen.height) - CONFIG.border_width,
                };
                let mut size = ww.get_size();
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
                let size = Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
                let mut size = ww.get_size();
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
