pub mod windowwrapper;
pub mod rect;
pub mod screen;
pub mod dockarea;
pub mod window_type;

#[derive(Clone, Copy)]
pub enum WindowState {
    Snapped,
    Maximized,
    Free,
    Tiled
}
