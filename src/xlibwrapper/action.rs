#![allow(unused_variables, dead_code)]

use super::xlibmodels::*;

pub struct ConfigurationNotification{
    pub win: Window
}

pub struct ConfigurationRequest{
    pub win: Window, 
    pub win_changes: WindowChanges, 
    pub value_mask: u64
}

pub struct ClientMessageRequest{
    pub win: Window, 
    pub message_type: u64, 
    pub data: Vec<i64>
}

pub struct MapRequest{
    pub win: Window
}

pub struct UnmapNotify{
    pub win: Window
}

pub struct ButtonPress{
    pub win: Window, 
    pub sub_win: Window, 
    pub button: u32, 
    pub x_root: u32, 
    pub y_root: u32, 
    pub state: u32
}

pub struct ButtonRelease{
    pub win: Window, 
    pub sub_win: Window, 
    pub button: u32, 
    pub x_root: u32, 
    pub y_root: u32, 
    pub state: u32
}

pub struct KeyPress{
    pub win: Window, 
    pub state: u32, 
    pub keycode: u32
}

pub struct KeyRelease{
    pub win: Window, 
    pub state: u32, 
    pub keycode: u32
}

pub struct MotionNotify{
    pub win: Window, 
    pub sub_win: Window,
    pub x_root: i32, 
    pub y_root: i32, 
    pub state: u32
}

pub struct EnterNotify{
    pub win: Window, 
    pub sub_win: Window
}

pub struct LeaveNotify{
    pub win: Window
}

pub struct PropertyNotify{
    pub win: Window, 
    pub atom: u64
}

pub struct Expose{
    pub win: Window
}

pub struct DestroyNotify{
    pub win: Window
}

pub struct ButtonReleased;

pub struct UnknownEvent;

