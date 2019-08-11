use super::xlibmodels::*;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EventType {
    ConfigurationNotification,
    ConfigurationRequest,
    MapRequest,
    ButtonPress,
    ButtonRelease,
    KeyPress,
    MotionNotify,
    EnterNotify,
    LeaveNotify,
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
    MapRequest(Window),
    ButtonPress(Window, Window, u32, u32, u32, u32),
    ButtonRelease(Window, Window, u32, u32, u32, u32),
    KeyPress(Window, u32, u32),
    MotionNotify(Window, i32, i32, u32),
    EnterNotify(Window, Window),
    LeaveNotify(Window),
    Expose(Window),
    DestroyWindow(Window),
    ButtonReleased,
    UnknownEvent
}
