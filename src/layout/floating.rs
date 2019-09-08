#![allow(dead_code, unused_variables)]
use super::*;
use crate::config::*;
use crate::xlibwrapper::util::{
    Position,
};
use crate::xlibwrapper::xlibmodels::*;


pub struct Floating;

impl Layout for Floating {
    fn place_window(&self, wm: &WindowManager, w: Window) -> Position {
        
        let ww = wm.clients.get(&w).unwrap();

        let screen = wm.lib.get_screen();

        let dw = (screen.width - ww.get_width() as i32).abs() / 2;
        let mut dh = (screen.height - ww.get_height() as i32).abs() / 2;

        if let Some(dock_rect) = wm.dock_area.as_rect(wm.lib.get_screen()) {
            dh = ((screen.height + dock_rect.get_size().height as i32) - ww.get_height() as i32).abs() / 2;
        }
        Position{x: dw, y: dh}
    }

    fn move_window(&self, wm: &WindowManager, w: Window, x: i32, y: i32) -> (Position, Position) {

        let mut y = y;
        match wm.dock_area.as_rect(wm.lib.get_screen()) {
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
}
