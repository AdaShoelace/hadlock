#![allow(unused_imports)]
use {
    crate::{
        config::CONFIG,
        layout::LayoutTag,
        models::{
            internal_action, rect::*, window_type::WindowType, windowwrapper::*, Direction,
            HandleState, WindowState,
        },
        state::State,
        wm,
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::masks::*,
        xlibwrapper::util::keysym_lookup::*,
        xlibwrapper::util::*,
        xlibwrapper::xlibmodels::*,
    },
    notify_rust::{Notification, Timeout},
    reducer::*,
    std::cell::RefCell,
    std::process::Command,
    std::rc::Rc,
};

impl Reducer<action::KeyPress> for State {
    fn reduce(&mut self, action: action::KeyPress) {
        let mod_not_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask;
        let mod_and_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask | Shift;

        let sym = self.lib.keycode_to_key_sym(action.keycode as u8);
        debug!("KeyCode to string: {:?}", into_hdl_keysym(&sym));

        let ws_keys: Vec<u8> = (1..=9)
            .map(|x| {
                self.lib
                    .str_to_keycode(&x.to_string())
                    .expect("key_press 1")
            })
            .collect();

        let mon = match self.monitors.get_mut(&self.current_monitor) {
            Some(mon) => mon,
            None => {
                warn!("No such monitor: {}", self.current_monitor);
                return;
            }
        };

        match mon.get_client(self.focus_w) {
            Some(_) => {
                managed_client(self, action, mod_not_shift, mod_and_shift, ws_keys);
            }
            None if action.win == self.lib.get_root() => {
                root(self, action, mod_not_shift, mod_and_shift, ws_keys);
            }
            None => {
                return;
            }
        }
        //debug!("return from ")
    }
}

fn managed_client(
    state: &mut State,
    action: action::KeyPress,
    mod_not_shift: bool,
    mod_and_shift: bool,
    ws_keys: Vec<u8>,
) -> Option<()> {
    //debug!("Windows exists: KeyPress");
    let keycode = action.keycode as u8;

    if mod_not_shift && state.lib.str_to_keycode("Return")? == keycode {
        spawn_process(CONFIG.term.as_str(), vec![]);
    }
    let resize = state
        .monitors
        .get(&state.current_monitor)?
        .get_current_layout()?
        == LayoutTag::Floating;
    if mod_and_shift {
        let old_size = state
            .monitors
            .get(&state.current_monitor)?
            .get_client(state.focus_w)?
            .get_size();
        match into_hdl_keysym(&state.lib.keycode_to_key_sym(keycode)) {
            HDLKeysym::XK_Right => {
                if resize {
                    let mon = state.monitors.get_mut(&state.current_monitor)?;

                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width + 10, old_size.height);
                    let ww = mon.remove_window(state.focus_w)?;
                    let new_ww = WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    };
                    mon.add_window(state.focus_w, new_ww);
                }
            }

            HDLKeysym::XK_Left => {
                if resize {
                    let mon = state.monitors.get_mut(&state.current_monitor)?;

                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width - 10, old_size.height);
                    let ww = mon.remove_window(state.focus_w)?;
                    let new_ww = WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    };
                    mon.add_window(state.focus_w, new_ww);
                }
            }

            HDLKeysym::XK_Down => {
                if resize {
                    let mon = state.monitors.get_mut(&state.current_monitor)?;

                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width, old_size.height + 10);
                    let ww = mon.remove_window(state.focus_w)?;
                    let new_ww = WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    };
                    mon.add_window(state.focus_w, new_ww);
                }
            }

            HDLKeysym::XK_Up => {
                if resize {
                    let mon = state.monitors.get_mut(&state.current_monitor)?;
                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width, old_size.height - 10);
                    let ww = mon.remove_window(state.focus_w)?;
                    let new_ww = WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    };
                    mon.add_window(state.focus_w, new_ww);
                }
            }

            HDLKeysym::XK_q => {
                let ww = state
                    .monitors
                    .get_mut(&state.current_monitor)?
                    .get_client_mut(state.focus_w)?;

                ww.handle_state.replace(HandleState::Destroy.into());
            }

            HDLKeysym::XK_e => {
                state.lib.exit();
            }

            HDLKeysym::XK_f => {
                let ww = state
                    .monitors
                    .get_mut(&state.current_monitor)?
                    .remove_window(state.focus_w)?;
                let new_ww = wm::toggle_monocle(state, ww);
                state
                    .monitors
                    .get_mut(&state.current_monitor)?
                    .add_window(state.focus_w, new_ww);
            }

            HDLKeysym::XK_l => {
                debug!("should print layout type");
                circulate_layout(state);
                wm::reorder(state);
            }

            _ => {
                if ws_keys.contains(&keycode) {
                    let ws_num = keycode_to_ws(keycode);
                    wm::move_to_ws(state, state.focus_w, ws_num);
                    if state
                        .monitors
                        .get(&state.current_monitor)?
                        .get_current_layout()?
                        != LayoutTag::Floating
                    {
                        wm::reorder(state);
                    }
                    wm::set_current_ws(state, ws_num)?;
                }
            }
        }
        return Some(());
    }

    if mod_not_shift {
        println!("Number pressed");

        match into_hdl_keysym(&state.lib.keycode_to_key_sym(keycode)) {
            HDLKeysym::XK_f => {
                let ww = state
                    .monitors
                    .get_mut(&state.current_monitor)?
                    .remove_window(state.focus_w)?;
                let new_ww = wm::toggle_maximize(state, ww);
                state
                    .monitors
                    .get_mut(&state.current_monitor)?
                    .add_window(state.focus_w, new_ww);
            }

            HDLKeysym::XK_Right | HDLKeysym::XK_l => {
                shift_window(state, Direction::East);
            }

            HDLKeysym::XK_Left | HDLKeysym::XK_h => {
                shift_window(state, Direction::West);
            }
            HDLKeysym::XK_Down | HDLKeysym::XK_j => {
                shift_window(state, Direction::South);
            }
            HDLKeysym::XK_Up | HDLKeysym::XK_k => {
                shift_window(state, Direction::North);
            }
            HDLKeysym::XK_c => {
                let mon = state.monitors.get_mut(&state.current_monitor)?;
                let windows = mon.place_window(state.focus_w);

                for win in windows.into_iter() {
                    let ww = mon.remove_window(win.0)?;
                    let new_ww = WindowWrapper {
                        window_rect: win.1,
                        previous_state: ww.current_state,
                        current_state: WindowState::Free,
                        handle_state: HandleState::Center.into(),
                        ..ww
                    };
                    mon.add_window(new_ww.window(), new_ww);
                }
            }
            HDLKeysym::XK_d => {
                spawn_process("dmenu_recency", vec![]);
            }

            HDLKeysym::XK_r => {
                let current_layout = state
                    .monitors
                    .get(&state.current_monitor)
                    .expect("Key_press - reorder - get_mon")
                    .get_current_ws()
                    .expect("Key_press - reorder - get_current_ws")
                    .get_current_layout();
                if current_layout == LayoutTag::Floating {
                    wm::reorder(state);
                }
            }

            _ => {
                if ws_keys.contains(&keycode) {
                    //debug!("mod_not_shift switch ws");
                    let ws_num = keycode_to_ws(keycode);
                    wm::set_current_ws(state, ws_num)?;
                }
            }
        }
    }
    Some(())
}

