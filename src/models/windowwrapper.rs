use crate::xlibwrapper::util::*;

#[derive(Copy, Clone)]
pub struct WindowWrapper {
    is_floating: bool,
    dec: Option<Window>,
    window: Window
}

impl WindowWrapper {
    pub fn new(window: Window) -> Self {
        Self {
            is_floating: false,
            dec: None,
            window
        }
    }

    pub fn floating(&self) -> bool {
        self.is_floating
    }
    
    pub fn decorated(&self) -> bool {
        match self.dec {
            Some(_) => true,
            _ => false
        }
    }

    pub fn set_decoration(&mut self, dec: Window) {
        self.dec = Some(dec);
    }

    pub fn get_dec(&self) -> Option<Window> {
        match self.dec {
            Some(_) => self.dec.clone(),
            None => None
        }
    }

    pub fn window(&self) -> Window {
        self.window
    }
}
