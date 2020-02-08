pub mod dockarea;
pub mod monitor;
pub mod rect;
pub mod screen;
pub mod window_type;
pub mod windowwrapper;
pub mod workspace;

use std::cell::RefCell;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowState {
    Snapped,
    Maximized,
    Monocle,
    Free,
    _Tiled,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Copy, Clone, Debug)]
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
