use super::{dockarea::DockArea, rect::Rect, screen::Screen, Direction};
use crate::xlibwrapper::util::{Position, Size};

const REGION_SIZE: Size = Size {
    width: 50,
    height: 50,
};

const REGION_SIZE_HORIZONTAL: Size = Size {
    width: 150,
    height: 40,
};

const REGION_SIZE_VERTICAL: Size = Size {
    width: 40,
    height: 150,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnappingRegion {
    pub dir: Direction,
    pub region: Rect,
}

impl SnappingRegion {
    pub fn new(screen: &Screen, dock_area: &DockArea, direction: Direction) -> Self {
        let dock_size = if let Some(area) = dock_area.as_rect(screen) {
            area.get_size()
        } else {
            Size {
                width: 0,
                height: 0,
            }
        };

        let rect = match direction {
            Direction::North => {
                let pos = Position::new(
                    (screen.width / 2) - REGION_SIZE_HORIZONTAL.width / 2,
                    screen.y + dock_size.height,
                );
                Rect::new(pos, REGION_SIZE_HORIZONTAL)
            }
            Direction::NorthEast => {
                let pos = Position::new(
                    screen.width - REGION_SIZE.width,
                    screen.y + dock_size.height,
                );
                Rect::new(pos, REGION_SIZE)
            }
            Direction::East => {
                let pos = Position::new(
                    screen.width - REGION_SIZE_VERTICAL.width,
                    (screen.height / 2) - REGION_SIZE_VERTICAL.height / 2,
                );
                Rect::new(pos, REGION_SIZE_VERTICAL)
            }
            Direction::SouthEast => {
                let pos = Position::new(
                    screen.width - REGION_SIZE.width,
                    screen.height - REGION_SIZE.height,
                );
                Rect::new(pos, REGION_SIZE)
            }
            Direction::South => {
                let pos = Position::new(
                    (screen.width / 2) - REGION_SIZE_HORIZONTAL.width / 2,
                    screen.height - REGION_SIZE_HORIZONTAL.height,
                );
                Rect::new(pos, REGION_SIZE_HORIZONTAL)
            }
            Direction::SouthWest => {
                let pos = Position::new(
                    screen.x,
                    (screen.height - dock_size.height) - REGION_SIZE.height,
                );
                Rect::new(pos, REGION_SIZE)
            }
            Direction::West => {
                let pos = Position::new(
                    screen.x,
                    (screen.height / 2) - REGION_SIZE_VERTICAL.height / 2,
                );
                Rect::new(pos, REGION_SIZE_VERTICAL)
            }
            Direction::NorthWest => {
                let pos = Position::new(screen.x, screen.y + dock_size.height);
                Rect::new(pos, REGION_SIZE)
            }
        };
        Self {
            dir: direction,
            region: rect,
        }
    }

    pub fn from_screen(screen: &Screen, dock_area: &DockArea) -> Vec<SnappingRegion> {
        vec![
            Self::new(screen, dock_area, Direction::North),
            Self::new(screen, dock_area, Direction::NorthEast),
            Self::new(screen, dock_area, Direction::East),
            Self::new(screen, dock_area, Direction::SouthEast),
            Self::new(screen, dock_area, Direction::South),
            Self::new(screen, dock_area, Direction::SouthWest),
            Self::new(screen, dock_area, Direction::West),
            Self::new(screen, dock_area, Direction::NorthWest),
        ]
    }

    pub fn contains(&self, pos: Position) -> bool {
        let region_pos = self.region.get_position();
        let region_size = self.region.get_size();

        let x_bound = pos.x >= region_pos.x && pos.x <= region_pos.x + region_size.width;
        let y_bound = pos.y >= region_pos.y && pos.y <= region_pos.y + region_size.height;

        x_bound && y_bound
    }
}
