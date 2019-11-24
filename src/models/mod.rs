pub mod windowwrapper;
pub mod rect;
pub mod screen;
pub mod dockarea;
pub mod window_type;
pub mod workspace;
pub mod monitor;

#[derive(Clone, Copy, Debug)]
pub enum WindowState {
    Snapped,
    Maximized,
    Free,
    Tiled
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    North,
    West,
    East,
    South
}
