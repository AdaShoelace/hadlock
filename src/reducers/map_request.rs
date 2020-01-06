#![allow(unused_imports)]
use {
    crate::{
        wm,
        models::{
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
            monitor::Monitor,
            HandleState
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    reducer::*,
    std::cell::RefCell,
};


impl Reducer<action::MapRequest> for State {
    fn reduce(&mut self, action: action::MapRequest) {
        debug!("MapRequest");
        if self.lib.get_window_type(action.win) == WindowType::Dock {
            match self.lib.get_window_strut_array(action.win) {
                Some(dock) => {
                    let w_geom = self.lib.get_geometry(action.win);
                    let mon = self.monitors
                        .values_mut()
                        .filter(|mon| wm::window_inside_screen(&w_geom, &mon.screen))
                        .collect::<Vec<&mut Monitor>>().remove(0);
                    mon.set_dock_area(dock);
                    self.lib.map_window(action.win);
                }
                None => {
                    return
                }
            }
        }

        let mon = self.monitors.get_mut(&self.current_monitor).expect("MapRequest: get_client_mut");
        if mon.contains_window(action.win) {
            let ww = mon.remove_window(action.win);
            mon.add_window(action.win, WindowWrapper { handle_state: HandleState::Map.into() , ..ww });
        } else {
            if self.lib.should_be_managed(action.win) {
                let (size, pos) = mon.place_window(action.win);
                mon.add_window(action.win, WindowWrapper::new(action.win, Rect::new(pos, size)));
            } else {
                self.lib.map_window(action.win);
            }
        }

    }
}

