use {
    crate::{
        models::{
            Direction,
            window_type::WindowType,
            rect::*,
            windowwrapper::*,
        },
        xlibwrapper::action,
        xlibwrapper::core::*,
        xlibwrapper::util::*,
        xlibwrapper::masks::*,
        state::State,
        xlibwrapper::xlibmodels::*,
        config::CONFIG,
    },
    std::rc::Rc,
    std::process::Command,
    reducer::*
};


impl Reducer<action::KeyPress> for State {
    fn reduce(&mut self, action: action::KeyPress) {
        let mod_not_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask;
        let mod_and_shift = (action.state & (Mod4Mask | Shift)) == Mod4Mask | Shift;

        let ws_keys: Vec<u8> = (1..=9).map(|x| {
            self.lib.str_to_keycode(&x.to_string()).expect("key_press 1")
        }).collect();

        let handled_windows = self.windows.keys().map(|key| *key).collect::<Vec<u64>>();
        debug!("KeyPress - root: {}, window: {}, handled_windows: {:?}", self.lib.get_root(), action.win, handled_windows);

        let mon = self.monitors.get_mut(&self.current_monitor).expect("KeyPress - monitor - get_mut");

        match mon.get_client_mut(self.focus_w) {
            Some(ww) => {
                managed_client(self, action, mod_not_shift, mod_and_shift, ws_keys);
            }
            None if action.win == self.lib.get_root() => {
                let keycode = action.keycode as u8;
                if mod_not_shift {
                    if self.lib.str_to_keycode("Return").expect("key_press: 17") == keycode {
                        spawn_process(CONFIG.term.as_str(), vec![]);
                    }
                    if self.lib.str_to_keycode("d").expect("key_press: \"d\"") == keycode {
                        debug!("dmenu_run");
                        spawn_process("dmenu_recency", vec![]);
                        return;
                    }

                    match ws_keys.contains(&keycode) {
                        true  => {
                            let ws_num = keycode_to_ws(keycode);
                            //wm.set_current_ws(ws_num);
                        },
                        _ => {}
                    }
                }
                if mod_and_shift {
                    if self.lib.str_to_keycode("e").expect("key_press: 18") == keycode {
                        self.lib.exit();
                    }
                }

            }
            None => { return; }
        }
    }
}

fn managed_client(state: &mut State, action: action::KeyPress, mod_not_shift: bool, mod_and_shift: bool, ws_keys: Vec<u8>) {

    debug!("Windows exists: KeyPress");
    let keycode = action.keycode as u8;

    if mod_not_shift && state.lib.str_to_keycode("Return").expect("key_press: 2") == keycode {
        spawn_process(CONFIG.term.as_str(), vec![]);
    }

    if mod_and_shift {
        let old_size = state.monitors.get(&state.current_monitor).unwrap().get_client(state.focus_w).unwrap().get_size();
        if state.lib.str_to_keycode("Right").expect("key_press: 3") == keycode {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let size = Size {width: old_size.width + 10, height: old_size.height};
            let (_dec_size, size) = mon.resize_window(state.focus_w, size.width, size.height);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.handle_state.replace(HandleState::Resize);
            return
        }
        if state.lib.str_to_keycode("Left").expect("key_press: 4") == keycode {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let size = Size {width: old_size.width - 10, height: old_size.height};
            let (_dec_size, size) = mon.resize_window(state.focus_w, size.width, size.height);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.handle_state.replace(HandleState::Resize);
            return
        }
        if state.lib.str_to_keycode("Down").expect("key_press: 5") == keycode {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let size = Size {width: old_size.width, height: old_size.height + 10};
            let (_dec_size, size) = mon.resize_window(state.focus_w, size.width, size.height);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.handle_state.replace(HandleState::Resize);
            return
        }
        if state.lib.str_to_keycode("Up").expect("key_press: 6") == keycode {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let size = Size {width: old_size.width, height: old_size.height - 10};
            let (_dec_size, size) = mon.resize_window(state.focus_w, size.width, size.height);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.handle_state.replace(HandleState::Resize);
            return
        }
        if state.lib.str_to_keycode("q").expect("key_press: 7") == keycode {
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.handle_state.replace(HandleState::Destroy);
        }
        if state.lib.str_to_keycode("e").expect("key_press: 8") == keycode {
            state.lib.exit();
        }
        if state.lib.str_to_keycode("f").expect("key_press: 8") == keycode {
            //wm.toggle_monocle(w);
        }

        match ws_keys.contains(&keycode) {
            true  => {
                let ws_num = keycode_to_ws(keycode);
                //wm.move_to_ws(w, ws_num as u8);
                //wm.set_current_ws(ws_num);
            },
            _ => {}
        }

    }

    if mod_not_shift {
        println!("Number pressed");

        if state.lib.str_to_keycode("f").expect("Dafuq?!?!") == keycode {
            //wm.toggle_maximize(wm.focus_w);
        }
        if state.lib.str_to_keycode("Right").expect("key_press: 9") == keycode || state.lib.str_to_keycode("l").expect("key_press: 10") == keycode {
            //wm.shift_window(wm.focus_w, Direction::East);
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let (pos, size) = mon.shift_window(state.focus_w, Direction::East);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.set_position(pos);
            ww.handle_state.replace(HandleState::Shift);
            state.lib.center_cursor(state.focus_w);
            return;
        }
        if state.lib.str_to_keycode("Left").expect("key_press: 11") == keycode || state.lib.str_to_keycode("h").expect("key_press: 12") == keycode {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let (pos, size) = mon.shift_window(state.focus_w, Direction::West);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.set_position(pos);
            ww.handle_state.replace(HandleState::Shift);
            state.lib.center_cursor(state.focus_w);
            return;
        }
        if state.lib.str_to_keycode("Down").expect("key_press: 13") == keycode || state.lib.str_to_keycode("j").expect("key_press: 14") == keycode  {
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let (pos, size) = mon.shift_window(state.focus_w, Direction::South);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.set_position(pos);
            ww.handle_state.replace(HandleState::Shift);
            state.lib.center_cursor(state.focus_w);
            return;
        }
        if state.lib.str_to_keycode("Up").expect("key_press: \"Up\"") == keycode || state.lib.str_to_keycode("k").expect("key_press: 16") == keycode  {
            debug!("Snap up");
            let mon = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut");
            let (pos, size) = mon.shift_window(state.focus_w, Direction::North);
            let ww = state.monitors.get_mut(&state.current_monitor).expect("KeyPress - monitor - get_mut").get_client_mut(state.focus_w).unwrap();
            ww.set_size(size);
            ww.set_position(pos);
            ww.handle_state.replace(HandleState::Shift);
            state.lib.center_cursor(state.focus_w);
            return;
        }
        if state.lib.str_to_keycode("c").expect("key_press: \"c\"") == keycode {
            debug!("Center window");
            //wm.place_window(wm.focus_w);
            //wm.center_cursor(wm.focus_w);
            return;
        }
        if state.lib.str_to_keycode("d").expect("key_press: \"d\"") == keycode {
            debug!("dmenu_run");
            spawn_process("dmenu_recency", vec![]);
            return;
        }
        if ws_keys.contains(&keycode) {
            let ws_num = keycode_to_ws(keycode);
            //wm.set_current_ws(ws_num);
        }
    }
}

fn keycode_to_ws(keycode: u8) -> u32 {
    ((keycode - 10) % 10) as u32
}

fn spawn_process(bin_name: &str, args: Vec<&str>) {
    let mut cmd = Command::new(bin_name);
    args
        .into_iter()
        .for_each(|arg| {cmd.arg(arg);});
    let _ = cmd.spawn();
}
