
use x11_dl::xlib;

pub(crate) type Mask = i64;
pub(crate) type Window = xlib::Window;
pub(crate) type Display = xlib::Display;
pub(crate) type Drawable = xlib::Drawable;
pub(crate) type Time = xlib::Time;
pub(crate) type KeyCode = xlib::KeyCode;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WindowChanges {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub sibling: Window,
    pub stack_mode: i32,
}

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
