#![allow(unused_variables, dead_code, deprecated)]
use super::util::*;
use x11_dl::xlib;

pub(crate) type Mask = i64;
pub(crate) type Window = xlib::Window;
pub(crate) type Display = xlib::Display;
pub(crate) type Drawable = xlib::Drawable;
pub(crate) type Time = xlib::Time;
pub(crate) type KeyCode = xlib::KeyCode;
pub(crate) type MonitorId = u32;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct WindowChanges {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub sibling: Window,
    pub stack_mode: i32,
}

pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct WindowAttributes<'a> {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub border_width: i32,
    pub depth: i32,
    pub visual: &'a mut xlib::Visual,
    pub root: Window,
    pub class: i32,
    pub bit_gravity: i32,
    pub win_gravity: i32,
    pub backing_store: i32,
    pub backing_planes: u64,
    pub backing_pixel: u64,
    pub save_under: bool,
    pub colormap: xlib::Colormap,
    pub map_installed: bool,
    pub map_state: i32,
    pub all_event_masks: i64,
    pub your_event_mask: i64,
    pub do_not_propagate_mask: i64,
    pub override_redirect: bool,
    pub screen: &'a mut xlib::Screen,
}

impl<'a> From<xlib::XWindowAttributes> for WindowAttributes<'a> {
    fn from(attr: xlib::XWindowAttributes) -> Self {
        unsafe {
            Self {
                x: attr.x,
                y: attr.y,
                width: attr.width,
                height: attr.height,
                border_width: attr.border_width,
                depth: attr.depth,
                visual: &mut *attr.visual,
                root: attr.root,
                class: attr.class,
                bit_gravity: attr.bit_gravity,
                win_gravity: attr.win_gravity,
                backing_store: attr.backing_store,
                backing_planes: attr.backing_planes,
                backing_pixel: attr.backing_pixel,
                save_under: from_c_bool(attr.save_under),
                colormap: attr.colormap,
                map_installed: from_c_bool(attr.map_installed),
                map_state: attr.map_state,
                all_event_masks: attr.all_event_masks,
                your_event_mask: attr.your_event_mask,
                do_not_propagate_mask: attr.do_not_propagate_mask,
                override_redirect: from_c_bool(attr.override_redirect),
                screen: &mut *attr.screen,
            }
        }
    }
}

impl<'a> Into<xlib::XWindowAttributes> for WindowAttributes<'a> {
    fn into(self) -> xlib::XWindowAttributes {
        unsafe {
            let mut ret: xlib::XWindowAttributes = std::mem::uninitialized();
            ret.x = self.x;
            ret.y = self.y;
            ret.width = self.width;
            ret.height = self.height;
            ret.border_width = self.border_width;
            ret.depth = self.depth;
            ret.visual = &mut *self.visual;
            ret.root = self.root;
            ret.class = self.class;
            ret.bit_gravity = self.bit_gravity;
            ret.win_gravity = self.win_gravity;
            ret.backing_store = self.backing_store;
            ret.backing_planes = self.backing_planes;
            ret.backing_pixel = self.backing_pixel;
            ret.save_under = to_c_bool(self.save_under);
            ret.colormap = self.colormap;
            ret.map_installed = to_c_bool(self.map_installed);
            ret.map_state = self.map_state;
            ret.all_event_masks = self.all_event_masks;
            ret.your_event_mask = self.your_event_mask;
            ret.do_not_propagate_mask = self.do_not_propagate_mask;
            ret.override_redirect = to_c_bool(self.override_redirect);
            ret.screen = &mut *self.screen;
            ret
        }
    }
}
