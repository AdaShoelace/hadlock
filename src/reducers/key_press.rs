use {
    crate::{
        config::CONFIG,
        models::{rect::*, window_type::WindowType, windowwrapper::*, Direction, WindowState},
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
    std::process::Command,
    std::rc::Rc,
};

impl Reducer<action::KeyPress> for State {
    fn reduce(&mut self, action: action::KeyPress) {
        let mod_not_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask;
        let mod_and_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask | Shift;

        let ws_keys: Vec<u8> = (1..=9)
            .map(|x| {
                self.lib
                    .str_to_keycode(&x.to_string())
                    .expect("key_press 1")
            })
        .collect();

    let handled_windows = self.windows.keys().map(|key| *key).collect::<Vec<u64>>();
    /*debug!(
      "KeyPress - root: {}, window: {}, handled_windows: {:?}",
      self.lib.get_root(),
      action.win,
      handled_windows
      );*/

    let mon = match self.monitors.get_mut(&self.current_monitor) {
        Some(mon) => mon,
        None => {
            warn!("No such monitor: {}", self.current_monitor);
            return
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
    }
}

fn managed_client(
    state: &mut State,
    action: action::KeyPress,
    mod_not_shift: bool,
    mod_and_shift: bool,
    ws_keys: Vec<u8>,
) -> Option<()> {
    debug!("Windows exists: KeyPress");
    let keycode = action.keycode as u8;

    if mod_not_shift && state.lib.str_to_keycode("Return")? == keycode {
        spawn_process(CONFIG.term.as_str(), vec![]);
    }

    if mod_and_shift {
        let old_size = state
            .monitors
            .get(&state.current_monitor)?
            .get_client(state.focus_w)?
            .get_size();
        if state.lib.str_to_keycode("Right")? == keycode {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)?;

            let (_dec_size, size) =
                mon.resize_window(state.focus_w, old_size.width + 10, old_size.height);
            let ww = mon.remove_window(state.focus_w);
            let new_ww = WindowWrapper {
                window_rect: Rect::new(ww.get_position(), size),
                handle_state: HandleState::Resize.into(),
                ..ww
            };
            mon.add_window(state.focus_w, new_ww);

            return Some(());
        }
        if state.lib.str_to_keycode("Left")? == keycode {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)?;

            let (_dec_size, size) =
                mon.resize_window(state.focus_w, old_size.width - 10, old_size.height);
            let ww = mon.remove_window(state.focus_w);
            let new_ww = WindowWrapper {
                window_rect: Rect::new(ww.get_position(), size),
                handle_state: HandleState::Resize.into(),
                ..ww
            };
            mon.add_window(state.focus_w, new_ww);
            return Some(());
        }
        if state.lib.str_to_keycode("Down")? == keycode {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)?;

            let (_dec_size, size) =
                mon.resize_window(state.focus_w, old_size.width, old_size.height + 10);
            let ww = mon.remove_window(state.focus_w);
            let new_ww = WindowWrapper {
                window_rect: Rect::new(ww.get_position(), size),
                handle_state: HandleState::Resize.into(),
                ..ww
            };
            mon.add_window(state.focus_w, new_ww);
            return Some(());
        }
        if state.lib.str_to_keycode("Up")? == keycode {
            let mon = state
                .monitors
                .get_mut(&state.current_monitor)?;
            let (_dec_size, size) =
                mon.resize_window(state.focus_w, old_size.width, old_size.height - 10);
            let ww = mon.remove_window(state.focus_w);
            let new_ww = WindowWrapper {
                window_rect: Rect::new(ww.get_position(), size),
                handle_state: HandleState::Resize.into(),
                ..ww
            };
            mon.add_window(state.focus_w, new_ww);
            return Some(());
        }
        if state.lib.str_to_keycode("q")? == keycode {
            let ww = state
                .monitors
                .get_mut(&state.current_monitor)?
                .get_client_mut(state.focus_w)?;

            ww.handle_state.replace(HandleState::Destroy);
            return Some(());
        }
        if state.lib.str_to_keycode("e")? == keycode {
            state.lib.exit();
            return Some(());
        }
        if state.lib.str_to_keycode("f")? == keycode {
            //wm.toggle_monocle(w);
        }

        match ws_keys.contains(&keycode) {
            true => {
                let ws_num = keycode_to_ws(keycode);
                //wm.move_to_ws(w, ws_num as u8);
                wm::set_current_ws(state, ws_num);
            }
            _ => {}
        }
    }

    if mod_not_shift {
        println!("Number pressed");

        if state.lib.str_to_keycode("f")? == keycode {
            let ww = state
                .monitors
                .get_mut(&state.current_monitor)?
                .remove_window(state.focus_w);
            let new_ww = wm::toggle_maximize(state, ww);
            state
                .monitors
                .get_mut(&state.current_monitor)?
                .add_window(state.focus_w, new_ww);
            return Some(());
        }
        if state.lib.str_to_keycode("Right")? == keycode
            || state.lib.str_to_keycode("l")? == keycode
        {
            shift_window(state, Direction::East);
            return Some(());
        }
        if state.lib.str_to_keycode("Left")? == keycode
            || state.lib.str_to_keycode("h")? == keycode
        {
            shift_window(state, Direction::West);
            return Some(());
        }
        if state.lib.str_to_keycode("Down")? == keycode
            || state.lib.str_to_keycode("j")? == keycode
        {
            shift_window(state, Direction::South);
            return Some(());
        }
        if state.lib.str_to_keycode("Up")? == keycode
            || state.lib.str_to_keycode("k")? == keycode
        {
            debug!("Snap up");
            shift_window(state, Direction::North);
            return Some(());
        }
        if state.lib.str_to_keycode("c")? == keycode {
            debug!("Center window");
            //wm.place_window(wm.focus_w);
            //wm.center_cursor(wm.focus_w);
            return Some(());
        }
        if state.lib.str_to_keycode("d")? == keycode {
            debug!("dmenu_run");
            spawn_process("dmenu_recency", vec![]);
            return Some(());
        }
        if ws_keys.contains(&keycode) {
            debug!("mod_not_shift switch ws");
            let ws_num = keycode_to_ws(keycode);
            wm::set_current_ws(state, ws_num);
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
) {
    let keycode = action.keycode as u8;
    if mod_not_shift {
        if state.lib.str_to_keycode("Return").expect("key_press: 17") == keycode {
            spawn_process(CONFIG.term.as_str(), vec![]);
        }
        if state.lib.str_to_keycode("d").expect("key_press: \"d\"") == keycode {
            debug!("dmenu_run");
            spawn_process("dmenu_recency", vec![]);
            return;
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
        if state.lib.str_to_keycode("e").expect("key_press: 18") == keycode {
            state.lib.exit();
        }
    }
}

fn shift_window(state: &mut State, direction: Direction) {
    let mon = state
        .monitors
        .get_mut(&state.current_monitor)
        .expect("KeyPress - shift_window - monitor - get_mut");
    let (pos, size) = mon.shift_window(state.focus_w, direction);
    let ww = mon.remove_window(state.focus_w);
    let ww = WindowWrapper {
        window_rect: Rect::new(pos, size),
        current_state: WindowState::Snapped,
        handle_state: HandleState::Shift.into(),
        ..ww
    };
    mon.add_window(state.focus_w, ww);
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
