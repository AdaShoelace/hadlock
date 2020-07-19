pub mod dockarea;
pub mod internal_action;
pub mod monitor;
pub mod rect;
pub mod screen;
pub mod window_type;
pub mod windowwrapper;
pub mod workspace;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowState {
    Snapped,
    Maximized,
    Monocle,
    Free,
    Tiled,
    Destroy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    West,
    East,
    South,
}
