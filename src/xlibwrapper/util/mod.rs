pub mod keysym_lookup;

use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug)]
pub struct Position { pub x: i32, pub y: i32 }

#[derive(Copy, Clone, Debug)]
pub struct Size { pub width: u32, pub height: u32 }

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Color {
    Red,
    Blue,
    SolarizedCyan,
    SolarizedNavy,
    SolarizedBlue,
    SolarizedPurple,
    SolarizedDarkPurple,
    SolarizedDarkGray,
    Custom(u64)
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
            Color::SolarizedDarkPurple => 0xaba9bf,
            Color::SolarizedDarkGray => 0xe0e0e2,
            Color::Custom(value) => value
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

