#![allow(unused_variables, dead_code)]

use super::xlibmodels::*;
use x11_dl::xlib::{
    self,
    XEvent
};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Action {
    ConfigurationNotification{win: Window},
    ConfigurationRequest{win: Window, win_changes: WindowChanges, value_mask: u64},
    ClientMessageRequest{win: Window, message_type: u64, data: Vec<i64>},
    MapRequest{win: Window},
    UnmapNotify{win: Window},
    ButtonPress{win: Window, sub_win: Window, button: u32, x_root: u32, y_root: u32, state: u32},
    ButtonRelease{win: Window, sub_win: Window, button: u32, x_root: u32, y_root: u32, state: u32},
    KeyPress{win: Window, state: u32, keycode: u32},
    KeyRelease{win: Window, state: u32, keycode: u32},
    MotionNotify{win: Window, x_root: i32, y_root: i32, state: u32},
    EnterNotify{win: Window, sub_win: Window},
    LeaveNotify{win: Window},
    PropertyNotify{win: Window, atom: u64},
    Expose{win: Window},
    DestroyNotify{win: Window},
    ButtonReleased,
    UnknownEvent
}

impl From<XEvent> for Action {
    fn from(xevent: XEvent) -> Self {
        match xevent.get_type() {
            xlib::ConfigureRequest => {
                let event = xlib::XConfigureRequestEvent::from(xevent);
                let window_changes = WindowChanges {
                    x: event.x,
                    y: event.y,
                    width: event.width,
                    height: event.height,
                    border_width: event.border_width,
                    sibling: event.above,
                    stack_mode: event.detail
                };
                Self::ConfigurationRequest{win: event.window, win_changes: window_changes, value_mask: event.value_mask}
            },
            xlib::MapRequest => {
                let event = xlib::XMapRequestEvent::from(xevent);
                Self::MapRequest{win: event.window}
            },
            xlib::UnmapNotify => {
                let event = xlib::XUnmapEvent::from(xevent);
                Self::UnmapNotify{win: event.window}
            },
            xlib::ButtonPress => {
                let event = xlib::XButtonEvent::from(xevent);
                Self::ButtonPress{
                    win: event.window,
                    sub_win: event.subwindow,
                    button: event.button,
                    x_root: event.x_root as u32,
                    y_root: event.y_root as u32,
                    state: event.state as u32
                }
            },
            xlib::ButtonRelease => {
                let event = xlib::XButtonEvent::from(xevent);
                Self::ButtonRelease{
                    win: event.window,
                    sub_win: event.subwindow,
                    button: event.button,
                    x_root: event.x_root as u32,
                    y_root: event.y_root as u32,
                    state: event.state as u32
                }
            },
            xlib::KeyPress => {
                let event = xlib::XKeyEvent::from(xevent);
                Self::KeyPress{win: event.window, state: event.state, keycode: event.keycode}
            },
            xlib::KeyRelease => {
                let event = xlib::XKeyEvent::from(xevent);
                Self::KeyRelease{win: event.window, state: event.state, keycode: event.keycode}
            },
            xlib::MotionNotify => {
                let event = xlib::XMotionEvent::from(xevent);
                Self::MotionNotify{
                    win: event.window,
                    x_root: event.x_root,
                    y_root: event.y_root,
                    state: event.state
                }
            },
            xlib::EnterNotify => {
                let event = xlib::XCrossingEvent::from(xevent);
                Self::EnterNotify{win: event.window, sub_win: event.subwindow}
            },
            xlib::LeaveNotify => {
                let event = xlib::XCrossingEvent::from(xevent);
                Self::LeaveNotify{win: event.window}
            },
            xlib::Expose => {
                let event = xlib::XExposeEvent::from(xevent);
                Self::Expose{win: event.window}
            },
            xlib::DestroyNotify => {
                let event = xlib::XDestroyWindowEvent::from(xevent);
                Self::DestroyNotify{win: event.window}
            },
            xlib::PropertyNotify => {
                let event = xlib::XPropertyEvent::from(xevent);
                Self::PropertyNotify{win: event.window, atom: event.atom}
            },
            xlib::ClientMessage => {
                let event = xlib::XClientMessageEvent::from(xevent);
                Self::ClientMessageRequest{
                    win: event.window, 
                    message_type: event.message_type,
                    data: vec![event.data.get_long(0), event.data.get_long(1), event.data.get_long(2)]
                }
            },
            _ => Self::UnknownEvent
        }
    }
}

