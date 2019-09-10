#![allow(dead_code, unused_variables)]
use super::*;
use crate::config::*;
use crate::xlibwrapper::util::{
    Position,
    Size
};
use crate::models::Direction;
use crate::xlibwrapper::xlibmodels::*;


pub struct Floating;

impl Layout for Floating {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {

        let ww = wm.clients.get(&w).unwrap();

        let screen = wm.lib.get_screen();

        let dw = (screen.width - ww.get_width() as i32).abs() / 2;
        let mut dh = (screen.height - ww.get_height() as i32).abs() / 2;

        if let Some(dock_rect) = wm.dock_area.as_rect(&wm.lib.get_screen()) {
            dh = ((screen.height + dock_rect.get_size().height as i32) - ww.get_height() as i32).abs() / 2;
        }
        Position{x: dw, y: dh}
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {

        let mut y = y;
        match wm.dock_area.as_rect(&wm.lib.get_screen()) {
            Some(dock) => {
                if y < dock.get_size().height as i32 {
                    y = dock.get_size().height as i32;
                }
            }
            None => {}
        }

        (Position{x, y},
         Position{ x: x + CONFIG.inner_border_width + CONFIG.border_width,
         y: y + CONFIG.inner_border_width + CONFIG.border_width + CONFIG.decoration_height})
    }

    fn resize_window(&self, wm: &WindowManager, w: Window, width: u32, height: u32) -> (Size, Size) {

        let ww = wm.clients.get(&w).unwrap();

        let dec_size = if let Some(dec_rect) = ww.get_dec_rect() {
            let mut dec_w = width - (2 * CONFIG.border_width as u32);
            let mut dec_h = height - (2 * CONFIG.border_width as u32);

            if width == dec_rect.get_size().width {
                dec_w = width;
            } else if height == dec_rect.get_size().height {
                dec_h = height;
            }

            let dec_size = Size { width: dec_w, height: dec_h };
            dec_size
        } else { Size { width: 0, height: 0 } };

        let window_rect = ww.get_inner_rect();
        let mut d_width = width - (2* CONFIG.inner_border_width as u32) - (2 * CONFIG.border_width as u32);
        let mut d_height = height - (2* CONFIG.inner_border_width as u32) - (2 * CONFIG.border_width as u32) - CONFIG.decoration_height as u32;

        if width == window_rect.get_size().width {
            d_width = width;
        } else if height == window_rect.get_size().height {
            d_height = height;
        }

        let window_size = Size { width: d_width, height: d_height };

        (dec_size, window_size)
    }

    fn maximize(&self, wm: &WindowManager, w: Window) -> Size {
        let screen = wm.lib.get_screen();
        let width = (screen.width as u32) - 2 * CONFIG.border_width as u32;
        let height = (screen.height as u32) - 2 * CONFIG.border_width as u32;
        match wm.dock_area.as_rect(&screen) {
            Some(dock) => {
                self.resize_window(&wm, w, width, height - dock.get_size().height).0
            },
            None => {
                self.resize_window(&wm, w, width, height).0
            }
        }
    }

    fn shift_window(&self, wm: &WindowManager, w: Window, direction: Direction) -> (Position, Size) {
        let screen = wm.lib.get_screen();
        let ww = wm.clients.get(&w).unwrap();
        match direction {
            Direction::North => {
                let pos = self.move_window(wm, w, 0, 0).0;
                let size = self.resize_window(wm, w, screen.width as u32, (screen.height as u32) / 2).0;
                (pos, size)
            },
            Direction::East => {
                let pos = self.move_window(wm, w, screen.width / 2, 0).0;
                let size = self.resize_window(wm, w, (screen.width as u32) / 2, screen.height as u32).0;
                (pos, size)
            },
            Direction::West => {
                let pos = self.move_window(wm, w, 0, 0).0;
                let size = self.resize_window(wm, w, (screen.width as u32) / 2, screen.height as u32).0;
                (pos, size)
            },
            Direction::South => {
                let pos = self.move_window(wm, w, 0, screen.height / 2).0;
                let size = self.resize_window(wm, w, screen.width as u32, (screen.height as u32) / 2).0;
                (pos, size)
            }
        }
    }
}
