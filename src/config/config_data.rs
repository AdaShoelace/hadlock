
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::xlibwrapper::util::Color;


#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigData {
    
    #[serde(rename="decorationHeight")]
    pub decoration_height: Option<i32>,

    #[serde(rename="borderWidth")]
    pub border_width: Option<i32>,

    #[serde(rename="innerBorderWidth")]
    pub inner_border_width: Option<i32>,

    #[serde(rename="borderColor")]
    pub border_color: Option<Color>,

    #[serde(rename="backgroundColor")]
    pub background_color: Option<Color>,

    #[serde(rename="focusedBackgroundColor")]
    pub focused_background_color: Option<Color>,

    #[serde(rename="workspaces")]
    pub workspaces: Option<BTreeMap<u8, String>>,
    
    #[serde(rename="terminal")]
    pub term: Option<String>,

    #[serde(rename="commands")]
    pub commands: Option<Vec<super::Command>>
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            decoration_height: Some(20),
            border_width: Some(2),
            inner_border_width: Some(0),
            border_color: Some(Color::DefaultBorder),
            background_color: Some(Color::DefaultBackground),
            focused_background_color: Some(Color::DefaultFocusedBackground),
            workspaces: {
                let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
                let _ = (1..=9).for_each(|ws| {
                    workspaces.insert(ws, ws.to_string());
                });
                Some(workspaces)
            },
            term: Some("xterm".to_string()),
            commands: None
        }
    }
}
