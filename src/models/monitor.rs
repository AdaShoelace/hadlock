use crate::xlibwrapper::{
    xlibmodels::Window,
    util::{
        Position,
        Size,
    }
};

use std::collections::HashMap;

use super::{
    workspace::Workspace,
    screen::Screen,
    windowwrapper::WindowWrapper,
    dockarea::DockArea,
    Direction
};

pub struct Monitor {
    pub screen: Screen,
    workspaces: HashMap<u32, Workspace>,
    dock_area: DockArea,
    pub current_ws: u32,
}

impl Monitor {
    pub fn new(screen: Screen, ws: Workspace) -> Self {

        let (current_ws, workspaces) = {
            let current_ws = ws.tag;
            let mut workspaces = HashMap::default();
            workspaces.insert(current_ws, ws);
            (current_ws, workspaces)
        };

        Self {
            screen,
            workspaces,
            dock_area: Default::default(),
            current_ws
        }
    }

    pub fn set_dock_area(&mut self, dock_area: DockArea) {
        self.dock_area = dock_area;
    }

    pub fn add_window(&mut self, w: Window, ww: WindowWrapper) {
        self.workspaces.get_mut(&self.current_ws).unwrap().add_window(w, ww);
    }

    pub fn remove_window(&mut self, w: Window) -> WindowWrapper {
        self.workspaces.get_mut(&self.current_ws).unwrap().remove_window(w)
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.get_client_keys().contains(&&w)
    }

    pub fn get_current_ws_mut(&mut self) -> &mut Workspace {
        self.workspaces.get_mut(&self.current_ws).expect(&format!("Monitor::get_current_ws, no such ws: {}", self.current_ws))
    }

    pub fn get_current_ws(&self) -> &Workspace {
        self.workspaces.get(&self.current_ws).expect(&format!("Monitor::get_current_ws, no such ws: {}", self.current_ws))
    }

    pub fn set_current_ws(&mut self, ws: u32) {
        if !self.workspaces.contains_key(&ws) {
            self.workspaces.insert(ws.into(), Workspace::new(ws));
        }
        self.current_ws = ws;
    }

    pub fn get_current_windows(&self) -> Vec<Window> {
        self.get_current_ws().clients
            .keys()
            .map(|x| *x)
            .collect::<Vec<Window>>()
    }

    pub fn get_client_keys(&self) -> Vec<Window> {
        let windows = self.workspaces
            .values()
            .map(|x| x.clients.keys().collect::<Vec<&Window>>())
            .flat_map(|x| x)
            .map(|x| *x)
            .collect::<Vec<Window>>();
        windows
    }

    pub fn get_client_mut(&mut self, w: Window) -> Option<&mut WindowWrapper> {
        self.workspaces.get_mut(&self.current_ws).unwrap().clients.get_mut(&w)
    }

    pub fn get_client(&self, w: Window) -> Option<&WindowWrapper> {
        self.workspaces.get(&self.current_ws).unwrap().clients.get(&w)
    }

    pub fn place_window(&mut self, w: Window) -> Position {
        let ww = self.get_client(w).unwrap().clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().layout.place_window(&dock_area.clone(), &screen.clone(), w, &ww)
    }

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) -> (Position, Position) {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().layout.move_window(&screen, &dock_area, w, x, y)
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) -> (Size, Size) {
        let ww = self.get_client(w).unwrap().clone();
        self.get_current_ws_mut().layout.resize_window(&ww, w, width, height)
    }

    pub fn maximize(&mut self, w: Window) -> Size {
        let ww = self.get_client(w).unwrap().clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().layout.maximize(&screen, &dock_area, &ww, w)
    }

    pub fn shift_window(&mut self, w: Window, direction: Direction) -> (Position, Size) {
        let ww = self.get_client(w).unwrap().clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().layout.shift_window(&screen, &ww, &dock_area, w, direction)
    }
}


