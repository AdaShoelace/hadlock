use super::{
    dockarea::DockArea, rect::Rect, screen::Screen, windowwrapper::WindowWrapper,
    workspace::Workspace, Direction,
};
use crate::{
    layout::LayoutTag,
    xlibwrapper::{
        util::{Position, Size},
        xlibmodels::{MonitorId, Window},
    },
};
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Monitor {
    pub id: MonitorId,
    pub screen: Screen,
    pub workspaces: HashMap<u32, Workspace>,
    pub dock_area: DockArea,
    pub current_ws: u32,
    pub mouse_follow: RefCell<bool>,
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
            current_ws,
            mouse_follow: RefCell::new(true),
        }
    }

    pub fn set_dock_area(&mut self, dock_area: DockArea) {
        self.dock_area = dock_area;
    }

    pub fn add_window(&mut self, w: Window, ww: WindowWrapper) {
        match self.workspaces.get_mut(&self.current_ws) {
            Some(ws) => ws.add_window(w, ww),
            None => warn!("Monitor: {}, current_ws: {}", self.id, self.current_ws), //TODO: fekking fix
        }
    }

    pub fn add_window_non_current(&mut self, w: Window, ww: WindowWrapper, ws: u32) {
        match self.workspaces.get_mut(&ws) {
            Some(ws) => ws.add_window(w, ww),
            None => warn!("Monitor: {}, current_ws: {}", self.id, self.current_ws), //TODO: fekking fix
        }
    }

    pub fn remove_window(&mut self, w: Window) -> Option<WindowWrapper> {
        let ret = self
            .workspaces
            .get_mut(&self.current_ws)?
            .remove_window(w)?;
        Some(ret)
    }

    pub fn remove_window_non_current(&mut self, w: Window, ws: u32) -> Option<WindowWrapper> {
        let ret = self.workspaces.get_mut(&ws)?.remove_window(w)?;
        Some(ret)
    }

    pub fn get_ws_by_window(&self, w: Window) -> Option<u32> {
        let mut tag_vec = self
            .workspaces
            .values()
            .filter(|ws| ws.contains_window(w))
            .map(|ws| ws.tag)
            .collect::<Vec<u32>>();
        if !tag_vec.is_empty() {
            Some(tag_vec.remove(0))
        } else {
            None
        }
    }

    pub fn get_newest(&self) -> Option<(&Window, &WindowWrapper)> {
        self.workspaces.get(&self.current_ws)?.get_newest()
    }

    pub fn get_previous(&self, win: Window) -> Option<&WindowWrapper> {
        let ws = self.workspaces.get(&self.current_ws)?;
        match ws.clients.get(&win) {
            Some(ww) => ws.get_previous(ww),
            _ => None,
        }
    }

    pub fn get_next(&self, win: Window) -> Option<&WindowWrapper> {
        let ws = self.workspaces.get(&self.current_ws)?;
        match ws.clients.get(&win) {
            Some(ww) => ws.get_next(ww),
            _ => None,
        }
    }

    /* In current workspace */
    pub fn swap_window<F>(&mut self, win: Window, mut f: F) -> Option<()>
    where
        F: FnMut(&Monitor, WindowWrapper) -> WindowWrapper + Sized,
    {
        let old_ww = self
            .workspaces
            .get_mut(&self.current_ws)?
            .remove_window(win)?;

        let new_ww = f(&self, old_ww);

        self.add_window(win, new_ww);
        Some(())
    }

    pub fn contains_window(&self, w: Window) -> bool {
        self.get_client_keys().contains(&&w)
    }

    pub fn contains_ws(&self, ws: u32) -> bool {
        debug!("in contains_ws");
        debug!(
            "{}, monitors ws' :{:?}",
            ws,
            self.workspaces.keys().collect::<Vec<&u32>>()
        );
        self.workspaces.contains_key(&ws)
    }

    pub fn get_current_ws_mut(&mut self) -> Option<&mut Workspace> {
        self.workspaces.get_mut(&self.current_ws)
    }

    pub fn get_current_ws(&self) -> Option<&Workspace> {
        self.workspaces.get(&self.current_ws)
    }

    pub fn get_current_layout(&self) -> Option<LayoutTag> {
        let ret = self.get_current_ws()?.get_current_layout();
        Some(ret)
    }

    pub fn remove_ws(&mut self, ws: u32) -> Option<Workspace> {
        self.workspaces.remove(&ws)
    }

    pub fn add_ws(&mut self, ws: Workspace) {
        self.workspaces.insert(ws.tag, ws);
    }

    pub fn swap_ws<F>(&mut self, ws: u32, mut f: F) -> Option<()>
    where
        F: FnMut(&Monitor, Workspace) -> Workspace + Sized,
    {
        let old_ws = self.remove_ws(ws)?;

        let new_ws = f(&self, old_ws);

        self.add_ws(new_ws);
        Some(())
    }

    pub fn get_client_keys(&self) -> Vec<Window> {
        self.workspaces
            .values()
            .map(|x| x.clients.keys().collect::<Vec<&Window>>())
            .flatten()
            .copied()
            .collect::<Vec<Window>>()
    }

    pub fn get_client_mut(&mut self, w: Window) -> Option<&mut WindowWrapper> {
        self.workspaces
            .get_mut(&self.current_ws)?
            .clients
            .get_mut(&w)
    }

    pub fn get_client(&self, w: Window) -> Option<&WindowWrapper> {
        self.workspaces.get(&self.current_ws)?.clients.get(&w)
    }

    // Layout functions
    pub fn place_window(&mut self, w: Window) -> Vec<(Window, Rect)> {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        let ws = self.get_current_ws_mut().expect("monitor: place_window 2");
        let windows = ws.clients.values().collect::<Vec<&WindowWrapper>>();
        ws.layout.place_window(&dock_area, &screen, w, windows)
    }

    pub fn move_window(&mut self, w: Window, x: i32, y: i32) -> (Position, Position) {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut()
            .expect("monitor: move_window")
            .layout
            .move_window(&screen, &dock_area, w, true, x, y)
    }

    pub fn reorder(&mut self, focus: Window, windows: &[WindowWrapper]) -> Vec<(Window, Rect)> {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut()
            .expect("Monitor: reorder")
            .layout
            .reorder(focus, &screen, &dock_area, windows.to_vec())
    }

    pub fn maximize(&self, w: Window, ww: &WindowWrapper) -> (Position, Size) {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws()
            .expect("monitor: maximize 2")
            .layout
            .maximize(&screen, &dock_area, &ww, w)
    }

    pub fn monocle(&self, w: Window, ww: &WindowWrapper) -> (Position, Size) {
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws()
            .expect("monitor: maximize 2")
            .layout
            .monocle(&screen, &dock_area, &ww, w)
    }

    pub fn shift_window(&mut self, w: Window, direction: Direction) -> Vec<WindowWrapper> {
        let ww = self.get_client(w).expect("monitor: shift_window 1").clone();
        let screen = self.screen.clone();
        let dock_area = self.dock_area.clone();
        self.get_current_ws_mut()
            .expect("monitor: shift_window 2")
            .layout
            .shift_window(&screen, &ww, &dock_area, w, direction)
    }
}

