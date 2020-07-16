#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::{
            monitor::Monitor, rect::*, window_type::WindowType, windowwrapper::*, WindowState,
        },
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::masks::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
};

impl Reducer<action::MapRequest> for State {
    fn reduce(&mut self, action: action::MapRequest) {
        if let Some(_win) = self.lib.transient_for_hint(action.win) {
            handle_transient_window(self, &action);
            return;
        }
        if self.lib.get_window_type(action.win) == WindowType::Dialog {
            handle_transient_window(self, &action);
            return;
        }

        debug!(
            "MapRequest - window: {} - Parent: {}",
            action.win, action.parent
        );

        if self.lib.get_window_type(action.win) == WindowType::Dock {
            handle_dock(self, &action);
            return;
        }

        if !self.lib.should_be_managed(action.win) {
            return;
        }

        let mon = self
            .monitors
            .get_mut(&self.current_monitor)
            .expect("MapRequest: get_client_mut");

        if mon.contains_window(action.parent) {
            self.lib.map_window(action.win);
            self.lib.take_focus(action.win);
            return;
        }
        let windows = mon.place_window(action.win);
        debug!(
            "Windows in mon before place_window: {:?}",
            mon.get_current_ws()
                .unwrap()
                .clients
                .keys()
                .collect::<Vec<&Window>>()
        );
        let window_amount = windows.len();
        for (win, rect) in windows.into_iter() {
            match mon.remove_window(win) {
                Some(ww) => {
                    if ww.current_state == WindowState::Maximized
                        || ww.current_state == WindowState::Monocle
                    {
                        mon.add_window(
                            win,
                            WindowWrapper {
                                window_rect: rect,
                                previous_state: ww.current_state,
                                current_state: WindowState::Free,
                                ..ww
                            },
                        )
                    } else {
                        mon.add_window(
                            win,
                            WindowWrapper {
                                window_rect: rect,
                                current_state: WindowState::Free,
                                ..ww
                            },
                        );
                    };
                }
                None => {
                    if win == action.win {
                        debug!("Mapping window not already in mon");
                        let ww = if window_amount == 1
                            && mon.get_current_layout().unwrap() != LayoutTag::Floating
                        {
                            let mut ww = WindowWrapper::new(action.win, rect, false);
                            ww.previous_state = WindowState::Maximized;
                            ww.current_state = WindowState::Maximized;
                            ww
                        } else {
                            WindowWrapper::new(action.win, rect, false)
                        };

                        self.focus_w = action.win;
                        mon.add_window(action.win, ww);
                        mon.get_current_ws_mut().unwrap().focus_w = self.focus_w;
                    }
                }
            };
        } //);
        debug!("Windows in mon after place_window:\n");
        mon.get_current_ws()
            .unwrap()
            .clients
            .iter()
            .for_each(|(key,_)| {
                debug!("{}", key);
            });
    }
}

fn handle_transient_window(state: &mut State, action: &action::MapRequest) {
    debug!(
        "Window type: {} is transient",
        state.lib.get_window_type(action.win).get_name()
    );
    let trans_size = {
        let attr = state.lib.get_window_attributes(action.win);
        Size {
            width: attr.width,
            height: attr.height,
        }
    };
    let mon = state
        .monitors
        .get_mut(&state.current_monitor)
        .expect("Shit went down in map_request");

    let pos = match mon.get_client(action.parent) {
        Some(win) => {
            let (pos, size) = (win.get_position(), win.get_size());
            Position {
                x: pos.x + (size.width / 2) - (trans_size.width / 2) as i32,
                y: pos.y + (size.height / 2) - (trans_size.height / 2) as i32,
            }
        }
        None if action.parent == state.lib.get_root() => {
            let (pos, size) = mon.screen.clone().into();
            pos.translate_relative(
                (size.width / 2) - (trans_size.width / 2) as i32,
                (size.height / 2) - (trans_size.height / 2) as i32,
            )
        }
        None => return,
    };
    mon.add_window(
        action.win,
        WindowWrapper::new(
            action.win,
            Rect::new(
                pos,
                Size {
                    width: trans_size.width as i32,
                    height: trans_size.height as i32,
                },
            ),
            true,
        ),
    );
}

fn handle_dock(state: &mut State, action: &action::MapRequest) {
    if let Some(dock) = state.lib.get_window_strut_array(action.win) {
        debug!("Mapping window is dock!");
        let w_geom = state.lib.get_geometry(action.win);
        let mon = state
            .monitors
            .values_mut()
            .filter(|mon| wm::window_inside_screen(&w_geom, &mon.screen))
            .collect::<Vec<&mut Monitor>>()
            .remove(0);
        mon.set_dock_area(dock);
        state
            .lib
            .select_input(action.win, PointerMotionMask | SubstructureRedirectMask);
        state.lib.map_window(action.win);
    }
}
