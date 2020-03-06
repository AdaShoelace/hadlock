pub mod keysym_lookup;

use serde::{self, de, Deserialize, Deserializer, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: i32,
    pub height: i32,
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
    let without_prefix = s.trim_start_matches("#");
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
            Color::Red => 0xff0000,
            Color::Blue => 0x0000ff,
            Color::SolarizedCyan => 0x81d2c7,
            Color::SolarizedNavy => 0xb5bad0,
            Color::SolarizedBlue => 0x7389ae,
            Color::SolarizedPurple => 0x624cab,
            Color::SolarizedDarkPurple => 0xaba9bf,
            Color::SolarizedDarkGray => 0xe0e0e2,
            Color::DefaultBackground => 0x5A3C85,
            Color::DefaultFocusedBackground => 0x9E416D,
            Color::DefaultBorder => 0x94c507,
            Color::Custom(value) => value,
        }
    }
}

pub fn from_c_bool(b: i32) -> bool {
    if b != 0 {
        true
    } else {
        false
    }
}

pub fn to_c_bool(b: bool) -> i32 {
    if b {
        1
    } else {
        0
    }
}
