use serde::{Serialize, Deserialize};
use crate::{Color, Point};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaterialType {
    Simple,
    Checker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub color1: Color,
    pub color2: Color,
    material_type: MaterialType,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub reflection: f64,
}


impl Material {

    pub fn new(color1: Color, color2: Color, material_type: MaterialType, ambient: f64, diffuse: f64, specular: f64, reflection: f64) -> Self {
        Material {
            color1,
            color2,
            material_type,
            ambient,
            diffuse,
            specular,
            reflection,
        }
    }
    
    pub fn color_at(&self, position: Point) -> Color {
        if self.material_type == MaterialType::Checker {           
            if (((position.x + 5.0) * 3.0) as i32 % 2) == ((position.z * 3.0) as i32 % 2) {
                return self.color1
            } else {
                return self.color2
            }
        }
        return self.color1
    }
} 