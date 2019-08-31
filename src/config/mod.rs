mod loader;
pub mod config_data;


use lazy_static::*;
use loader::*;
use config_data::*;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref CONFIG: ConfigData = {
        loader::load_config()
    };
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Command { pub program: String, pub args: Vec<String> }

