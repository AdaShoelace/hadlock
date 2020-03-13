#![allow(dead_code)]
use indexmap::IndexMap;

use crate::{
    layout::{column_master, floating, Layout, LayoutTag},
    models::windowwrapper::WindowWrapper,
    xlibwrapper::xlibmodels::Window,
};

#[derive(Debug)]
pub struct Workspace {
    pub tag: u32,
    pub clients: IndexMap<Window, WindowWrapper>,
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

    pub fn get_current_layout(&self) -> LayoutTag {
        self.current_tag
    }

    pub fn circulate_layout(&mut self) {
        let index = self
            .available_layouts
            .iter()
            .position(|lt| self.current_tag == *lt)
            .unwrap()
            + 1;

        let index = index % self.available_layouts.len();
        self.current_tag = self.available_layouts[index];
        self.layout = match self.current_tag {
            LayoutTag::Floating => Box::new(floating::Floating::default()),
            LayoutTag::ColumnMaster => Box::new(column_master::ColumnMaster::default()),
        }
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.clients.contains_key(&w)
    }

    pub fn add_window(&mut self, w: Window, ww: WindowWrapper) {
        self.clients.insert(w, ww);
        self.clients.sort_by(|_ka, va, _kb, vb| va.toc.cmp(&vb.toc));
    }

    pub fn remove_window(&mut self, w: Window) -> Option<WindowWrapper> {
        let ret = self.clients.remove(&w);
        self.clients.sort_by(|_ka, va, _kb, vb| va.toc.cmp(&vb.toc));
        ret
    }

    pub fn get_newest(&self) -> Option<(&Window, &WindowWrapper)> {
        self.clients.iter().last()
    }

    pub fn get_newest_mut(&mut self) -> Option<(&Window, &mut WindowWrapper)> {
        self.clients.iter_mut().last()
    }
}

impl PartialEq for Workspace {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl Eq for Workspace {}
