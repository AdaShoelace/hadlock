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

impl Reducer<action::MapRequest> for State {
    fn reduce(&mut self, action: action::MapRequest) {
        match self.lib.transient_for_hint(action.win) {
            Some(_win) => {
                self.lib.map_window(action.win);
                return;
            }
            None => (),
        };

        debug!(
            "Window type: {}",
            self.lib.get_window_type(action.win).get_name()
        );

        debug!(
            "MapRequest - window: {} - Parent: {}",
            action.win, action.parent
        );
        if self.lib.get_window_type(action.win) == WindowType::Dock {
            match self.lib.get_window_strut_array(action.win) {
                Some(dock) => {
                    debug!("Mapping window is dock!");
                    let w_geom = self.lib.get_geometry(action.win);
                    let mon = self
                        .monitors
                        .values_mut()
                        .filter(|mon| wm::window_inside_screen(&w_geom, &mon.screen))
                        .collect::<Vec<&mut Monitor>>()
                        .remove(0);
                    mon.set_dock_area(dock);
                    self.lib.map_window(action.win);
                    return;
                }
                None => return,
            }
        }

        if !self.lib.should_be_managed(action.win) {
            //self.lib.map_window(action.win);
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("MapRequest: get_client_mut");
        if mon.contains_window(action.win) {
            if mon.contains_window(action.parent) {
                debug!("Child to existing window");
                self.lib.map_window(action.win);
                return;
            }
            let ww = mon.remove_window(action.win);
            mon.add_window(
                action.win,
                WindowWrapper {
                    handle_state: HandleState::Map.into(),
                    ..ww
                },
            );
            //self.lib.map_window(action.win);
            return;
        } else {
            let (size, pos) = mon.place_window(action.win);
            mon.add_window(
                action.win,
                WindowWrapper::new(action.win, Rect::new(pos, size)),
            );
        }
    }
}
