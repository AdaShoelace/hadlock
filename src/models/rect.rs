
use crate::xlibwrapper::util::*;
use crate::xlibwrapper::core::Geometry;

#[derive(Copy, Clone)]
pub struct Rect {
    position: Position,
    size: Size,
}

impl Rect {
    pub fn new(position: Position, size: Size)  -> Self {
        Self {
            position,
            size
        }
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    pub fn get_size(&self) -> Size {
        self.size.clone()
    }
}

impl From<Geometry> for Rect {
    fn from(item: Geometry) -> Self {
        Self {
            position: Position { x: item.x, y: item.y },
            size: Size { width: item.width, height: item.height }
        }
    }
}
