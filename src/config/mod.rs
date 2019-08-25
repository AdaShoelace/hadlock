mod loader;
pub mod config_data;


use lazy_static::*;
use loader::*;
use config_data::*;

lazy_static! {
    pub static ref CONFIG: ConfigData = {
        loader::load_config()
    };
}

