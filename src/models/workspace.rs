#![allow(dead_code)]
use std::collections::HashMap;

use crate::{
    layout::{floating, column_master, Layout, LayoutTag},
    models::windowwrapper::WindowWrapper,
    xlibwrapper::xlibmodels::Window,
};

#[derive(Debug)]
pub struct Workspace {
    pub tag: u32,
    pub clients: HashMap<Window, WindowWrapper>,
    pub layout: Box<dyn Layout>,
    available_layouts: Vec<LayoutTag>,
    current_tag: LayoutTag,
}

impl Workspace {
    pub fn new(tag: u32) -> Self {
        Self {
            tag,
            clients: Default::default(),
            layout: Box::new(floating::Floating::default()),
            available_layouts: vec![LayoutTag::Floating, LayoutTag::ColumnMaster],
            current_tag: LayoutTag::Floating,
        }
    }

    pub fn circulate_layout(&mut self) {
        let index = self
            .available_layouts
            .iter()
            .position(|lt| self.current_tag == *lt)
            .unwrap() + 1;
        
        let index = index % self.available_layouts.len();
        self.current_tag = self.available_layouts[index];
        self.layout = match self.current_tag {
            LayoutTag::Floating => Box::new(floating::Floating::default()),
            LayoutTag::ColumnMaster => Box::new(column_master::ColumnMaster::default())
        }
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.clients.contains_key(&w)
    }

    pub fn add_window(&mut self, w: Window, ww: WindowWrapper) {
        //warn!("{} added to desktop: {}", w, self.tag);
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
