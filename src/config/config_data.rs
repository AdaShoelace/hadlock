
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::xlibwrapper::util::Color;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigData {
    
    #[serde(rename="decorationHeight")]
    pub decoration_height: i32,

    #[serde(rename="borderWidth")]
    pub border_width: i32,

    #[serde(rename="innerBorderWidth")]
    pub inner_border_width: i32,

    #[serde(rename="borderColor")]
    pub border_color: Color,

    #[serde(rename="backgroundColor")]
    pub background_color: Color,

    #[serde(rename="focusedBackgroundColor")]
    pub focused_background_color: Color,

    #[serde(rename="workspaces")]
    pub workspaces: BTreeMap<u8, String>,
    
    #[serde(rename="terminal")]
    pub term: String,

    #[serde(rename="commands")]
    pub commands: Option<Vec<super::Command>>
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            decoration_height: 20,
            border_width: 2,
            inner_border_width: 0,
            border_color: Color::SolarizedCyan,
            background_color: Color::SolarizedDarkPurple,
            focused_background_color: Color::SolarizedPurple,
            workspaces: {
                let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
                let _ = (1..=12).for_each(|ws| {
                    workspaces.insert(ws, ws.to_string());
                });
                workspaces
            },
            term: "xterm".to_string(),
            commands: None
        }
    }
}
