use crate::xlibwrapper::util::*;
use crate::xlibwrapper::xlibmodels::*;
use super::rect::*;

#[derive(Copy, Clone)]
pub struct WindowWrapper {
    is_floating: bool,
    dec: Option<Window>,
    window: Window,
    window_rect: Rect,
    dec_rect: Option<Rect>,
}

impl WindowWrapper {
    pub fn new(window: Window, window_rect: Rect) -> Self {
        Self {
            is_floating: false,
            dec: None,
            window,
            window_rect,
            dec_rect: None
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

    pub fn set_decoration(&mut self, dec: Window, dec_rect: Rect) {
        self.dec = Some(dec);
        self.dec_rect = Some(dec_rect);
    }

    pub fn get_dec(&self) -> Option<Window> {
        match self.dec {
            Some(_) => self.dec.clone(),
            None => None
        }
    }
    pub fn set_dec_size(&mut self, size: Size) {
        match self.dec_rect {
            Some(rect) => {
                self.dec_rect = Some(Rect::new(rect.get_position(), size));
            }
            None => {}
        }
    }

    pub fn set_dec_position(&mut self, position: Position) {
        match self.dec_rect {
            Some(rect) => {
                self.dec_rect = Some(Rect::new(position, rect.get_size()));
            }
            None => {}
        }
    }
    pub fn get_dec_rect(&self) -> Option<Rect> {
        self.dec_rect.clone()
    }
    
    pub fn set_position(&mut self, pos: Position) {
        match self.dec_rect {
            Some(dec_rect) => self.dec_rect = Some(Rect::new(pos, dec_rect.get_size())),
            None => self.window_rect = Rect::new(pos, self.window_rect.get_size())
        }
    }

    pub fn window(&self) -> Window {
        self.window
    }
    
    pub fn set_inner_size(&mut self, size: Size) {
        self.window_rect = Rect::new(self.window_rect.get_position(), size);
    }

    pub fn get_inner_rect(&self) -> Rect {
        self.window_rect.clone()
    }

    /*
     * Will set total size of window, so the outmost boundaries (excluding window borders)
     */
    pub fn set_size(&mut self, size: Size) {
        match self.dec_rect {
            Some(rect) => self.set_dec_size(size),
            None => self.set_inner_size(size)
        }
    }
    
    pub fn get_size(&self) -> Size {
        Size { width: self.get_width(), height: self.get_height() }
    }

    pub fn get_width(&self) -> u32 {
        match self.dec_rect {
            Some(rect) => rect.get_size().width,
            None => self.window_rect.get_size().width
        }
    }

    pub fn get_height(&self) -> u32 {
        match self.dec_rect {
            Some(rect) => rect.get_size().height,
            None => self.window_rect.get_size().height
        }
    }

    pub fn get_position(&self) -> Position {
        match self.dec_rect {
            Some(dec_rect) => dec_rect.get_position(),
            None => self.window_rect.get_position()
        }
    }
    

}
