use crate::xlibwrapper::xlibmodels::*;
pub mod windowwrapper;
pub mod rect;
pub mod screen;
pub mod dockarea;
pub mod window_type;
pub mod monitor;
pub mod workspace;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowState {
    Snapped,
    Maximized,
    Monocle,
    Free,
    _Tiled
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    West,
    East,
    South
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
