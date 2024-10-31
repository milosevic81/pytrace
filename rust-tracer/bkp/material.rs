use serde::{Serialize, Deserialize};
use crate::{Color, Point};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
    Solid {
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    },
    Checker {
        color1: Color,
        color2: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    },
}

impl Material {
    pub fn new_solid(
        color: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    ) -> Self {
        Material::Solid {
            color,
            ambient,
            diffuse,
            specular,
            reflection,
        }
    }

    pub fn new_checker(
        color1: Color,
        color2: Color,
        ambient: f64,
        diffuse: f64,
        specular: f64,
        reflection: f64,
    ) -> Self {
        Material::Checker {
            color1,
            color2,
            ambient,
            diffuse,
            specular,
            reflection,
        }
    }

    pub fn color_at(&self, position: Point) -> Color {
        match self {
            Material::Solid { color, .. } => *color,
            Material::Checker { color1, color2, .. } => {
                if (((position.x + 5.0) * 3.0) as i32 % 2) == ((position.z * 3.0) as i32 % 2) {
                    *color1
                } else {
                    *color2
                }
            }
        }
    }

    pub fn ambient(&self) -> f64 {
        match self {
            Material::Solid { ambient, .. } => *ambient,
            Material::Checker { ambient, .. } => *ambient,
        }
    }

    pub fn diffuse(&self) -> f64 {
        match self {
            Material::Solid { diffuse, .. } => *diffuse,
            Material::Checker { diffuse, .. } => *diffuse,
        }
    }

    pub fn specular(&self) -> f64 {
        match self {
            Material::Solid { specular, .. } => *specular,
            Material::Checker { specular, .. } => *specular,
        }
    }

    pub fn reflection(&self) -> f64 {
        match self {
            Material::Solid { reflection, .. } => *reflection,
            Material::Checker { reflection, .. } => *reflection,
        }
    }
} 