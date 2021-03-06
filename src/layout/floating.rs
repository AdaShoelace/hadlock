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

#[derive(Debug, Clone)]
pub struct Floating {
    pub layout_type: LayoutTag,
}

impl Default for Floating {
    fn default() -> Self {
        Self {
            layout_type: LayoutTag::Floating,
        }
    }
}

impl std::fmt::Display for Floating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.layout_type)
    }
}

impl Layout for Floating {
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
        let new_size = Size {
            width: (screen.width / 10) * 8,
            height: (screen.height / 10) * 6,
        };

        let dw = (screen.width - new_size.width as i32) / 2;
        let mut dh = (screen.height - new_size.height as i32) / 2;

        if let Some(dock_rect) = dock_area.as_rect(&screen) {
            dh =
                ((screen.height + dock_rect.get_size().height as i32) - new_size.height as i32) / 2;
        }
        let new_pos = Position {
            x: screen.x + dw,
            y: screen.y + dh,
        };

        vec![(w, Rect::new(new_pos, new_size))]
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
        let space_rect = match dock_area.as_rect(screen) {
            Some(dock) => Rect::new(
                Position {
                    x: screen.x,
                    y: screen.y + dock.get_size().height,
                },
                Size {
                    width: screen.width,
                    height: screen.height - dock.get_size().height,
                },
            ),
            None => {
                let rect_tuple: (Position, Size) = screen.clone().into();
                Rect::new(rect_tuple.0, rect_tuple.1)
            }
        };

        let win_size_x = (space_rect.get_size().width / 2) - 2 * CONFIG.border_width;
        let win_size_y = (space_rect.get_size().height / 2) - 2 * CONFIG.border_width;

        let center_win_pos = Position {
            x: space_rect.get_position().x + (space_rect.get_size().width / 2) - (win_size_x / 2),
            y: space_rect.get_position().y + (space_rect.get_size().height / 2) - (win_size_y / 2),
        };

        let step_size_x = (space_rect.get_position().x - center_win_pos.x).abs()
            / if !windows.is_empty() {
                windows.len() as i32
            } else {
                1
            };
        let step_size_y = (space_rect.get_position().y - center_win_pos.y).abs()
            / if !windows.is_empty() {
                windows.len() as i32
            } else {
                1
            };

        windows
            .iter()
            .rev()
            .enumerate()
            .map(|(index, win)| {
                let pos = Position {
                    x: center_win_pos.x - (index as i32 * step_size_x),
                    y: center_win_pos.y - (index as i32 * step_size_y),
                };
                let size = Size {
                    width: win_size_x,
                    height: win_size_y,
                };
                (win.window(), Rect::new(pos, size))
            })
            .collect::<Vec<(Window, Rect)>>()
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
                let mut size = Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height / 2
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::NorthEast => {
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
                let mut size = Size {
                    width: (screen.width / 2) - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
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
                let mut size = Size {
                    width: (screen.width) / 2 - 2 * CONFIG.border_width,
                    height: screen.height - CONFIG.border_width,
                };
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::SouthEast => {
                let mut pos = self
                    .move_window(
                        &screen,
                        &dock_area,
                        w,
                        true,
                        screen.x + screen.width / 2,
                        screen.height / 2,
                    )
                    .0;
                let mut size = Size {
                    width: (screen.width) / 2 - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
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
            Direction::South => {
                let mut pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.height / 2)
                    .0;
                let mut size = Size {
                    width: screen.width - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
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
            Direction::SouthWest => {
                let mut pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.height / 2)
                    .0;
                let mut size = Size {
                    width: (screen.width / 2) - 2 * CONFIG.border_width,
                    height: (screen.height / 2) - 2 * CONFIG.border_width,
                };
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
            Direction::West => {
                let pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.y)
                    .0;
                let mut size = Size {
                    width: (screen.width) / 2 - 2 * CONFIG.border_width,
                    height: (screen.height) - CONFIG.border_width,
                };
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
            Direction::NorthWest => {
                let pos = self
                    .move_window(&screen, &dock_area, w, true, screen.x, screen.y)
                    .0;
                let mut size = Size {
                    width: (screen.width / 2) - 2 * CONFIG.border_width,
                    height: (screen.height) / 2 - 2 * CONFIG.border_width,
                };
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height / 2
                }
                vec![WindowWrapper {
                    window_rect: Rect::new(pos, size),
                    ..ww.clone()
                }]
            }
        }
    }
}
