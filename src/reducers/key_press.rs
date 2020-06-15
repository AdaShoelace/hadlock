#![allow(unused_imports)]
#![allow(clippy::cognitive_complexity)]
use {
    crate::{
        config::{Axis, Key, KeyAction, KeyEffect, CONFIG},
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
        let has_mod = action.state & !CONFIG.super_key != 0;
        let super_is_pressed = action.state & CONFIG.super_key == CONFIG.super_key;

        let sym = self
            .lib
            .keycode_to_key_sym(action.keycode as u8)
            .expect("failed to convert action.keycode to KeySym");
        //debug!("KeyCode to string: {:?}", into_hdl_keysym(&sym));

        // Valid key presses must either include super or be one of the XF86 symbols.
        if !super_is_pressed && !sym.starts_with("XF86") {
            return;
        }

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
                managed_client(self, action, has_mod, ws_keys);
            }
            None if action.win == self.lib.get_root() => {
                root(self, action, has_mod, ws_keys);
            }
            None => {}
        }
    }
}

fn handle_key_effect(
    state: &mut State,
    action: &action::KeyPress,
    effect: &KeyEffect,
    ws_keys: &[u8],
) -> Option<()> {
    let keycode = action.keycode as u8;
    match effect {
        KeyEffect::Kill => {
            let ww = state
                .monitors
                .get_mut(&state.current_monitor)?
                .get_client_mut(state.focus_w)?;

            ww.handle_state.replace(HandleState::Destroy.into());
        }
        KeyEffect::OpenTerm => {
            spawn_process(CONFIG.term.as_str(), vec![]);
        }
        KeyEffect::Resize(delta, axis) => {
            let resize = state
                .monitors
                .get(&state.current_monitor)?
                .get_current_layout()?
                == LayoutTag::Floating;
            let old_size = state
                .monitors
                .get(&state.current_monitor)?
                .get_client(state.focus_w)?
                .get_size();
            if resize {
                let mon = state.monitors.get_mut(&state.current_monitor)?;
                if *axis == Axis::Horizontal {
                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width + (delta), old_size.height);
                    mon.swap_window(state.focus_w, |_, ww| WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    });
                } else {
                    let (_dec_size, size) =
                        mon.resize_window(state.focus_w, old_size.width, old_size.height + (delta));
                    mon.swap_window(state.focus_w, |_mon, ww| WindowWrapper {
                        window_rect: Rect::new(ww.get_position(), size),
                        handle_state: HandleState::Resize.into(),
                        ..ww
                    });
                }
            }
        }
        KeyEffect::Exit => state.lib.exit(),
        KeyEffect::ToggleMonocle => {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)
                .expect("ToggleMonocle get_mut monitor");
            mon.swap_window(state.focus_w, |mon, ww| wm::toggle_monocle(mon, ww));
        }
        KeyEffect::ToggleMaximize => {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)
                .expect("ToggleMonocle get_mut monitor");
            mon.swap_window(state.focus_w, |mon, ww| wm::toggle_maximize(mon, ww));
        }
        KeyEffect::CirculateLayout => {
            //debug!("should print layout type");
            circulate_layout(state);
            wm::reorder(state);
        }
        KeyEffect::ShiftWindow(direction) => {
            shift_window(state, *direction);
        }
        KeyEffect::ChangeCurrentWorkspace => {
            if ws_keys.contains(&keycode) {
                let ws_num = keycode_to_ws(keycode);
                wm::set_current_ws(state, ws_num);
            }
        }
        KeyEffect::MoveToWorkspace => {
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
        KeyEffect::SwapMaster => {
            swap_master(state);
        }
        KeyEffect::Center => {
            let mon = state.monitors.get_mut(&state.current_monitor)?;
            if mon.get_current_layout()? != LayoutTag::Floating {
                return Some(());
            }
            let windows = mon.place_window(state.focus_w);

            for (win, rect) in windows.into_iter() {
                mon.swap_window(win, |_, ww| WindowWrapper {
                    window_rect: rect,
                    previous_state: ww.current_state,
                    current_state: WindowState::Free,
                    handle_state: HandleState::Center.into(),
                    ..ww
                });
            }
        }
        KeyEffect::Reorder => {
            let current_layout = state
                .monitors
                .get(&state.current_monitor)?
                .get_current_layout()?;
            if current_layout == LayoutTag::Floating {
                wm::reorder(state);
            }
        }
        KeyEffect::Custom(command) => {
            spawn_process(&command.program, command.args.clone());
        }
        _ => (),
    }
    Some(())
}

