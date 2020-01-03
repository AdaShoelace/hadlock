use {
    crate::config::CONFIG,
    crate::models::{screen::Screen, windowwrapper::*, HandleState, WindowState},
    crate::state::*,
    crate::xlibwrapper::action::*,
    crate::xlibwrapper::core::XlibWrapper,
    crate::xlibwrapper::{action, masks::*, util::*, xlibmodels::*},
    reducer::*,
    std::cell::RefCell,
    std::rc::Rc,
    x11_dl::xlib::{self, XEvent},
};

pub struct HdlReactor {
    lib: Rc<XlibWrapper>,
}

impl Reactor<State> for HdlReactor {
    type Output = ();

    fn react(&self, state: &State) {
        //debug!("{:#?}", state);

        state.monitors.values().for_each(|mon| {
            let handle_state = *mon.handle_state.borrow();
            match handle_state {
                HandleState::Focus => {
                    self.lib.update_desktops(mon.current_ws, None);
                    mon.handle_state.replace(HandleState::Handled);
                },
                _ => ()
            }
            mon.workspaces.values().for_each(|ws| {
                ws.clients.iter().for_each(|(key, val)| {
                    let handle_state = *val.handle_state.borrow();
                    match handle_state {
                        HandleState::New => {
                            self.lib.add_to_save_set(*key);
                            self.lib.add_to_root_net_client_list(*key);
                            self.lib.move_window(*key, val.get_position());
                            self.lib.resize_window(*key, val.get_size());
                            self.subscribe_to_events(*key);
                            self.lib.map_window(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Map => {
                            /*let pos = Position {
                              x: mon.screen.x + val.get_position().x,
                              y: mon.screen.y + val.get_position().y
                              };*/
                            self.lib.move_window(*key, val.get_position());
                            self.lib.map_window(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Unmap => {
                            self.lib.unmap_window(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Move => {
                            self.lib.move_window(*key, val.get_position());
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Resize => {
                            debug!("Resize in reactor");
                            self.lib.resize_window(*key, val.get_size());
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Focus => {
                            debug!("Focus");
                            self.set_focus(*key, &val);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Unfocus => {
                            self.unset_focus(*key, &val);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Shift => {
                            self.lib.move_window(*key, val.get_position());
                            self.lib.resize_window(*key, val.get_size());
                            self.lib.center_cursor(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Maximize
                            | HandleState::Monocle => {
                                self.lib.move_window(*key, val.get_position());
                                self.lib.resize_window(*key, val.get_size());
                                self.set_focus(*key, &val);
                                self.lib.raise_window(*key);
                                self.lib.center_cursor(*key);
                                val.handle_state.replace(HandleState::Handled);
                            },
                            HandleState::MaximizeRestore
                                | HandleState::MonocleRestore => {
                                    self.lib.move_window(*key, val.get_position());
                                    self.lib.resize_window(*key, val.get_size());
                                    self.set_focus(*key, &val);
                                    self.lib.center_cursor(*key);
                                    val.handle_state.replace(HandleState::Handled);
                                }
                        HandleState::Destroy => {
                            let windows = state
                                .monitors
                                .get(&state.current_monitor)
                                .expect("HdlReactor - Destroy")
                                .get_current_windows();
                            self.kill_window(*key, windows);
                        }
                        _ => (),
                    }
                });
                self.lib.sync(false);
            });
        });
    }
}

impl HdlReactor {
    pub fn new(lib: Rc<XlibWrapper>) -> Self {
        Self { lib }
    }

    fn subscribe_to_events(&self, w: Window) {
        self.lib.select_input(
            w,
            EnterWindowMask | LeaveWindowMask | FocusChangeMask | PropertyChangeMask,
        );
        self.lib.sync(false);
    }

    fn grab_buttons(&self, w: Window) {
        let buttons = vec![Button1, Button3];
        buttons.iter().for_each(|button| {
            self.lib.grab_button(
                *button,
                Mod4Mask,
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
        let _keys = vec![
            "q", "Left", "Up", "Right", "Down", "Return", "f", "e", "c", "h", "j", "k", "l", "d",
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
            .iter()
            .map(|key| keysym_lookup::into_keysym(key).expect("Core: no such key"))
            .for_each(|key_sym| self.lib.grab_keys(w, key_sym, Mod4Mask | Shift));
    }

    fn set_focus(&self, focus: Window, ww: &WindowWrapper) {
        if focus == self.lib.get_root() {
            return;
        }
        let size = ww.get_size();
        self.grab_buttons(focus);
        self.lib.sync(false);
        self.grab_keys(focus);
        self.lib.sync(false);
        self.lib.take_focus(focus);
        self.lib.set_border_width(focus, CONFIG.border_width as u32);
        self.lib.set_border_color(focus, CONFIG.border_color);
        self.lib.resize_window(
            focus,
            Size {
                width: size.width - 2 * CONFIG.border_width,
                height: size.height - 2 * CONFIG.border_width,
            },
        );
        self.lib.sync(false);
    }

    pub fn unset_focus(&self, w: Window, ww: &WindowWrapper) {
        self.lib.ungrab_all_buttons(w);
        self.lib.sync(false);
        self.lib.ungrab_keys(w);
        self.lib.sync(false);
        self.lib.set_border_width(w, 0);
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
