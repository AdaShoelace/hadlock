use x11_dl::xlib::{Window};
use x11_dl::xinerama::XineramaScreenInfo as XSInfo;
use crate::xlibwrapper::xlibmodels::{WindowAttributes as WinAttr};
use std::convert::From;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Screen {
    pub root: Window,
    pub height: i32,
    pub width: i32,
    pub x: i32,
    pub y: i32
}

impl Screen {
    pub fn new(root: Window, width: i32, height: i32, x: i32, y: i32) -> Self {
        Self {
            root,
            width,
            height,
            x,
            y
        }
    }
}

impl From<&XSInfo> for Screen {
    fn from(screeninfo: &XSInfo) -> Self {
        Self {
            root: 0,
            height: screeninfo.height.into(),
            width: screeninfo.width.into(),
            x: screeninfo.x_org.into(),
            y: screeninfo.y_org.into()
        }
    }
}

impl<'a> From<&WinAttr<'a>> for Screen {
    fn from(root: &WinAttr) -> Self {
        Screen {
            root: root.root,
            height: root.height,
            width: root.width,
            x: root.x,
            y: root.y,
        }
    }
}
