use serde::{Serialize, Deserialize};
use crate::{Point, Vector3};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }
} 