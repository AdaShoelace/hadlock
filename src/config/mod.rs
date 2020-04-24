pub mod config_model;
mod loader;

use crate::models::Direction;
use lazy_static::*;
use serde::{Deserialize, Serialize};

use config_model::*;

lazy_static! { pub static ref CONFIG: Config =  loader::load_config(); }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum KeyEffect {
    Snap(Direction),
    Resize(Direction),
    MoveToWorkspace(u32),
    ChangeCurrentWorkspace(u32),
    CirculateLayout,
    Center,
    Reorder,
    OpenTerm,
    Kill,
    Custom(Command)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KeyAction {

    #[serde(rename = "modKey")]
    pub mod_key: Option<String>,
    pub key: String,
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
