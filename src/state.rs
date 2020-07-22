use {
    crate::models::{monitor::Monitor, windowwrapper::WindowWrapper, workspace::Workspace},
    crate::xlibwrapper::{util::Position, xlibmodels::*, DisplayServer},
    derivative::*,
    std::collections::HashMap,
    std::rc::Rc,
};

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct State {
    #[derivative(Debug = "ignore")]
    pub lib: Box<Rc<dyn DisplayServer>>,
    pub windows: HashMap<Window, WindowWrapper>,
    pub focus_w: Window,
    pub monitors: HashMap<MonitorId, Monitor>,
    pub hide_space: Position,
    pub current_monitor: MonitorId,
    pub latest_cursor_pos: Position,
    pub ws_switch: bool,
    pub drag_start_pos: (i32, i32),
    pub drag_start_frame_pos: (i32, i32),
    pub drag_start_frame_size: (u32, u32),
}

impl State {
    pub fn new(lib: Box<Rc<dyn DisplayServer>>) -> Self {
        let focus_w = lib.get_root();
        let monitors = {
            let mut monitors = HashMap::default();
            lib.get_screens()
                .into_iter()
                .enumerate()
                .for_each(|(i, val)| {
                    info!("Monitors in init: {}", i);
                    monitors.insert(
                        i as u32,
                        Monitor::new(i as u32, val, Workspace::new(i as u32, focus_w)),
                    );
                });
            let mon_count = monitors.iter().count();
            debug!("Monitor on start: {}", mon_count);
            monitors
        };
        let hide_space = lib
            .get_screens()
            .iter()
            .fold(Position::new(0, 0), |ret_pos, s| {
                ret_pos + Position::new(s.x + s.width, s.y)
            });
        let latest_cursor_pos = lib.pointer_pos(lib.get_root());
        Self {
            lib,
            windows: HashMap::default(),
            focus_w,
            monitors,
            hide_space,
            current_monitor: 0,
            latest_cursor_pos,
            ws_switch: false,
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
        }
    }

    #[allow(dead_code)]
    pub fn workspaces(&self) -> HashMap<u32, &Workspace> {
        self.monitors
            .values()
            .map(|mon| {
                mon.workspaces
                    .iter()
                    .map(|(key, value)| (*key, value))
                    .collect::<Vec<(u32, &Workspace)>>()
            })
            .flatten()
            .collect::<HashMap<u32, &Workspace>>()
    }

    pub fn clients(&self) -> HashMap<Window, &WindowWrapper> {
        let client_vec = self
            .monitors
            .values()
            .map(|mon| &mon.workspaces)
            .flatten()
            .map(|(_, val)| {
                val.clients
                    .values()
                    .map(|ww| ww)
                    .collect::<Vec<&WindowWrapper>>()
            })
            .flatten()
            .collect::<Vec<&WindowWrapper>>();
        let mut ret = HashMap::new();
        for client in client_vec {
            ret.insert(client.window(), client);
        }
        ret
    }
}
