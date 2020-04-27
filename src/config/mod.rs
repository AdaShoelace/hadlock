pub mod config_model;
mod loader;

use crate::models::Direction;
use lazy_static::*;
use serde::{Deserialize, Serialize};

use config_model::*;

lazy_static! { pub static ref CONFIG: Config =  loader::load_config(); }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Key {
    Letter(String),
    Number
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum KeyEffect {
    Snap(Direction),
    Resize(i32, Axis),
    MoveToWorkspace,
    ChangeCurrentWorkspace,
    CirculateLayout,
    Center,
    Reorder,
    OpenTerm,
    Kill,
    Exit,
    ShiftWindow(Direction),
    SwapMaster,
    ToggleMonocle,
    ToggleMaximize,
    Custom(Command)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyAction {
    #[serde(rename = "modKey")]
    pub mod_key: Option<String>,
    pub key: Key,
    pub effect: KeyEffect
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ExecTime {
    Pre,
    Post,
    Now
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Command {
    #[serde(rename = "execTime")]
    pub exec_time: ExecTime,
    pub program: String,
    pub args: Vec<String>,
}
