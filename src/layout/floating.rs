#![allow(dead_code, unused_variables)]
use super::*;
use crate::{
    config::*,
    xlibwrapper::{
        xlibmodels::*,
        util::{
            Position,
            Size
        },
    },
    models::{
        Direction,
        windowwrapper::WindowWrapper,
        screen::Screen,
        dockarea::DockArea
    },
};

#[derive(Debug)]
pub struct Floating;

impl Floating {
    fn get_size(ww: &WindowWrapper, size: (Size, Size)) -> Size {
        if ww.is_decorated() {
            size.0
        } else {
            size.1
        }
    }
}

impl Layout for Floating {

    fn place_window(&self, dock_area: &DockArea, screen: &Screen, w: Window, ww: &WindowWrapper) -> (Size, Position) {

        let ww_size = ww.get_size();
        let new_size = Size{ width: (screen.width as u32 / 10) * 8, height: (screen.height as u32 / 10) * 6 as u32 };

        let dw = (screen.width - new_size.width as i32) / 2;
        let mut dh = (screen.height - new_size.height as i32) / 2;

        if let Some(dock_rect) = dock_area.as_rect(&screen) {
            dh = ((screen.height + dock_rect.get_size().height as i32) - new_size.height as i32) / 2;
        }
        let ret = Position{x: screen.x + dw, y: screen.y + dh};
        
        (new_size, ret)
    }

    fn move_window(&self, screen: &Screen, dock_area: &DockArea, w: Window, x: i32, y: i32) -> (Position, Position) {

        let mut y = y;
        match dock_area.as_rect(&screen) {
            Some(dock) => {
                if y < dock.get_size().height as i32 {
                    y = dock.get_size().height as i32 - CONFIG.border_width; }
            }
            None => {}
        }

        (Position{x, y},
         Position{ x: x + CONFIG.border_width,
         y: y + CONFIG.decoration_height + CONFIG.border_width})
    }

    fn resize_window(&self, ww: &WindowWrapper, w: Window, width: u32, height: u32) -> (Size, Size) {
        println!("width: {}", width);

        println!("is window decorated: {}", ww.is_decorated());
        if let Some(dec_rect) = ww.get_dec_rect() {
            let dec_size = Size { width, height };
            let window_size = Size { width, height: height - CONFIG.decoration_height as u32 };
            (dec_size, window_size)
        } else {
            let dec_size = Size { width: 0, height: 0 };
            let window_size = Size { width, height };
            (dec_size, window_size)
        }
    }

    fn maximize(&self, screen: &Screen, dock_area: &DockArea, ww: &WindowWrapper, w: Window) -> Size {
        match dock_area.as_rect(&screen) {
            Some(dock) => {
                Floating::get_size(&ww, self.resize_window(&ww, w, screen.width as u32 - 2 * CONFIG.border_width as u32, screen.height as u32 - CONFIG.border_width as u32- dock.get_size().height))
            },
            None => {
                Floating::get_size(&ww, self.resize_window(&ww, w, screen.width as u32, screen.height as u32))
            }
        }
    }

    fn shift_window(&self, screen: &Screen, ww: &WindowWrapper, dock_area: &DockArea, w: Window, direction: Direction) -> (Position, Size) {
        let origin = Position { x: screen.x, y: screen.y };
        match direction {
            Direction::North => {
                let pos = self.move_window(&screen, &dock_area, w, screen.x, screen.y).0;
                let size = if ww.is_decorated() {
                    Size { width: screen.width as u32 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) / 2 - 2 * CONFIG.border_width as u32}
                } else {
                    Size { width: screen.width as u32, height: (screen.height as u32) / 2}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(&ww, w, size.width, size.height));
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height / 2;
                }
                (pos, size)
            },
            Direction::East => {
                let pos = self.move_window(&screen, &dock_area, w, screen.x + screen.width / 2, screen.y).0;
                let size = if ww.is_decorated() {
                    Size{ width: (screen.width as u32) / 2 - 2 * CONFIG.border_width as u32, height: screen.height as u32 - CONFIG.border_width as u32}

                } else {
                    Size{ width: (screen.width as u32) / 2, height: screen.height as u32}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(&ww, w, size.width, size.height));
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                (pos, size)
            },
            Direction::West => {
                let pos = self.move_window(&screen, &dock_area, w, screen.x, screen.y).0;
                let size = if ww.is_decorated() {
                    Size{ width: (screen.width as u32) / 2 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) - CONFIG.border_width as u32}
                } else {
                    Size{ width: (screen.width as u32) / 2, height: (screen.height as u32)}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(&ww, w, size.width, size.height));
                if let Some(dock) = dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                (pos, size)
            },
            Direction::South => {
                let mut pos = self.move_window(&screen, &dock_area, w, screen.x, screen.height / 2).0;
                let size = if ww.is_decorated() {
                    Size{ width: screen.width as u32 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) / 2 - 2 * CONFIG.border_width as u32}
                } else {
                    Size{ width: screen.width as u32 , height: (screen.height as u32) / 2}

                };
                let mut size = Floating::get_size(&ww, self.resize_window(&ww, w, size.width, size.height));
                if let Some(dock) = dock_area.as_rect(&screen) {
                    let offset = dock.get_size().height / 2 - CONFIG.border_width as u32;
                    size.height -= offset;
                    pos.y += offset as i32;
                }
                (pos, size)
            }
        }
    }

}

