use {
    crate::config::{Key, CONFIG},
    crate::models::{windowwrapper::*, WindowState},
    crate::state::*,
    crate::{
        xlibwrapper::xlibmodels::*,
        xlibwrapper::DisplayServer,
        xlibwrapper::{masks::*, util::*},
    },
    reducer::*,
    std::rc::Rc,
};

pub struct HdlReactor {
    lib: Box<Rc<dyn DisplayServer>>,
    prev_state: State,
}

impl Reactor<State> for HdlReactor {
    type Error = Box<dyn std::error::Error>;

    fn react(&mut self, state: &State) -> Result<(), Self::Error> {
        // Monitors
        if self.prev_state.current_monitor != state.current_monitor {
            let mon = state.monitors.get(&state.current_monitor).ok_or("oops")?;
            self.lib.update_desktops(mon.current_ws, None);
            if *mon.mouse_follow.borrow() {
                if let Some(win) = mon.get_client(state.focus_w) {
                    state.lib.center_cursor(win.window());
                } else {
                    state.lib.move_cursor(Position {
                        x: mon.screen.x + mon.screen.width / 2,
                        y: mon.screen.y + mon.screen.height / 2,
                    });
                }
                mon.mouse_follow.replace(false);
            }
        }

        let mon = state.monitors.get(&state.current_monitor).ok_or("oops")?;
        if self
            .prev_state
            .monitors
            .get(&self.prev_state.current_monitor)
            .unwrap()
            .current_ws
            != mon.current_ws
        {
            self.lib.update_desktops(mon.current_ws, None);
        }

        let num_of_clients = state.clients().len();
        for ww in state.clients().values() {
            let window = ww.window();
            if !self.prev_state.clients().contains_key(&window) {
                debug!("pointer pos on map: {:?}", state.latest_cursor_pos);
                self.lib.add_to_save_set(window);
                self.lib.add_to_root_net_client_list(window);
                if ww.current_state != WindowState::Maximized
                    || ww.current_state != WindowState::Monocle
                {
                    self.lib.set_border_width(window, 0);
                } else {
                    self.lib
                        .set_border_width(window, CONFIG.border_width as u32);
                }
                self.lib.set_border_color(window, CONFIG.background_color);
                self.lib.move_window(window, ww.get_position());
                self.lib.resize_window(window, ww.get_size());
                self.subscribe_to_events(window);
                self.lib.map_window(window);
                self.set_focus(ww.window(), ww);
            } else {
                if let Some(c) = self.prev_state.clients().get(&ww.window()) {
                    if window == state.focus_w && window != self.prev_state.focus_w {
                        self.set_focus(window, ww);
                        self.lib.flush();
                    }
                    if window != state.focus_w {
                        self.unset_focus(window, ww);
                    }
                    if c.get_position() != ww.get_position() {
                        self.lib.move_window(window, ww.get_position());
                    }

                    if c.get_size() != ww.get_size() {
                        self.lib.resize_window(window, ww.get_size());
                    }

                    if ww.is_trans {
                        self.lib.raise_window(window);
                    }
                    if ww.hidden {
                        self.lib.move_window(window, state.hide_space);
                    } else {
                        self.lib.move_window(window, ww.get_position());
                    }
                    if self
                        .prev_state
                        .clients()
                        .get(&window)
                        .unwrap()
                        .current_state
                        != ww.current_state
                    {
                        match (ww.previous_state, ww.current_state) {
                            (_, WindowState::Maximized) | (_, WindowState::Monocle) => {
                                self.lib.set_border_width(window, 0);
                                self.lib.raise_window(window);
                            }
                            (WindowState::Maximized, current) | (WindowState::Monocle, current)
                                if current != WindowState::Maximized
                                    || current != WindowState::Monocle =>
                            {
                                if num_of_clients > 1 {
                                    self.lib
                                        .set_border_width(window, CONFIG.border_width as u32);
                                    self.lib.set_border_color(window, CONFIG.background_color);
                                }
                                self.set_focus(window, ww);
                            }
                            (_, WindowState::Snapped) => {
                                self.lib.center_cursor(window);
                                self.set_focus(window, ww);
                            }

                            _ => {}
                        }
                    }
                }
                if ww.current_state == WindowState::Destroy {
                    debug!("killing window: {}", window);
                    self.kill_window(
                        window,
                        state.clients().keys().map(|w| *w).collect::<Vec<Window>>(),
                    );
                }
            }
        }
        self.prev_state = state.clone();
        Ok(())
    }
}
impl HdlReactor {
    pub fn new(lib: Box<Rc<dyn DisplayServer>>, state: State) -> Self {
        Self {
            lib,
            prev_state: state,
        }
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
                CONFIG.mod_key,
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
        let key_list = CONFIG
            .key_bindings
            .iter()
            .filter(|binding| match binding.key {
                Key::Letter(_) => true,
                _ => false,
            })
            .cloned()
            .map(|binding| match binding.key {
                Key::Letter(x) => x,
                _ => "".to_string(),
            })
            .filter(|key| !key.is_empty())
            .chain(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
                    .iter()
                    .map(|x| x.to_string()),
            )
            .collect::<Vec<String>>();

        for mod_key in mod_masks_vec() {
            for key in &key_list {
                if let Some(key_sym) = keysym_lookup::into_keysym(&key) {
                    self.lib.grab_keys(w, key_sym, CONFIG.mod_key | mod_key);
                }
            }
        }
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
        debug!("focusing: {:0x}", focus);
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
        // info!("Top level windows: {}", self.lib.top_level_window_count());
    }
}
