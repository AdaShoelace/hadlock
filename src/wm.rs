use std::collections::{
    HashMap,
};
use std::rc::Rc;

use crate::{
    HadlockOption,
    config::*,
    xlibwrapper::{
        masks::*,
        core::*,
        util::*,
        xlibmodels::*,
    },
    models::{
        windowwrapper::*,
        rect::*,
        dockarea::*,
        window_type::*,
        screen::*,
        WindowState,
        Direction
    }
};

pub fn window_inside_screen(w_geom: &Geometry, screen: &Screen) -> bool {
    let inside_width = w_geom.x >= screen.x && w_geom.x < screen.x + screen.width;
    let inside_height = w_geom.y >= screen.y && w_geom.y < screen.y + screen.height;
    inside_width && inside_height
}
