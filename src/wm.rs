#![allow(dead_code)]

use crate::{
    models::{rect::*, screen::*, windowwrapper::*, workspace::*, HandleState, WindowState},
    state::State,
    xlibwrapper::{util::*, xlibmodels::*},
};

pub fn window_inside_screen(w_geom: &Geometry, screen: &Screen) -> bool {
    let inside_width = w_geom.x >= screen.x && w_geom.x < screen.x + screen.width;
    let inside_height = w_geom.y >= screen.y && w_geom.y < screen.y + screen.height;
    inside_width && inside_height
}

pub fn toggle_maximize(state: &mut State, ww: WindowWrapper) -> WindowWrapper {
    let ww_state = ww.current_state;
    let mon = state
        .monitors
        .get_mut(&state.current_monitor)
        .expect("toggle_maximize - monitor - get_mut");
    match ww_state {
        WindowState::Maximized => {
            debug!("Toggle back to: {:?} - Maximize", ww.previous_state);
            WindowWrapper {
                window_rect: Rect::new(ww.restore_position, ww.restore_size),
                previous_state: ww.current_state,
                current_state: ww.previous_state,
                handle_state: HandleState::MaximizeRestore.into(),
                ..ww
            }
        }
        _ => {
            let (pos, size) = mon.maximize(ww.window(), &ww);
            WindowWrapper {
                restore_position: ww.get_position(),
                restore_size: ww.get_size(),
                window_rect: Rect::new(pos, size),
                previous_state: ww.current_state,
                current_state: WindowState::Maximized,
                handle_state: HandleState::Maximize.into(),
                ..ww
            }
        }
    }
}

pub fn toggle_monocle(state: &mut State, ww: WindowWrapper) -> WindowWrapper {
    let ww_state = ww.current_state;
    let mon = state
        .monitors
        .get_mut(&state.current_monitor)
        .expect("toggle_maximize - monitor - get_mut");
    match ww_state {
        WindowState::Monocle => {
            debug!("Toggle back to: {:?} - Monocle", ww.previous_state);
            WindowWrapper {
                window_rect: Rect::new(ww.restore_position, ww.restore_size),
                previous_state: ww.current_state,
                current_state: ww.previous_state,
                handle_state: HandleState::MonocleRestore.into(),
                ..ww
            }
        }
        _ => {
            let (pos, size) = mon.monocle(ww.window(), &ww);
            WindowWrapper {
                restore_position: ww.get_position(),
                restore_size: ww.get_size(),
                window_rect: Rect::new(pos, size),
                previous_state: ww.current_state,
                current_state: WindowState::Monocle,
                handle_state: HandleState::Monocle.into(),
                ..ww
            }
        }
    }
}

pub fn get_mon_by_ws(state: &State, ws: u32) -> Option<MonitorId> {
    let mut ret_vec = state
        .monitors
        .iter()
        .filter(|(_, val)| val.contains_ws(ws))
        .map(|(key, _)| *key)
        .collect::<Vec<MonitorId>>();
    if ret_vec.len() != 1 {
        None
    } else {
        Some(ret_vec.remove(0))
    }
}

pub fn get_mon_by_window(state: &State, w: Window) -> Option<MonitorId> {
    let mut ret_vec = state
        .monitors
        .iter()
        .filter(|(_, val)| val.contains_window(w))
        .map(|(key, _)| *key)
        .collect::<Vec<MonitorId>>();
    if ret_vec.len() != 1 {
        None
    } else {
        Some(ret_vec.remove(0))
    }
}

