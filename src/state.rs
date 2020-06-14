#![allow(clippy::match_single_binding)]
use {
    crate::models::internal_action,
    crate::models::{monitor::Monitor, windowwrapper::WindowWrapper, workspace::Workspace},
    crate::xlibwrapper::{xlibmodels::*, DisplayServer, util::Position},
    derivative::*,
    std::collections::HashMap,
    std::rc::Rc,
    std::sync::mpsc::Sender,
};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct State {
    #[derivative(Debug = "ignore")]
    pub lib: Box<Rc<dyn DisplayServer>>,
    pub tx: Sender<internal_action::InternalAction>,
    pub windows: HashMap<Window, WindowWrapper>,
    pub focus_w: Window,
    pub monitors: HashMap<MonitorId, Monitor>,
    pub current_monitor: MonitorId,
    pub latest_cursor_pos: Position,
    pub ws_switch: bool,
    pub drag_start_pos: (i32, i32),
    pub drag_start_frame_pos: (i32, i32),
    pub drag_start_frame_size: (u32, u32),
}

impl State {
    pub fn new(
        lib: Box<Rc<dyn DisplayServer>>,
        tx: Sender<internal_action::InternalAction>,
    ) -> Self {
        let focus_w = lib.get_root();
        let monitors = {
            let mut monitors = HashMap::default();
            lib.get_screens().iter().enumerate().for_each(|(i, val)| {
                info!("Monitors in init: {}", i);
                monitors.insert(
                    i as u32,
                    Monitor::new(i as u32, val.clone(), Workspace::new(i as u32, focus_w)),
                );
            });
            let mon_count = monitors.iter().count();
            debug!("Monitor on start: {}", mon_count);
            monitors
        };
        Self {
            lib,
            tx,
            windows: HashMap::default(),
            focus_w,
            monitors,
            current_monitor: 0,
            latest_cursor_pos: Position::new(0,0),
            ws_switch: false,
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
        }
    }
}
