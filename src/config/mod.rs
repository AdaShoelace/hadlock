mod loader;
pub mod config_data;


use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use config_data::*;
use crate::xlibwrapper::util::Color;


lazy_static! {
    pub static ref CONFIG: Config = {
        loader::load_config()
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ExecTime {
    Pre,
    Post
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command { 

    #[serde(rename="execTime")]
    pub exec_time: ExecTime,
    pub program: String, 
    pub args: Vec<String> 
}

#[derive(Debug)]
pub struct Config {
    pub decoration_height: i32,
    pub border_width: i32,
    pub inner_border_width: i32,
    pub border_color: Color,
    pub background_color: Color,
    pub focused_background_color: Color,
    pub workspaces: BTreeMap<u8, String>,
    pub term: String,
    pub commands: Option<Vec<Command>>
}

impl From<ConfigData> for Config {
    fn from(config: ConfigData) -> Self {
        let def = ConfigData::default();
        Self {
            decoration_height: config.decoration_height.unwrap_or(def.decoration_height.unwrap()),
            border_width: config.border_width.unwrap_or(def.border_width.unwrap()),
            inner_border_width: config.inner_border_width.unwrap_or(def.inner_border_width.unwrap()),
            border_color: config.border_color.unwrap_or(def.border_color.unwrap()),
            background_color: config.background_color.unwrap_or(def.background_color.unwrap()),
            focused_background_color: config.focused_background_color.unwrap_or(def.focused_background_color.unwrap()),
            workspaces: config.workspaces.unwrap_or(def.workspaces.unwrap()),
            term: config.term.unwrap_or(def.term.unwrap()),
            commands: config.commands
        }
    }
}

