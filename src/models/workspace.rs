use std::{
    collections::HashSet,
};

use crate::{
    xlibwrapper::{
        xlibmodels::Window,
    },
    models::{
        screen::Screen,
    },
    layout::{
        Layout,
        floating,
    },
};

pub struct Workspace {
    pub tag: u32,
    pub screen: Screen,
    pub windows: HashSet<Window>,
    pub layout: Box<dyn Layout>,
}

impl Workspace {
    pub fn new(tag: u32, screen: Screen) -> Self {
        Self {
            tag,
            screen,
            windows: HashSet::default(),
            layout: Box::new(floating::Floating)
        }
    }

    pub fn add_window(&mut self, w: Window) {
        self.windows.insert(w);
    }

    pub fn remove_window(&mut self, w: Window) {
        self.windows.remove(&w);
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Eq for Workspace {}