pub fn set_current_ws(state: &mut State, ws: u32) -> Option<()> {
    let mon = match get_mon_by_ws(state, ws) {
        Some(mon) => state.monitors.get_mut(&mon)?,
        None => state.monitors.get_mut(&state.current_monitor)?,
    };

    if ws == mon.current_ws {
        state.current_monitor = mon.id;
        state.lib.move_cursor(Position {
            x: mon.screen.x + (mon.screen.width / 2),
            y: mon.screen.y + (mon.screen.height / 2),
        });
        mon.handle_state.replace(HandleState::Focus.into());
        return Some(());
    }

    let mut old_ws = mon.remove_ws(mon.current_ws).expect("Should be here..");
    old_ws.clients.values_mut().for_each(|client| {
        client.handle_state.replace(HandleState::Unmap.into());
    });
    mon.add_ws(old_ws);

    if mon.contains_ws(ws) {
        let mut new_ws = mon.remove_ws(ws).expect("Should also be here");
        new_ws.clients.values_mut().for_each(|client| {
            client.handle_state.replace(HandleState::Map.into());
        });
        debug!("swithcing to ws: {:?}", new_ws);
        mon.add_ws(new_ws);
    } else {
        mon.add_ws(Workspace::new(ws));
    }

    state.current_monitor = mon.id;
    if mon.workspaces.get(&mon.current_ws)?.clients.is_empty() {
        mon.remove_ws(mon.current_ws);
    }
    mon.current_ws = ws;
    state.lib.move_cursor(Position {
        x: mon.screen.x + (mon.screen.width / 2),
        y: mon.screen.y + (mon.screen.height / 2),
    });
    mon.handle_state.replace(HandleState::Focus);
    Some(())
}

pub fn move_to_ws(state: &mut State, w: Window, ws: u32) -> Option<()> {
    let ww = state
        .monitors
        .get_mut(&state.current_monitor)?
        .remove_window(w)?;

    let mon = match get_mon_by_ws(state, ws) {
        Some(mon) => state.monitors.get_mut(&mon)?,
        None => state.monitors.get_mut(&state.current_monitor)?,
    };

    if ws == mon.current_ws {
        let (size, pos) = mon.place_window(w);
        let new_ww = WindowWrapper {
            restore_position: pos,
            window_rect: Rect::new(pos, size),
            handle_state: HandleState::Map.into(),
            ..ww
        };
        mon.add_window(w, new_ww);
        state.lib.unmap_window(w);
        return Some(());
    }

    if mon.contains_ws(ws) {
        let mut old_ws = mon.remove_ws(ws)?;
        let (size, pos) = mon.place_window(w);
        let new_ww = WindowWrapper {
            restore_position: pos,
            window_rect: Rect::new(pos, size),
            handle_state: HandleState::Map.into(),
            ..ww
        };
        old_ws.add_window(w, new_ww);
        mon.add_ws(old_ws);
        state.lib.unmap_window(w);
    } else {
        let mut new_ws = Workspace::new(ws);
        let (size, pos) = mon.place_window(w);
        let new_ww = WindowWrapper {
            restore_position: pos,
            window_rect: Rect::new(pos, size),
            handle_state: HandleState::Map.into(),
            ..ww
        };
        new_ws.add_window(w, new_ww);
        mon.add_ws(new_ws);
        state.lib.unmap_window(w);
    }

    Some(())
}

pub fn pointer_is_inside(state: &State, screen: &Screen) -> bool {
    let pointer_pos = state.lib.pointer_pos(state.focus_w);
    //debug!("pointer pos: {:?}", pointer_pos);
    let inside_height =
        pointer_pos.y >= screen.y && pointer_pos.y <= screen.y + screen.height as i32;

    let inside_width = pointer_pos.x >= screen.x && pointer_pos.x <= screen.x + screen.width as i32;

    inside_height && inside_width
}

pub fn point_is_inside(_state: &State, screen: &Screen, x: i32, y: i32) -> bool {
    let inside_height = y >= screen.y && y <= screen.y + screen.height as i32;

    let inside_width = x >= screen.x && x <= screen.x + screen.width as i32;

    inside_height && inside_width
}

pub fn get_monitor_by_mouse(state: &State) -> MonitorId {
    let mon_vec = state
        .monitors
        .iter()
        .filter(|(_key, mon)| pointer_is_inside(state, &mon.screen))
        .map(|(key, _mon)| *key)
        .collect::<Vec<u32>>();
    match mon_vec.get(0) {
        Some(mon_id) => *mon_id,
        None => state.current_monitor,
    }
}

pub fn get_monitor_by_point(state: &State, x: i32, y: i32) -> MonitorId {
    let mon_vec = state
        .monitors
        .iter()
        .filter(|(_key, mon)| point_is_inside(state, &mon.screen, x, y))
        .map(|(key, _mon)| *key)
        .collect::<Vec<u32>>();
    match mon_vec.get(0) {
        Some(mon_id) => *mon_id,
        None => state.current_monitor,
    }
}

