use {
    crate::models::{monitor::Monitor, windowwrapper::WindowWrapper, workspace::Workspace},
    crate::xlibwrapper::{core::*, xlibmodels::*},
    derivative::*,
    std::collections::HashMap,
    std::rc::Rc,
};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct State {
    #[derivative(Debug = "ignore")]
    pub lib: Rc<XlibWrapper>,
    pub windows: HashMap<Window, WindowWrapper>,
    pub focus_w: Window,
    pub monitors: HashMap<MonitorId, Monitor>,
    pub current_monitor: MonitorId,
    pub drag_start_pos: (i32, i32),
    pub drag_start_frame_pos: (i32, i32),
    pub drag_start_frame_size: (u32, u32),
}

impl State {
    pub fn new(lib: Rc<XlibWrapper>) -> Self {
        let focus_w = lib.get_root();
        let monitors = {
            let mut monitors = HashMap::default();
            let _ = lib.get_screens().iter().enumerate().for_each(|(i, val)| {
                info!("Monitors in init: {}", i);
                monitors.insert(
                    i as u32,
                    Monitor::new(i as u32, val.clone(), Workspace::new(i as u32)),
                );
            });
            let mon_count = monitors.iter().count();
            debug!("Monitor on start: {}", mon_count);
            monitors
        };
        Self {
            lib,
            windows: HashMap::default(),
            focus_w,
            monitors,
            current_monitor: 0,
            drag_start_pos: (0, 0),
            drag_start_frame_pos: (0, 0),
            drag_start_frame_size: (0, 0),
        }
    }
}
