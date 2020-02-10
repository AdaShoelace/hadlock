#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        models::{
            monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, HandleState,
        },
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::UpdateLayout> for State {
    fn reduce(&mut self, action: action::UpdateLayout) {
        let mon = self.monitors.get_mut(&self.current_monitor)
            .expect("UpdateLayout - reducer - monitor - get_mut");
        let windows = mon
            .get_current_ws()
            .expect("UpdateLayout - reducer - monitor - get_current_ws")
            .clients
            .values()
            .map(|x| x.clone())
            .collect::<Vec<WindowWrapper>>()
            .clone();
        if self.focus_w == self.lib.get_root() {
            return
        }
        let mut rects = mon.place_window(self.focus_w);
        windows
            .into_iter()
            .enumerate()
            .map(|(index, win)| {
                debug!("iteration: {}, rect.len: {}", index, rects.len());
                WindowWrapper {
                    window_rect: rects.remove(0).1,
                    handle_state: vec![HandleState::Move, HandleState::Resize].into(),
                    ..win
                }
            })
            .for_each(|win| {
                mon.remove_window(win.window());
                mon.add_window(win.window(), win)
            });
    }
}
