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
    },
};


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

    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {
        let ww = wm.clients.get(&w).unwrap();

        let screen = wm.get_focused_screen();

        let dw = (screen.width - ww.get_width() as i32).abs() / 2;
        let mut dh = (screen.height - ww.get_height() as i32).abs() / 2;

        if let Some(dock_rect) = wm.dock_area.as_rect(&screen) {
            dh = ((screen.height + dock_rect.get_size().height as i32) - ww.get_height() as i32).abs() / 2;
        }
        let ret = Position{x: screen.x + dw, y: screen.y + dh};
        {
            use std::fs::{File, OpenOptions};
            use std::io::{Write};
            let path = "/home/pierre/hadlock.log";
            match OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(path)
                {
                    Ok(mut x) => {
                        let output_text = &format!("Floating::place_window, Screen from \"pointer_is_inside\":{:?}, window position: {:?}" , wm.get_focused_screen(), ret);
                        write!(x, "{}\n", output_text);
                    },
                    Err(e) => println!("{}", e)
                };
        }
        ret
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {

        let mut y = y;
        match wm.dock_area.as_rect(&wm.get_focused_screen()) {
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

    fn resize_window(&self, wm: &WindowManager, w: Window, width: u32, height: u32) -> (Size, Size) {
        println!("width: {}", width);
        let ww = wm.clients.get(&w).unwrap();

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

    fn maximize(&self, wm: &WindowManager, w: Window) -> Size {
        let screen = wm.get_focused_screen();
        let ww = wm.clients.get(&w).expect("Not in client list, called from Floating::maxmize");

        let ww = wm.clients.get(&w).expect("window missing in clients, call in Floating::maximize");
        match wm.dock_area.as_rect(&screen) {
            Some(dock) => {
                Floating::get_size(&ww, self.resize_window(&wm, w, screen.width as u32 - 2 * CONFIG.border_width as u32, screen.height as u32 - CONFIG.border_width as u32- dock.get_size().height))
            },
            None => {
                Floating::get_size(&ww, self.resize_window(&wm, w, screen.width as u32, screen.height as u32))
            }
        }
    }

    fn shift_window(&self, wm: &WindowManager, w: Window, direction: Direction) -> (Position, Size) {
        let screen = wm.get_focused_screen();
        let origin = Position { x: screen.x, y: screen.y };
        let ww = wm.clients.get(&w).unwrap();
        match direction {
            Direction::North => {
                let pos = self.move_window(wm, w, screen.x, screen.y).0;
                let size = if ww.is_decorated() {
                    Size { width: screen.width as u32 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) / 2 - 2 * CONFIG.border_width as u32}
                } else {
                    Size { width: screen.width as u32, height: (screen.height as u32) / 2}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(wm, w, size.width, size.height));
                if let Some(dock) = wm.dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height / 2;
                }
                (pos, size)
            },
            Direction::East => {
                let pos = self.move_window(wm, w, screen.x + screen.width / 2, screen.y).0;
                let size = if ww.is_decorated() {
                    Size{ width: (screen.width as u32) / 2 - 2 * CONFIG.border_width as u32, height: screen.height as u32 - CONFIG.border_width as u32}

                } else {
                    Size{ width: (screen.width as u32) / 2, height: screen.height as u32}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(wm, w, size.width, size.height));
                if let Some(dock) = wm.dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                (pos, size)
            },
            Direction::West => {
                let pos = self.move_window(wm, w, screen.x, screen.y).0;
                let size = if ww.is_decorated() {
                    Size{ width: (screen.width as u32) / 2 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) - CONFIG.border_width as u32}
                } else {
                    Size{ width: (screen.width as u32) / 2, height: (screen.height as u32)}
                };
                let mut size = Floating::get_size(&ww, self.resize_window(wm, w, size.width, size.height));
                if let Some(dock) = wm.dock_area.as_rect(&screen) {
                    size.height -= dock.get_size().height;
                }
                (pos, size)
            },
            Direction::South => {
                let mut pos = self.move_window(wm, w, screen.x, screen.height / 2).0;
                let size = if ww.is_decorated() {
                    Size{ width: screen.width as u32 - 2 * CONFIG.border_width as u32, height: (screen.height as u32) / 2 - 2 * CONFIG.border_width as u32}
                } else {
                    Size{ width: screen.width as u32 , height: (screen.height as u32) / 2}

                };
                let mut size = Floating::get_size(&ww, self.resize_window(wm, w, size.width, size.height));
                if let Some(dock) = wm.dock_area.as_rect(&screen) {
                    let offset = dock.get_size().height / 2 - CONFIG.border_width as u32;
                    size.height -= offset;
                    pos.y += offset as i32;
                }
                (pos, size)
            }
        }
    }

}

