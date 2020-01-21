mod loader;
pub mod config;


use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::xlibwrapper::util::Color;
use config::*;


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


