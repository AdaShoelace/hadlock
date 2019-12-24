use {
    std::rc::Rc,
    std::collections::HashMap,
    crate::models::{
        //monitor::Monitor,
        //workspace::Workspace,
        windowwrapper::WindowWrapper,
    },
    crate::xlibwrapper::{
        masks::*,
        xlibmodels::*,
        core::*,
        xatom::*,
    },
    derivative::*,
};

#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct State {
    #[derivative(Debug="ignore")]
    pub lib: Rc<XlibWrapper>,
    pub windows: HashMap<Window, WindowWrapper>,
    pub drag_start_pos: (i32, i32),
    pub drag_start_frame_pos: (i32, i32),
    pub drag_start_frame_size: (u32, u32),
}

impl State {
    pub fn new(lib: Rc<XlibWrapper>) -> Self {
        Self {
            lib,
            windows: HashMap::default(),
            drag_start_pos: (0,0),
            drag_start_frame_pos: (0,0),
            drag_start_frame_size: (0,0),
        }
    }
}
