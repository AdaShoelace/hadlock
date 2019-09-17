mod loader;
pub mod config_data;


use lazy_static::*;

use config_data::*;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: ConfigData = {
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

