
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(super) struct ConfigData {
    pub decoration_height: i32,
    pub border_width: i32,
    pub inner_border_width: i32,
    pub workspaces: HashMap<u8, String>,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            decoration_height: 20,
            border_width: 2,
            inner_border_width: 0,
            workspaces: Default::default()
        }
    }
}
