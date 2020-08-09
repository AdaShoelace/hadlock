pub mod dockarea;
pub mod monitor;
pub mod rect;
pub mod screen;
pub mod snapping_region;
pub mod window_type;
pub mod windowwrapper;
pub mod workspace;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowState {
    Snapped(Direction),
    Maximized,
    Monocle,
    Free,
    Tiled,
    Destroy,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
