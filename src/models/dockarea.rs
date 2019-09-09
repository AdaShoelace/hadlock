use super::rect::*;
use super::screen::*;
use crate::xlibwrapper::util::*;


#[derive(Clone, Debug, Default)]
pub struct DockArea {
    top: i32,
    top_start_x: i32,
    top_end_x: i32,

    bottom: i32,
    bottom_start_x: i32,
    bottom_end_x: i32,

    right: i32,
    right_start_y: i32,
    right_end_y: i32,

    left: i32,
    left_start_y: i32,
    left_end_y: i32,
}

impl From<&[i64]> for DockArea {
    fn from(slice: &[i64]) -> Self {
        DockArea {
            left: slice[0] as i32,
            right: slice[1] as i32,
            top: slice[2] as i32,
            bottom: slice[3] as i32,
            left_start_y: slice[4] as i32,
            left_end_y: slice[5] as i32,
            right_start_y: slice[6] as i32,
            right_end_y: slice[7] as i32,
            top_start_x: slice[8] as i32,
            top_end_x: slice[9] as i32,
            bottom_start_x: slice[10] as i32,
            bottom_end_x: slice[11] as i32,
        }
    }
}

impl DockArea {
    pub fn as_rect(&self, s: &Screen) -> Option<Rect> {
        let screen_width = s.width as i32;
        let screen_height = s.height as i32;

        if self.top > 0 {
            return Some(self.rect_from_top());
        }
        if self.bottom > 0 {
            return Some(self.rect_from_bottom(screen_height));
        }
        if self.left > 0 {
            return Some(self.rect_from_left());
        }
        if self.right > 0 {
            return Some(self.rect_from_right(screen_width));
        }
        None
    }

    fn rect_from_top(&self) -> Rect {
        Rect::new(
            Position {
                x: self.top_start_x,
                y: 0,
            },
            Size {
                width: self.top_end_x as u32 - self.top_start_x as u32,
                height: self.top as u32,
            }
        )
    }

    fn rect_from_bottom(&self, screen_height: i32) -> Rect {
        Rect::new(
            Position {

                x: self.bottom_start_x,
                y: screen_height - self.bottom,
            },
            Size {
                width: self.bottom_end_x as u32 - self.bottom_start_x as u32,
                height: self.bottom as u32,
            }
        )
    }

    fn rect_from_left(&self) -> Rect {
        Rect::new(
            Position {
                x: 0,
                y: self.left_start_y,
            },
            Size {
                width: self.left as u32,
                height: self.left_end_y as u32 - self.left_start_y as u32,
            }
        )
    }

    fn rect_from_right(&self, screen_width: i32) -> Rect {
        Rect::new(
            Position {
                x: screen_width - self.right,
                y: self.right_start_y,
            },
            Size {
                width: self.right as u32,
                height: self.right_end_y as u32 - self.right_start_y as u32,
            }
        )
    }
}
