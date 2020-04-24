use {
    crate::config::CONFIG,
    crate::models::{internal_action::InternalAction, windowwrapper::*, HandleState, WindowState},
    crate::state::*,
    crate::{
        xlibwrapper::xlibmodels::*,
        xlibwrapper::DisplayServer,
        xlibwrapper::{masks::*, util::*},
    },
    reducer::*,
    std::rc::Rc,
    std::sync::mpsc::Sender,
};

pub struct HdlReactor {
    lib: Box<Rc<dyn DisplayServer>>,
    tx: Sender<InternalAction>,
}

impl Reactor<State> for HdlReactor {
    type Output = ();

    fn react(&self, state: &State) {
        //debug!("{:#?}", state);

        state.monitors.values().for_each(|mon| {
            let handle_state = *mon.handle_state.borrow();
            match handle_state {
                HandleState::Focus => {
                    debug!("Setting current monitor to: {}", state.current_monitor);
                    self.lib.update_desktops(mon.current_ws, None);
                    if *mon.mouse_follow.borrow() {
                        state.lib.move_cursor(Position {
                            x: mon.screen.x + mon.screen.width / 2,
                            y: mon.screen.y + mon.screen.height / 2,
                        });
                        mon.mouse_follow.replace(false);
                    }
                    mon.handle_state.replace(HandleState::Handled);
                }
                HandleState::UpdateLayout => {
                    debug!("layout shall be updated");
                    let _ = self.tx.send(InternalAction::UpdateLayout);
                }
                _ => (),
            }

            mon.workspaces.iter().for_each(|(_key, ws)| {
                //debug!("ws {} has len: {}", key, ws.clients.len());
                ws.clients.iter().for_each(|(key, val)| {
                    if mon.id != state.current_monitor {
                        val.handle_state.replace_with(|old| {
                            let mut handle_state = vec![HandleState::Unfocus];
                            old.append(&mut handle_state);
                            old.to_vec()
                        });
                    }

                    if val.is_trans {
                        self.lib.raise_window(*key);
                    }
                    let mut set_handled = false;
                    let handle_state = val.handle_state.clone();
                    //debug!("window: {}, handle_state: {:?}", *key, handle_state);
                    handle_state
                        .into_inner()
                        .iter()
                        .for_each(|handle_state| match handle_state {
                            HandleState::New => {
                                self.lib.add_to_save_set(*key);
                                self.lib.add_to_root_net_client_list(*key);
                                self.lib.set_border_width(*key, CONFIG.border_width as u32);
                                self.lib.set_border_color(*key, CONFIG.background_color);
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.subscribe_to_events(*key);
                                self.lib.map_window(*key);
                                set_handled = true;
                            }
                            HandleState::Map => {
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.lib.map_window(*key);
                                set_handled = true;
                            }
                            HandleState::Unmap => {
                                self.lib.unmap_window(*key);
                                set_handled = true;
                            }
                            HandleState::Move => {
                                self.lib.move_window(*key, val.get_position());
                                set_handled = true;
                            }
                            HandleState::Center => {
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.lib.raise_window(*key);
                                self.set_focus(*key, val);
                                set_handled = true;
                            }
                            HandleState::Resize => {
                                self.lib.resize_window(*key, val.get_size());
                                set_handled = true;
                            }
                            HandleState::Focus => {
                                self.set_focus(*key, &val);
                                set_handled = true;
                            }
                            HandleState::Unfocus => {
                                self.unset_focus(*key, &val);
                                debug!("Unfocusing: {}", key);
                                set_handled = true;
                                //let _ = self.tx.send(InternalAction::Focus);
                            }
                            HandleState::Shift => {
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.lib.center_cursor(*key);
                                self.set_focus(*key, val);
                                set_handled = true;
                            }
                            HandleState::Maximize | HandleState::Monocle => {
                                debug!("Maximise: {}", key);
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.set_focus(*key, &val);
                                self.lib.set_border_width(*key, 0);
                                self.lib.raise_window(*key);
                                set_handled = true;
                            }
                            HandleState::MaximizeRestore | HandleState::MonocleRestore => {
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                if ws.clients.len() > 1  {
                                    self.lib.set_border_width(*key, CONFIG.border_width as u32);
                                    self.lib.set_border_color(*key, CONFIG.background_color);
                                }
                                set_handled = true;
                            }
                            HandleState::Destroy => {
                                let windows = state
                                    .monitors
                                    .get(&state.current_monitor)
                                    .expect("HdlReactor - Destroy")
                                    .get_current_windows();
                                self.kill_window(*key, windows);

                                if let Some(ww) = mon.get_previous(*key) {
                                    let _ =
                                        self.tx.send(InternalAction::FocusSpecific(ww.window()));
                                }

                                if mon.get_newest().is_none() {
                                    let _ = self.tx.send(InternalAction::Focus);
                                }
                            }
                            _ => (),
                        });
                    if set_handled {
                        val.handle_state.replace(HandleState::Handled.into());
                    }
                });
                self.lib.flush();
            });
        });
    }
}
impl HdlReactor {
    pub fn new(lib: Box<Rc<dyn DisplayServer>>, tx: Sender<InternalAction>) -> Self {
        Self { lib, tx }
    }

