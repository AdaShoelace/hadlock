use super::{rect::*, WindowState};
use crate::xlibwrapper::util::*;
use crate::xlibwrapper::xlibmodels::*;
use std::time::Instant;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowWrapper {
    pub window: Window,
    pub window_rect: Rect,
    pub hidden: bool,
    pub is_trans: bool,
    pub restore_position: Position,
    pub restore_size: Size,
    pub current_state: WindowState,
    pub previous_state: WindowState,
    pub toc: Instant,
}

impl WindowWrapper {
    pub fn new(window: Window, window_rect: Rect, is_trans: bool) -> Self {
        let restore_size = window_rect.get_size();
        Self {
            window,
            window_rect,
            hidden: false,
            is_trans,
            restore_position: Position { x: 0, y: 0 },
            restore_size,
            current_state: WindowState::Free,
            previous_state: WindowState::Free,
            toc: Instant::now(),
        }
    }

    pub fn set_window_state(&mut self, state: WindowState) {
        self.previous_state = self.current_state;
        self.current_state = state;
    }

    pub fn restore_prev_state(&mut self) {
        let temp = self.current_state;
        self.current_state = self.previous_state;
        self.previous_state = temp;
    }

    pub fn set_position(&mut self, pos: Position) {
        self.window_rect = Rect::new(pos, self.window_rect.get_size())
    }

    pub fn window(&self) -> Window {
        self.window
    }

    pub fn set_size(&mut self, size: Size) {
        self.restore_size = self.get_size();
        self.window_rect = Rect::new(self.window_rect.get_position(), size);
    }

    pub fn get_size(&self) -> Size {
        self.window_rect.get_size()
    }

    pub fn get_position(&self) -> Position {
        self.window_rect.get_position()
    }

    pub fn save_restore_position(&mut self) {
        self.restore_position = self.get_position();
    }

    pub fn get_restore_position(&self) -> Position {
        self.restore_position
    }

    pub fn save_restore_size(&mut self) {
        self.restore_size = self.get_size();
    }

    pub fn get_restore_size(&self) -> Size {
        self.restore_size
    }
}
