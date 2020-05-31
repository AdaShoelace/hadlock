#![allow(non_upper_case_globals, dead_code)]
use super::util::keysym_lookup::into_mod;
use crate::config::{KeyAction, CONFIG};
use x11::xlib;

// masks
pub const SubstructureRedirectMask: i64 = xlib::SubstructureRedirectMask;
pub const SubstructureNotifyMask: i64 = xlib::SubstructureNotifyMask;
pub const EnterWindowMask: i64 = xlib::EnterWindowMask;
pub const LeaveWindowMask: i64 = xlib::LeaveWindowMask;
pub const ExposureMask: i64 = xlib::ExposureMask;
pub const StructureNotifyMask: i64 = xlib::StructureNotifyMask;

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

pub const PointerMotionMask: i64 = xlib::PointerMotionMask;

// Mod masks
pub const Shift: u32 = xlib::ShiftMask;
pub const Mod1Mask: u32 = xlib::Mod1Mask;
pub const Mod2Mask: u32 = xlib::Mod2Mask;
pub const Mod3Mask: u32 = xlib::Mod3Mask;
pub const Mod4Mask: u32 = xlib::Mod4Mask;
pub const Mod5Mask: u32 = xlib::Mod5Mask;
pub const AnyModifier: u32 = xlib::AnyModifier;

// Modes
pub const GrabModeAsync: i32 = xlib::GrabModeAsync;

pub const RevertToParent: i32 = xlib::RevertToParent;
pub const RevertToNone: i32 = xlib::RevertToNone;
pub const RevertToPointerRoot: i32 = xlib::RevertToPointerRoot;

pub const CurrentTime: u64 = xlib::CurrentTime;

pub const FocusChangeMask: i64 = xlib::FocusChangeMask;
pub const PropertyChangeMask: i64 = xlib::PropertyChangeMask;

pub fn mod_masks() -> u32 {
    let ret = CONFIG
        .key_bindings
        .iter()
        .filter(|binding| {
            if let KeyAction {
                mod_key: Some(_mod_key),
                ..
            } = binding
            {
                true
            } else {
                false
            }
        })
        .cloned()
        .map(|binding| binding.mod_key.unwrap())
        .map(|mod_key| into_mod(&mod_key))
        .fold(0, |acc, mod_key| acc | mod_key);
    debug!("mod keys: {:b}", ret);
    ret
}

pub fn mod_masks_vec() -> Vec<u32> {
    let mut ret = CONFIG
        .key_bindings
        .iter()
        .filter(|binding| binding.mod_key.is_some())
        .cloned()
        .map(|binding| binding.mod_key.unwrap())
        .map(|mod_key| into_mod(&mod_key))
        .collect::<Vec<u32>>();
    ret.dedup();
    ret.push(0);
    debug!("mod_masks: {:?}", ret);
    ret
}