    fn subscribe_to_events(&self, w: Window) {
        self.lib.select_input(
            w,
            SubstructureNotifyMask
            | SubstructureRedirectMask
            | EnterWindowMask
            | LeaveWindowMask
            | FocusChangeMask
            | PropertyChangeMask
            | PointerMotionMask,
        );
        self.lib.flush();
    }

    fn grab_buttons(&self, w: Window) {
        let buttons = vec![Button1, Button3];
        buttons.iter().for_each(|button| {
            self.lib.grab_button(
                *button,
                CONFIG.super_key,
                w,
                false,
                (ButtonPressMask | ButtonReleaseMask | ButtonMotionMask) as u32,
                GrabModeAsync,
                GrabModeAsync,
                0,
                0,
            );
        });
    }

    fn grab_keys(&self, w: Window) {
        vec![
            "q", "Left", "Up", "Right", "Down", "Return", "c", "d", "e", "f", "h", "j", "k", "l",
            "m", "r", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
            .iter()
            .map(|key| keysym_lookup::into_keysym(key).expect("Core: no such key"))
            .for_each(|key_sym| self.lib.grab_keys(w, key_sym, CONFIG.super_key | mod_masks()));
    }

    fn set_focus(&self, focus: Window, ww: &WindowWrapper) {
        if focus == self.lib.get_root() {
            return;
        }
        self.grab_buttons(focus);
        self.lib.sync(false);
        self.grab_keys(focus);
        self.lib.sync(false);
        self.lib.take_focus(focus);

        if !(ww.current_state == WindowState::Monocle || ww.current_state == WindowState::Maximized)
        {
            self.lib.set_border_width(focus, CONFIG.border_width as u32);
        }
        self.lib.set_border_color(focus, CONFIG.border_color);
        self.lib.sync(false);
    }

    pub fn unset_focus(&self, w: Window, ww: &WindowWrapper) {
        self.lib.ungrab_all_buttons(w);
        self.lib.sync(false);
        //self.lib.ungrab_keys(w);
        //self.lib.sync(false);
        self.lib.set_border_color(w, CONFIG.background_color);
        self.lib.resize_window(w, ww.get_size());
        self.lib.remove_focus(w);
        self.lib.sync(false);
    }

    pub fn kill_window(&self, w: Window, clients: Vec<Window>) {
        if w == self.lib.get_root() {
            return;
        }

        if self.lib.kill_client(w) {
            self.lib.update_net_client_list(clients);
        }
        info!("Top level windows: {}", self.lib.top_level_window_count());
    }
}