fn managed_client(
    state: &mut State,
    action: action::KeyPress,
    has_mod: bool,
    ws_keys: Vec<u8>,
) -> Option<()> {
    //debug!("Windows exists: KeyPress");
    let keycode = action.keycode as u8;

    for key_action in CONFIG.key_bindings.iter() {
        match key_action {
            KeyAction {
                mod_key: Some(mk),
                key: Key::Letter(key),
                effect,
            } if has_mod => {
                if into_mod(mk) == (action.state & into_mod(mk))
                    && state.lib.str_to_keycode(key) == Some(action.keycode as u8)
                {
                    //debug!("Effect: {:?}", effect);
                    if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                        debug!("Something went wrong calling handle_key_effect in root");
                    }
                }
            }
            KeyAction {
                mod_key: None,
                key: Key::Letter(key),
                effect,
            } if !has_mod => {
                if state.lib.str_to_keycode(key) == Some(action.keycode as u8) {
                    //debug!("Effect: {:?}", effect);
                    if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                        debug!("Something went wrong calling handle_key_effect in root");
                    }
                }
            }
            KeyAction {
                mod_key: Some(mk),
                key: Key::Number,
                effect,
            } if has_mod && ws_keys.contains(&keycode) => {
                if into_mod(mk) == (action.state & into_mod(mk)) {
                    //debug!("Effect: {:?}", effect);
                    if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                        debug!("Something went wrong calling handle_key_effect in root");
                    }
                }
            }
            KeyAction {
                mod_key: None,
                key: Key::Number,
                effect,
            } if !has_mod && ws_keys.contains(&keycode) => {
                //debug!("Effect: {:?}", effect);
                if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                    debug!("Something went wrong calling handle_key_effect in root");
                }
            }
            _ => {} //debug!("nope"),
        }
    }
    Some(())
}

fn root(
    state: &mut State,
    action: action::KeyPress,
    has_mod: bool,
    ws_keys: Vec<u8>,
) -> Option<()> {
    let keycode = action.keycode as u8;

    for key_action in CONFIG.key_bindings.iter() {
        match key_action {
            KeyAction {
                mod_key: Some(mk),
                key: Key::Letter(key),
                effect,
            } if has_mod => {
                if into_mod(mk) == (action.state & into_mod(mk))
                    && state.lib.str_to_keycode(key) == Some(action.keycode as u8)
                {
                    debug!("Effect: {:?}", effect);
                    if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                        debug!("Something went wrong calling handle_key_effect in root");
                    }
                }
            }
            KeyAction {
                mod_key: None,
                key: Key::Letter(key),
                effect,
            } if !has_mod => {
                if state.lib.str_to_keycode(key) == Some(action.keycode as u8) {
                    debug!("Effect: {:?}", effect);
                    if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                        debug!("Something went wrong calling handle_key_effect in root");
                    }
                }
            }
            KeyAction {
                mod_key: None,
                key: Key::Number,
                effect,
            } if !has_mod && ws_keys.contains(&keycode) => {
                debug!("Effect: {:?}", effect);
                if handle_key_effect(state, &action, effect, &ws_keys).is_none() {
                    debug!("Something went wrong calling handle_key_effect in root");
                }
            }
            _ => debug!("nope"),
        }
    }
    Some(())
}

fn shift_window(state: &mut State, direction: Direction) -> Option<()> {
    debug!(
        "state focus_w: 0x{:x}, root: 0x{:x}",
        state.focus_w,
        state.lib.get_root()
    );
    let mon = state.monitors.get_mut(&state.current_monitor)?;
    if mon.get_current_layout().expect("shift layout failed") != LayoutTag::Floating {
        let (newest, _) = mon.get_newest()?;
        if mon.get_current_ws().unwrap().focus_w != *newest {
            match direction {
                Direction::North => {
                    if let Some(ww) = mon.get_previous(state.focus_w) {
                        let _ = state
                            .tx
                            .send(internal_action::InternalAction::FocusSpecific(ww.window()));
                    }
                }
                Direction::South => {
                    if let Some(ww) = mon.get_next(state.focus_w) {
                        let _ = state
                            .tx
                            .send(internal_action::InternalAction::FocusSpecific(ww.window()));
                    }
                }
                Direction::West => {
                    let _ = state
                        .tx
                        .send(internal_action::InternalAction::FocusSpecific(*newest));
                }
                _ => (),
            }
        }

        if state.focus_w == *newest && direction == Direction::East {
            if let Some(ww) = mon.get_next(state.focus_w) {
                let _ = state
                    .tx
                    .send(internal_action::InternalAction::FocusSpecific(ww.window()));
            }
        } else if state.focus_w == *newest && direction != Direction::East {
            debug!("window is newest but direction is: {:?}", direction);
        }
        return Some(());
    }
    if state.focus_w == state.lib.get_root() {
        return Some(());
    }
    let windows = mon.shift_window(state.focus_w, direction);

    for win in windows.into_iter() {
        mon.swap_window(win.window(), |_, ww| WindowWrapper {
            previous_state: ww.current_state,
            current_state: WindowState::Snapped,
            handle_state: HandleState::Shift.into(),
            ..win
        });
    }
    Some(())
}

fn swap_master(state: &mut State) -> Option<()> {
    debug!("Swap master");
    let mon = state.monitors.get_mut(&state.current_monitor)?;
    let newest = mon.get_newest()?;
    let (win, client) = (*newest.0, newest.1.clone());
    if win != state.focus_w {
        let client_toc = client.toc;
        let mut tmp_toc = std::time::Instant::now();
        mon.swap_window(state.focus_w, |_mon, ww| WindowWrapper {
            toc: {
                tmp_toc = ww.toc;
                client_toc
            },
            handle_state: HandleState::Unfocus.into(),
            ..ww
        })?;
        mon.swap_window(win, |_mon, ww| WindowWrapper {
            toc: tmp_toc,
            handle_state: HandleState::Focus.into(),
            ..ww
        })?;
        wm::reorder(state);
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
        .timeout(Timeout::Milliseconds(1500))
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

fn spawn_process(bin_name: &str, args: Vec<String>) {
    let mut cmd = Command::new(bin_name);
    args.into_iter().for_each(|arg| {
        cmd.arg(arg);
    });
    let _ = cmd.spawn();
}
