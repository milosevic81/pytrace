use crate::Vector3;
use std::ops::{Add, Mul};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub vector: Vector3,
}

impl Color {
    pub const COLOR_MAX: f64 = 255.0;

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            vector: Vector3::new(r, g, b)
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f64 / Self::COLOR_MAX;
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f64 / Self::COLOR_MAX;
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f64 / Self::COLOR_MAX;
        Color::new(r, g, b)
    }

    pub const WHITE: Color = Color { vector: Vector3 { x: 1.0, y: 1.0, z: 1.0 } };
    pub const BLACK: Color = Color { vector: Vector3 { x: 0.0, y: 0.0, z: 0.0 } };
    pub const RED: Color = Color { vector: Vector3 { x: 1.0, y: 0.0, z: 0.0 } };
    pub const GREEN: Color = Color { vector: Vector3 { x: 0.0, y: 1.0, z: 0.0 } };
    pub const BLUE: Color = Color { vector: Vector3 { x: 0.0, y: 0.0, z: 1.0 } };
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            vector: self.vector + other.vector
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar: f64) -> Color {
        Color {
            vector: self.vector * scalar
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(
            self.vector.x * other.vector.x,
            self.vector.y * other.vector.y,
            self.vector.z * other.vector.z,
        )
    }
}
