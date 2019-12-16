#![allow(unused_variables, dead_code)]

use super::xlibmodels::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EventType {
    ConfigurationNotification,
    ConfigurationRequest,
    ClientMessageRequest,
    MapRequest,
    UnmapNotify,
    ButtonPress,
    ButtonRelease,
    KeyPress,
    KeyRelease,
    MotionNotify,
    EnterNotify,
    LeaveNotify,
    PropertyNotify,
    Expose,
    DestroyWindow,
    UnknownEvent
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub payload: Option<EventPayload>
}

impl Event {
    pub fn new(event_type: EventType, payload: Option<EventPayload>) -> Self {
        Self {
            event_type,
            payload
        }
    }
}

#[derive(Debug)]
pub enum EventPayload {
    ConfigurationNotification(Window),
    ConfigurationRequest(Window, WindowChanges, u64),
    ClientMessageRequest(Window, u64, Vec<i64>),
    MapRequest(Window),
    UnmapNotify(Window),
    ButtonPress(Window, Window, u32, u32, u32, u32),
    ButtonRelease(Window, Window, u32, u32, u32, u32),
    KeyPress(Window, u32, u32),
    KeyRelease(Window, u32, u32),
    MotionNotify(Window, i32, i32, u32),
    EnterNotify(Window, Window),
    LeaveNotify(Window),
    PropertyNotify(Window, u64),
    Expose(Window),
    DestroyWindow(Window),
    ButtonReleased,
    UnknownEvent
}
