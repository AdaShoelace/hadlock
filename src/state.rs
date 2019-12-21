use {
    crate::xlibwrapper::{
        masks::*,
        xlibmodels::*,
        core::*,
        xatom::*,
    }
};

#[derive(Default)]
pub struct State {
    windows: Vec<Window>
}
