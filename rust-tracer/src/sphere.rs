use serde::{Serialize, Deserialize};
use crate::{Point, Ray, Material, Vector3};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn intersects(&self, ray: &Ray) -> Option<f64> {
        let sphere_to_ray = ray.origin - self.center;
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * c;

        if discriminant >= 0.0 {
            let distance = (-b - discriminant.sqrt()) / 2.0;
            if distance > 0.0 {
                return Some(distance);
            }
        }
        None
    }

    pub fn normal(&self, surface_point: Point) -> Vector3 {
        (surface_point - self.center).normalize()
    }
}
