use std::collections::HashMap;
use std::rc::Rc;

use crate::{
    config::*,
    models::{
        dockarea::*, rect::*, screen::*, window_type::*, windowwrapper::*, workspace::*, Direction,
        WindowState,
    },
    state::State,
    xlibwrapper::{core::*, masks::*, util::*, xlibmodels::*},
    HadlockOption,
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
        WindowState::Maximized => WindowWrapper {
            window_rect: Rect::new(ww.restore_position, ww.restore_size),
            previous_state: ww.current_state,
            current_state: ww.previous_state,
            handle_state: HandleState::MaximizeRestore.into(),
            ..ww
        },
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

pub fn set_current_ws(state: &mut State, ws: u32) {
    let mon = state
        .monitors
        .get_mut(&state.current_monitor)
        .expect("Wm - set_current_state_ws - monitor - get_mut");

    if ws == mon.current_ws {
        return;
    }

    let mut old_ws = mon.remove_ws(mon.current_ws).expect("Should be here..");
    old_ws.clients.values_mut().for_each(|client| {
        client.handle_state.replace(HandleState::Unmap);
    });
    mon.add_ws(old_ws);

    if mon.contains_ws(ws) {
        let mut new_ws = mon.remove_ws(ws).expect("Should also be here");
        new_ws.clients.values_mut().for_each(|client| {
            client.handle_state.replace(HandleState::Map);
        });
        debug!("swithcing to ws: {:?}", new_ws);
        mon.add_ws(new_ws);
    } else {
        mon.add_ws(Workspace::new(ws));
    }
    mon.current_ws = ws;
    state.lib.update_desktops(mon.current_ws, None);
}