impl std::fmt::Display for Monitor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.current_ws)
    }
}

#[cfg(test)]
mod test {
    use crate::models::{
        monitor::Monitor, rect::*, screen::Screen, windowwrapper::*, workspace::Workspace,
        WindowState,
    };
    use crate::xlibwrapper::util::*;
    use std::time::Instant;

    const ROOT: u64 = 111;

    fn setup_mon(amount_ws: u32) -> Monitor {
        let screen = Screen::new(ROOT, 1920, 1080, 0, 0);
        let mut mon = Monitor::new(0, screen, Workspace::new(0, 0));
        for i in 0..(amount_ws - 1) {
            mon.add_ws(Workspace::new(i + 1, 0))
        }
        mon
    }

    #[test]
    fn add_window() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());

        assert_eq!(Some(&base_ww), mon.get_client(base_ww.window()))
    }

    #[test]
    fn remove_window_pass() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());
        assert_eq!(Some(base_ww.clone()), mon.remove_window(base_ww.window()))
    }

    #[test]
    fn remove_window_fail() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());
        let tested = WindowWrapper {
            window: 12,
            ..base_ww
        };
        assert_eq!(None, mon.remove_window(tested.window()))
    }

    #[test]
    fn get_newest_pass() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());
        let tested_win = 12;
        let tested = WindowWrapper {
            window: tested_win,
            toc: Instant::now(),
            ..base_ww
        };
        mon.add_window(tested.window(), tested.clone());
        assert_eq!(Some((&tested_win, &tested)), mon.get_newest())
    }

    #[test]
    fn get_newest_fail() {
        let mon = setup_mon(1);
        assert_eq!(None, mon.get_newest())
    }

    #[test]
    fn get_previous_pass() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        let tested = base_ww.clone();
        mon.add_window(base_ww.window(), base_ww.clone());
        let second_win = 12;
        let second = WindowWrapper {
            window: second_win,
            toc: Instant::now(),
            ..base_ww
        };
        mon.add_window(second.window(), second);
        assert_eq!(Some(&tested), mon.get_previous(second_win))
    }

    #[test]
    fn get_previous_fail() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        let tested = base_ww.clone();
        mon.add_window(base_ww.window(), base_ww.clone());
        assert_eq!(None, mon.get_previous(tested.window()))
    }

    #[test]
    fn get_next_pass() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        let first = base_ww.clone();
        mon.add_window(base_ww.window(), base_ww.clone());
        let next_win = 12;
        let next = WindowWrapper {
            window: next_win,
            toc: Instant::now(),
            ..base_ww
        };
        mon.add_window(next_win, next.clone());

        let tested_win = next_win + 1;
        let tested = WindowWrapper::new(
            tested_win,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(tested_win + 1, tested);
        assert_eq!(Some(&next), mon.get_next(first.window()))
    }

    #[test]
    fn get_next_fail() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        let tested = base_ww.clone();
        mon.add_window(base_ww.window(), base_ww.clone());
        assert_eq!(None, mon.get_next(tested.window()))
    }

    #[test]
    fn swap_window_pass() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());

        let tested = WindowWrapper {
            current_state: WindowState::Maximized,
            ..base_ww
        };

        mon.swap_window(1, |_mon, ww| WindowWrapper {
            current_state: WindowState::Maximized,
            ..ww
        });

        assert_eq!(Some(&tested), mon.get_client(tested.window()))
    }

    #[test]
    fn swap_window_fail() {
        let mut mon = setup_mon(1);
        let base_ww = WindowWrapper::new(
            1,
            Rect::new(
                Position { x: 0, y: 0 },
                Size {
                    width: 200,
                    height: 200,
                },
            ),
            false,
        );
        mon.add_window(base_ww.window(), base_ww.clone());

        let tested = WindowWrapper {
            window: 10,
            current_state: WindowState::Maximized,
            ..base_ww
        };

        mon.swap_window(10, |_mon, ww| WindowWrapper {
            current_state: WindowState::Maximized,
            ..ww
        });

        assert_eq!(None, mon.get_client(tested.window()))
    }
}
