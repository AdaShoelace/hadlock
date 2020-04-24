use super::KeyAction;
use crate::xlibwrapper::util::{
    Color,
    keysym_lookup::{ModMask, into_mod}
};
use crate::layout::LayoutTag;
use x11_dl::xlib::{Mod4Mask, ShiftMask};
use serde::{self, Deserialize, Serialize, Deserializer, de};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "superKey", deserialize_with = "super_deserialize")]
    pub super_key: ModMask,

    #[serde(rename = "modKey", deserialize_with = "mod_deserialize")]
    pub mod_key: ModMask,

    #[serde(rename = "decorate", default = "default_decorate")]
    pub decorate: bool,

    #[serde(rename = "decorationHeight", default = "default_decoration_height")]
    pub decoration_height: i32,

    #[serde(rename = "borderWidth", default = "default_border_width")]
    pub border_width: i32,

    #[serde(rename = "innerBorderWidth", default = "default_inner_border_width")]
    pub inner_border_width: i32,

    #[serde(rename = "borderColor", default = "default_border_color")]
    pub border_color: Color,

    #[serde(rename = "backgroundColor", default = "default_background_color")]
    pub background_color: Color,

    #[serde(
        rename = "focusedBackgroundColor",
        default = "default_focused_background_color"
    )]
    pub focused_background_color: Color,

    #[serde(rename = "outerGap", default = "default_outer_gap")]
    pub outer_gap: i32,

    #[serde(rename = "innerGap", default = "default_inner_gap")]
    pub inner_gap: i32,

    #[serde(rename = "smartGaps", default = "default_smart_gaps")]
    pub smart_gaps: bool,

    #[serde(rename = "defaultLayout", default = "default_layout")]
    pub default_layout: LayoutTag,

    #[serde(rename = "keyBindings", default = "default_key_bindings")]
    pub key_bindings: Vec<KeyAction>,

    #[serde(rename = "workspaces", default = "default_workspaces")]
    pub workspaces: BTreeMap<u8, String>,

    #[serde(rename = "terminal", default = "default_terminal")]
    pub term: String,

    #[serde(rename = "commands", default = "default_commands")]
    pub commands: Vec<super::Command>,
}

fn super_deserialize<'de, D>(desierializer: D) -> Result<ModMask, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(desierializer)?;

    let ret = into_mod(&s);
    if ret != 0 {
        debug!("ControlMask: {}, super_key: {}", x11_dl::xlib::ControlMask, ret);
        Ok(ret)
    } else {
        Err(de::Error::custom(format!(
                    "{} is not a valid key", s
        )))
    }
}

fn mod_deserialize<'de, D>(desierializer: D) -> Result<ModMask, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(desierializer)?;
    let ret = into_mod(&s);
    debug!("ShiftMask: {}, super_key: {}", x11_dl::xlib::ShiftMask, ret);
    Ok(ret)
}

fn default_decorate() -> bool {
    false
}

fn default_decoration_height() -> i32 {
    20
}

fn default_border_width() -> i32 {
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

fn default_outer_gap() -> i32 {
    0
}

fn default_inner_gap() -> i32 {
    0
}

fn default_smart_gaps() -> bool {
    false
}

fn default_layout() -> LayoutTag {
    LayoutTag::Floating
}

fn default_key_bindings() -> Vec<KeyAction> {
    vec![]
}

fn default_workspaces() -> BTreeMap<u8, String> {
    let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
    (1..=9).for_each(|ws| {
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
            super_key: Mod4Mask,
            mod_key: ShiftMask,
            decorate: default_decorate(),
            decoration_height: default_decoration_height(),
            border_width: default_border_width(),
            inner_border_width: default_inner_border_width(),
            border_color: default_border_color(),
            background_color: default_background_color(),
            focused_background_color: default_focused_background_color(),
            outer_gap: default_outer_gap(),
            inner_gap: default_inner_gap(),
            smart_gaps: default_smart_gaps(),
            default_layout: default_layout(),
            key_bindings: default_key_bindings(),
            workspaces: {
                let mut workspaces: BTreeMap<u8, String> = BTreeMap::new();
                (1..=9).for_each(|ws| {
                    workspaces.insert(ws, ws.to_string());
                });
                workspaces
            },
            term: "xterm".to_string(),
            commands: vec![],
        }
    }
}
