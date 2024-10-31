use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            *self / mag
        } else {
            Vector3::zero()
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f64) -> Vector3 {
        Vector3::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
        )
    }
}

// Point is just an alias for Vector3
pub type Point = Vector3; 