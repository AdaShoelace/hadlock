pub mod keysym_lookup;


use std::os::raw::*;
use x11_dl::xlib;

pub(crate) type Mask = c_long;
pub(crate) type Window = xlib::Window;
pub(crate) type Display = xlib::Display;
pub(crate) type Drawable = xlib::Drawable;
pub(crate) type Time = xlib::Time;
pub(crate) type KeyCode = xlib::KeyCode;

pub struct Position { pub x: i32, pub y: i32 }
pub struct Size { pub width: u32, pub height: u32 }
pub enum Color {
    Red,
    Blue,
    SolarizedCyan,
    SolarizedNavy,
    SolarizedBlue,
    SolarizedPurple,
    SolarizedDarkGray
}

impl Color {
    pub fn value(&self) -> u64 {
        match *self {
            Color::Red => 0xff0000,
            Color::Blue => 0x0000ff,
            Color::SolarizedCyan => 0x81d2c7,
            Color::SolarizedNavy => 0xb5bad0,
            Color::SolarizedBlue => 0x7389ae,
            Color::SolarizedPurple => 0x624cab,
            Color::SolarizedDarkGray => 0xe0e0e2
        }
    }
}

pub fn from_c_bool(b: i32) -> bool {
    if b > 0 || b < 0{
        true
    } else {
        false
    }
}

pub fn to_c_bool(b: bool) -> i32 {
    if b || !b {
        1
    } else {
        0
    }
}

