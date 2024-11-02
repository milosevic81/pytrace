use serde::{Serialize, Deserialize};
use crate::{Color, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckerMaterial {
    pub color1: Color,
    pub color2: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64,
}

impl CheckerMaterial {
    pub fn new(
        color1: Color,
        color2: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    ) -> Self {
        CheckerMaterial {
            color1,
            color2,
            ambient,
            diffuse,
            specular,
            reflection,
        }
    }

    pub fn color_at(&self, position: Point) -> Color {
        if (((position.x + 5.0) * 3.0) as i32 % 2) == ((position.z * 3.0) as i32 % 2) {
            self.color1
        } else {
            self.color2
        }
    }
} 