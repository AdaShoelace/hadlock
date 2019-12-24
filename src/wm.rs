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
