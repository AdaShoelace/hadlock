use x11::xlib;

// input masks
pub const SubstructureRedirectMask: i64 = xlib::SubstructureRedirectMask;
pub const SubstructureNotifyMask: i64 = xlib::SubstructureNotifyMask;

// Mouse buttons
pub const Button1: u32 = xlib::Button1;
pub const Button2: u32 = xlib::Button2;
pub const Button3: u32 = xlib::Button3;
pub const Button4: u32 = xlib::Button4;
pub const Button5: u32 = xlib::Button5;

// Mouse button masks
pub const ButtonPressMask: i64 = xlib::ButtonPressMask;
pub const ButtonReleaseMask: i64 = xlib::ButtonReleaseMask;
pub const ButtonMotionMask: i64 = xlib::ButtonMotionMask;
pub const Button1Mask: u32 = xlib::Button1Mask;
pub const Button2Mask: u32 = xlib::Button2Mask;
pub const Button3Mask: u32 = xlib::Button3Mask;
pub const Button4Mask: u32 = xlib::Button4Mask;
pub const Button5Mask: u32 = xlib::Button5Mask;

// Mod masks
pub const Shift: u32 = xlib::ShiftMask;
pub const Mod1Mask: u32 = xlib::Mod1Mask;
pub const Mod2Mask: u32 = xlib::Mod2Mask;
pub const Mod3Mask: u32 = xlib::Mod3Mask;
pub const Mod4Mask: u32 = xlib::Mod4Mask;
pub const Mod5Mask: u32 = xlib::Mod5Mask;

// Modes
pub const GrabModeAsync: i32 = xlib::GrabModeAsync;

pub const RevertToParent: i32 = xlib::RevertToParent;
pub const RevertToNone: i32 = xlib::RevertToNone;
pub const RevertToPointerRoot: i32 = xlib::RevertToPointerRoot;

