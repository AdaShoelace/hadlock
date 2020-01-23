#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
    layout::{floating, Layout},
    models::windowwrapper::WindowWrapper,
    xlibwrapper::xlibmodels::Window,
};

#[derive(Debug)]
pub struct Workspace {
    pub tag: u32,
    pub clients: HashMap<Window, WindowWrapper>,
    pub layout: Box<dyn Layout>,
}

impl Workspace {
    pub fn new(tag: u32) -> Self {
        Self {
            tag,
            clients: HashMap::default(),
            layout: Box::new(floating::Floating),
        }
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.clients.contains_key(&w)
    }

    pub fn add_window(&mut self, w: Window, ww: WindowWrapper) {
        warn!("{} added to desktop: {}", w, self.tag);
        self.clients.insert(w, ww);
    }

    pub fn remove_window(&mut self, w: Window) -> Option<WindowWrapper> {
        self.clients.remove(&w)
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Eq for Workspace {}
