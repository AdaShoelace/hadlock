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
        xlibwrapper::masks::*,
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
                debug!(
                    "Window type: {} is transient",
                    self.lib.get_window_type(action.win).get_name()
                );
                let trans_size = {
                    let attr = self.lib.get_window_attributes(action.win);
                    Size {
                        width: attr.width,
                        height: attr.height,
                    }
                };
                let mon = self
                    .monitors
                    .get_mut(&self.current_monitor)
                    .expect("Shit went down in map_request");

                let pos = match mon.get_client(action.parent) {
                    Some(win) => {
                        let (pos, size) = (win.get_position(), win.get_size());
                        Position {
                            x: pos.x + (size.width / 2) - (trans_size.width / 2) as i32,
                            y: pos.y + (size.height / 2) - (trans_size.height / 2) as i32,
                        }
                    }
                    None if action.parent == self.lib.get_root() => {
                        let screen = mon.screen.clone();
                        let (pos, size) = (
                            Position {
                                x: screen.x,
                                y: screen.y,
                            },
                            Size {
                                width: screen.width as i32,
                                height: screen.height as i32,
                            },
                        );
                        Position {
                            x: pos.x + (size.width / 2) - (trans_size.width / 2) as i32,
                            y: pos.y + (size.height / 2) - (trans_size.height / 2) as i32,
                        }
                    }
                    None => return,
                };
                let ww = WindowWrapper::new(
                    action.win,
                    Rect::new(
                        pos,
                        Size {
                            width: trans_size.width as i32,
                            height: trans_size.height as i32,
                        },
                    ),
                    true,
                );
                mon.add_window(
                    ww.window(),
                    WindowWrapper {
                        handle_state: HandleState::New.into(),
                        ..ww
                    },
                );
                return;
            }
            None => (),
        };
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
                    self.lib.select_input(action.win, PointerMotionMask | SubstructureRedirectMask);
                    self.lib.map_window(action.win);
                    return;
                }
                None => return,
            }
        }

        if !self.lib.should_be_managed(action.win) {
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("MapRequest: get_client_mut");
        if mon.contains_window(action.win) {
            let ww = match mon.remove_window(action.win) {
                Some(ww) => ww,
                None => {
                    let class = self.lib.get_class_hint(action.win);
                    debug!("{:?} not in ws: {}", class, mon.current_ws);
                    return;
                }
            };
            mon.add_window(
                action.win,
                WindowWrapper {
                    handle_state: HandleState::Map.into(),
                    ..ww
                },
            );
            return;
        } else {
            if mon.contains_window(action.parent) {
                self.lib.map_window(action.win);
                return;
            }
            let windows = mon.place_window(action.win);
            //debug!("Place in map_request: {:?}", windows);
            debug!("Windows in mon before place_window: {:?}", mon.get_current_ws().unwrap().clients.keys().collect::<Vec<&Window>>());
            let _ = windows.into_iter().for_each(|(win, rect)| {
                match mon.remove_window(win) {
                    Some(ww) => {
                        let ww = WindowWrapper {
                            window_rect: rect,
                            handle_state: vec![HandleState::Move, HandleState::Resize].into(),
                            ..ww
                        };
                        mon.add_window(win, ww);
                    }
                    None => {
                        let ww = WindowWrapper::new(action.win, rect, false);
                        mon.add_window(action.win, ww);
                    }
                };
            });
            debug!("Windows in mon after place_window: {:?}", mon.get_current_ws().unwrap().clients.keys().collect::<Vec<&Window>>());
        }
    }
}
