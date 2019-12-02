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

#[derive(Debug)]
pub struct Monitor {
    pub id: u32,
    pub screen: Screen,
    workspaces: HashMap<u32, Workspace>,
    dock_area: DockArea,
    current_ws: u32,
}

impl Monitor {
    pub fn new(id: u32, screen: Screen, ws: Workspace) -> Self {

        let (current_ws, workspaces) = {
            let current_ws = ws.tag;
            let mut workspaces = HashMap::default();
            workspaces.insert(current_ws, ws);
            (current_ws, workspaces)
        };

        Self {
            id,
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
        match self.workspaces.get_mut(&self.current_ws) {
            Some(ws) => ws.add_window(w, ww),
            None => {warn!("Monitor: {}, current_ws: {}", self.id, self.current_ws)} //TODO: fekking fix
        }
    }

    pub fn add_window_to_ws(&mut self, w: Window, ww: WindowWrapper, ws: u32) {
        match self.workspaces.get_mut(&ws) {
            Some(ws) => ws.add_window(w, ww),
            None => {
                self.workspaces.insert(ws.into(), Workspace::new(ws));
                let new_ws = self.workspaces.get_mut(&ws).unwrap();
                new_ws.add_window(w, ww);
            }
        }
    }

    pub fn remove_window(&mut self, w: Window) -> WindowWrapper {
        self.workspaces.get_mut(&self.current_ws).expect("monitor: remove_window").remove_window(w)
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.get_client_keys().contains(&&w)
    }

    pub fn contains_ws(&self, ws: u32) -> bool {
        debug!("in contains_ws");
        debug!("{}, monitors ws' :{:?}", ws, self.workspaces.keys().collect::<Vec<&u32>>());
        self.workspaces.contains_key(&ws)
    }

    pub fn get_current_ws_mut(&mut self) -> Option<&mut Workspace> {
        self.workspaces.get_mut(&self.current_ws)
    }

    pub fn get_current_ws(&self) -> Option<&Workspace> {
        self.workspaces.get(&self.current_ws)
    }
    pub fn get_current_ws_tag(&self) -> u32 {
        self.current_ws
    }
    pub fn set_current_ws(&mut self, ws: u32) {
        if let Some(temp_ws) = self.workspaces.get(&self.current_ws) {
            if temp_ws.clients.is_empty() {
                self.workspaces.remove(&self.current_ws);
            }
        }
        if !self.workspaces.contains_key(&ws) {
            self.workspaces.insert(ws.into(), Workspace::new(ws));
        }


        self.current_ws = ws;
    }

    pub fn get_active_ws_tags(&self) -> Vec<u32> {
        self.workspaces
            .keys()
            .map(|key| *key)
            .collect::<Vec<u32>>()
    }


    pub fn get_current_windows(&self) -> Vec<Window> {
        match self.get_current_ws() {
            Some(ws) => {
                ws
                    .clients
                    .keys()
                    .map(|x| *x)
                    .collect::<Vec<Window>>()
            },
            None => vec![]
        }
    }

    pub fn get_client_keys(&self) -> Vec<Window> {
        let windows = self.workspaces
            .values()
            .map(|x| x.clients.keys().collect::<Vec<&Window>>())
            .flatten()
            .map(|x| *x)
            .collect::<Vec<Window>>();
        windows
    }

    pub fn get_client_mut(&mut self, w: Window) -> Option<&mut WindowWrapper> {
        self.workspaces.get_mut(&self.current_ws)?.clients.get_mut(&w)
    }

    pub fn get_client(&self, w: Window) -> Option<&WindowWrapper> {
        self.workspaces.get(&self.current_ws)?.clients.get(&w)
    }

    pub fn place_window(&mut self, w: Window) -> (Size, Position) {
        let ww = self.get_client(w).expect("monitor: place_window 1").clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().expect("monitor: place_window 2").layout.place_window(&dock_area.clone(), &screen.clone(), w, &ww)
    }

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) -> (Position, Position) {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().expect("monitor: move_window").layout.move_window(&screen, &dock_area, w, x, y)
    }

    pub fn resize_window(&mut self, w: Window, width: u32, height: u32) -> (Size, Size) {
        let ww = self.get_client(w).expect("monitor: resize_window 1").clone();
        self.get_current_ws_mut().expect("monitor: resize_window 2").layout.resize_window(&ww, w, width, height)
    }

    pub fn maximize(&mut self, w: Window) -> Size {
        let ww = self.get_client(w).expect("monitor: maximize 1").clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().expect("monitor: maximize 2").layout.maximize(&screen, &dock_area, &ww, w)
    }

    pub fn shift_window(&mut self, w: Window, direction: Direction) -> (Position, Size) {
        let ww = self.get_client(w).expect("monitor: shift_window 1").clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut().expect("monitor: shift_window 2").layout.shift_window(&screen, &ww, &dock_area, w, direction)
    }
}

impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.current_ws)
    }
}


