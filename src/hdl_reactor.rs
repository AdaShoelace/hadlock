use {
    crate::config::CONFIG,
    crate::models::{windowwrapper::*, HandleState, WindowState},
    crate::state::*,
    crate::{
        xlibwrapper::core::XlibWrapper,
        xlibwrapper::xlibmodels::*,
        xlibwrapper::{masks::*, util::*},
    },
    reducer::*,
    std::rc::Rc,
    std::sync::mpsc::Sender,
};

pub struct HdlReactor {
    lib: Rc<XlibWrapper>,
    tx: Sender<()>,
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
                    mon.handle_state.replace(HandleState::Handled);
                }
                _ => (),
            }
            mon.workspaces.values().for_each(|ws| {
                ws.clients.iter().for_each(|(key, val)| {
                    let handle_state = *val.handle_state.borrow();
                    match handle_state {
                        HandleState::New => {
                            self.lib.add_to_save_set(*key);
                            self.lib.add_to_root_net_client_list(*key);
                            debug!("BorderWidth on new: {}", CONFIG.border_width);
                            self.lib.set_border_width(*key, CONFIG.border_width as u32);
                            self.lib.move_window(*key, val.get_position());
                            let old_size = val.get_size();
                            let new_size = Size {
                                width: old_size.width - 2 * CONFIG.border_width,
                                height: old_size.height - 2 * CONFIG.border_width,
                            };
                            self.lib.resize_window(*key, new_size);
                            self.subscribe_to_events(*key);
                            self.lib.map_window(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Map => {
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
                        HandleState::Center => {
                            self.lib.move_window(*key, val.get_position());
                            self.lib.resize_window(*key, val.get_size());
                            self.lib.raise_window(*key);
                            self.set_focus(*key, val);
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
                            debug!("Unfocus");
                            self.unset_focus(*key, &val);
                            val.handle_state.replace(HandleState::Handled);
                            self.tx.send(());
                        }
                        HandleState::Shift => {
                            self.lib.move_window(*key, val.get_position());
                            self.lib.resize_window(*key, val.get_size());
                            self.lib.center_cursor(*key);
                            self.set_focus(*key, val);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::Maximize | HandleState::Monocle => {
                            self.lib.move_window(*key, val.get_position());
                            self.lib.resize_window(*key, val.get_size());
                            self.set_focus(*key, &val);
                            self.lib.set_border_width(*key, 0);
                            self.lib.raise_window(*key);
                            self.lib.center_cursor(*key);
                            val.handle_state.replace(HandleState::Handled);
                        }
                        HandleState::MaximizeRestore | HandleState::MonocleRestore => {
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
                self.lib.flush();
            });
        });
    }
}
impl HdlReactor {
    pub fn new(lib: Rc<XlibWrapper>, tx: Sender<()>) -> Self {
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
        self.grab_buttons(focus);
        self.lib.sync(false);
        self.grab_keys(focus);
        self.lib.sync(false);
        //self.lib.take_focus(focus);
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
        self.lib.ungrab_keys(w);
        self.lib.sync(false);
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
