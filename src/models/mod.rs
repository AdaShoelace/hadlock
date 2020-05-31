pub mod dockarea;
pub mod internal_action;
pub mod monitor;
pub mod rect;
pub mod screen;
pub mod window_type;
pub mod windowwrapper;
pub mod workspace;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowState {
    Snapped,
    Maximized,
    Monocle,
    Free,
    Tiled,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum HandleState {
    New,
    Handled,
    Map,
    Unmap,
    Focus,
    Unfocus,
    Destroy,
    Move,
    Center,
    Shift,
    Resize,
    Maximize,
    MaximizeRestore,
    Monocle,
    MonocleRestore,
    UpdateLayout,
}

impl From<HandleState> for Vec<HandleState> {
    fn from(w: HandleState) -> Vec<HandleState> {
        vec![w]
    }
}

impl From<HandleState> for RefCell<Vec<HandleState>> {
    fn from(w: HandleState) -> RefCell<Vec<HandleState>> {
        RefCell::new(vec![w])
    }
}
