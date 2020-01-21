use crate::xlibwrapper::util::Color;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "decorate", default = "default_decorate")]
    pub decorate: bool,

    #[serde(rename = "decorationHeight", default = "default_decoration_height")]
    pub decoration_height: i32,

    #[serde(rename = "borderWidth", default = "default_decoration_height")]
    pub border_width: i32,

    #[serde(rename = "innerBorderWidth", default = "default_inner_border_width")]
    pub inner_border_width: i32,

    #[serde(rename = "borderColor", default = "default_border_color")]
    pub border_color: Color,

    #[serde(rename = "backgroundColor", default = "default_background_color")]
    pub background_color: Color,

    #[serde(rename = "focusedBackgroundColor", default = "default_focused_background_color")]
    pub focused_background_color: Color,

    #[serde(rename = "workspaces", default = "default_workspaces")]
    pub workspaces: BTreeMap<u8, String>,

    #[serde(rename = "terminal", default = "default_terminal")]
    pub term: String,

    #[serde(rename = "commands", default = "default_commands")]
    pub commands: Vec<super::Command>,
}

fn default_decorate() -> bool {
    false
}

fn default_decoration_height() -> i32 {
    20
}

fn default_border_borde_width() -> i32 {
    2
}

fn default_inner_border_width() -> i32 {
    0
}

fn default_border_color() -> Color {
    Color::DefaultBorder
}

fn default_background_color() -> Color {
    Color::DefaultBackground
}

fn default_focused_background_color() -> Color {
    Color::DefaultFocusedBackground
}

fn default_workspaces() -> BTreeMap<u8, String> {
    let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
    let _ = (1..=9).for_each(|ws| {
        workspaces.insert(ws, ws.to_string());
    });
    workspaces
}

fn default_terminal() -> String {
    "xterm".into()
}

fn default_commands() -> Vec<super::Command> {
    vec![]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            decorate: false,
            decoration_height: 20,
            border_width: 2,
            inner_border_width: 0,
            border_color: Color::DefaultBorder,
            background_color: Color::DefaultBackground,
            focused_background_color: Color::DefaultFocusedBackground,
            workspaces: {
                let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
                let _ = (1..=9).for_each(|ws| {
                    workspaces.insert(ws, ws.to_string());
                });
                workspaces
            },
            term: "xterm".to_string(),
            commands: vec![],
        }
    }
}
