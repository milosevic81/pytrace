use serde::{Serialize, Deserialize};
use crate::{Color, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64,
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    ) -> Self {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            reflection,
        }
    }

    pub fn color_at(&self, position: Point) -> Color {
        self.color
    }
} 