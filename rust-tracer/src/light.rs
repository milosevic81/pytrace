use crate::{Point, Color};

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Point,
    pub color: Color,
}

impl Light {
    pub fn new(position: Point, color: Color) -> Self {
        Light {
            position,
            color,
        }
    }

    pub fn default(position: Point) -> Self {
        Light {
            position,
            color: Color::WHITE,
        }
    }
}
