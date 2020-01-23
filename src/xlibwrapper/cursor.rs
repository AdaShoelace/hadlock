use x11_dl::xlib;

pub struct Cursor {
    pub normal_cursor: u64,
    pub move_cursor: u64,
}

const NORMAL: u32 = 68;
const MOVE: u32 = 52;

impl Cursor {
    pub fn new(xlib: &xlib::Xlib, disp: *mut xlib::Display) -> Self {
        unsafe {
            Cursor {
                normal_cursor: (xlib.XCreateFontCursor)(disp, NORMAL),
                move_cursor: (xlib.XCreateFontCursor)(disp, MOVE),
            }
        }
    }
}