fn root(
    state: &mut State,
    action: action::KeyPress,
    mod_not_shift: bool,
    mod_and_shift: bool,
    ws_keys: Vec<u8>,
) -> Option<()> {
    let keycode = action.keycode as u8;
    if mod_not_shift {
        match into_hdl_keysym(&state.lib.keycode_to_key_sym(keycode)) {
            HDLKeysym::XK_Return => {
                spawn_process(CONFIG.term.as_str(), vec![]);
            }
            HDLKeysym::XK_d => {
                spawn_process("dmenu_recency", vec![]);
            }
            HDLKeysym::XK_r => {
                let current_layout = state
                    .monitors
                    .get(&state.current_monitor)
                    .expect("Key_press - reorder - get_mon")
                    .get_current_ws()
                    .expect("Key_press - reorder - get_current_ws")
                    .get_current_layout();
                if current_layout == LayoutTag::Floating {
                    wm::reorder(state);
                }
            }
            _ => (),
        }

        match ws_keys.contains(&keycode) {
            true => {
                let ws_num = keycode_to_ws(keycode);
                wm::set_current_ws(state, ws_num);
            }
            _ => {}
        }
    }
    if mod_and_shift {
        match into_hdl_keysym(&state.lib.keycode_to_key_sym(keycode)) {
            HDLKeysym::XK_e => {
                state.lib.exit();
            }

            HDLKeysym::XK_l => {
                circulate_layout(state);
                wm::reorder(state);
            }
            _ => (),
        }
    }
    Some(())
}

fn shift_window(state: &mut State, direction: Direction) -> Option<()> {
    let mon = state.monitors.get_mut(&state.current_monitor)?;

    if mon.get_current_ws()?.get_current_layout() != LayoutTag::Floating {
        return Some(());
    }

    let windows = mon.shift_window(state.focus_w, direction);

    for win in windows.into_iter() {
        let ww = mon.remove_window(win.window())?;
        let ww = WindowWrapper {
            previous_state: ww.current_state,
            current_state: WindowState::Snapped,
            handle_state: HandleState::Shift.into(),
            ..win
        };
        mon.add_window(win.window(), ww);
    }
    Some(())
}

fn circulate_layout(state: &mut State) -> Option<()> {
    let mon = state.monitors.get_mut(&state.current_monitor)?;
    let ws = mon.get_current_ws_mut()?;
    ws.circulate_layout();

    let notify_res = Notification::new()
        .summary("Layout switched")
        .body(&format!("New layout: {}", ws.layout))
        .icon("firefox")
        .timeout(Timeout::Milliseconds(3000))
        .show();
    match notify_res {
        Ok(_) => Some(()),
        Err(e) => {
            warn!("Error showing notification: {}", e);
            Some(())
        }
    }
}

fn keycode_to_ws(keycode: u8) -> u32 {
    ((keycode - 10) % 10) as u32
}

fn spawn_process(bin_name: &str, args: Vec<&str>) {
    let mut cmd = Command::new(bin_name);
    args.into_iter().for_each(|arg| {
        cmd.arg(arg);
    });
    let _ = cmd.spawn();
}
