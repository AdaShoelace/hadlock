pub mod keysym_lookup;

use crate::models::screen::Screen;
use serde::{self, de, Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn translate(self, x: i32, y: i32) -> Self {
        Self::new(x, y)
    }

    pub fn translate_relative(self, delta_x: i32, delta_y: i32) -> Self {
        Self {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl From<Screen> for (Position, Size) {
    fn from(screen: Screen) -> Self {
        (
            Position {
                x: screen.x,
                y: screen.y,
            },
            Size {
                width: screen.width,
                height: screen.height,
            },
        )
    }
}

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
    DefaultBackground,
    DefaultFocusedBackground,
    DefaultBorder,
    #[serde(deserialize_with = "color_deserialize")]
    Custom(u64),
}

fn color_deserialize<'de, D>(desierializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(desierializer)?;
    let without_prefix = s.trim_start_matches('#');
    match u64::from_str_radix(without_prefix, 16) {
        Ok(res) => Ok(res),
        Err(e) => Err(de::Error::custom(format!(
            "Failed to deserialize color: {}",
            e
        ))),
    }
}

impl Color {
    pub fn value(&self) -> u64 {
        match *self {
            Color::Red => 0x00ff_0000,
            Color::Blue => 0x0000_00ff,
            Color::SolarizedCyan => 0x0081_d2c7,
            Color::SolarizedNavy => 0xb005_bad0,
            Color::SolarizedBlue => 0x0073_89ae,
            Color::SolarizedPurple => 0x0062_4cab,
            Color::SolarizedDarkPurple => 0x00ab_a9bf,
            Color::SolarizedDarkGray => 0x00e0_e0e2,
            Color::DefaultBackground => 0x005A_3C85,
            Color::DefaultFocusedBackground => 0x009E_416D,
            Color::DefaultBorder => 0x0094_c507,
            Color::Custom(value) => value,
        }
    }
}

pub fn from_c_bool(b: i32) -> bool {
    b != 0
}

pub fn to_c_bool(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}
