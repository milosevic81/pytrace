use std::time::Instant;
use crate::{Color, Image, Point, Ray, Scene, Vector3, Sphere};

pub struct RenderEngine {
    max_depth: i32,
    max_distance: f64,
    min_displace: f64,
    specular_k: i32,
}

impl RenderEngine {
    pub fn new() -> Self {
        RenderEngine {
            max_depth: 5,
            max_distance: 10.0,
            min_displace: 0.0001,
            specular_k: 50,
        }
    }

    pub fn render(&self, scene: &Scene) -> Image {
        let start_time = Instant::now();
        
        let width = scene.width;
        let height = scene.height;
        let aspect_ratio = width as f64 / height as f64;
        let (x0, x1) = (-1.0, 1.0);
        let (y0, y1) = (x0 / aspect_ratio, x1 / aspect_ratio);
        let (dx, dy) = (
            (x1 - x0) / (width - 1) as f64,
            (y1 - y0) / (height - 1) as f64
        );

        let mut image = Image::new(width, height);

        for iy in 0..height {
            let y = y0 + iy as f64 * dy;
            for ix in 0..width {
                let x = x0 + ix as f64 * dx;
                let ray = Ray::new(
                    scene.camera,
                    Point::new(x, y, 0.0) - scene.camera
                );
                let color = self.ray_trace(&ray, scene, 0);
                image.set_pixel(ix, iy, color);
            }
            // Progress indicator
            print!("\rRendering: {:.1}%", (iy as f64 / height as f64) * 100.0);
        }

        let duration = start_time.elapsed();
        println!("\nRendering completed in {:.2?}", duration);
        
        image
    }

    fn ray_trace(&self, ray: &Ray, scene: &Scene, depth: i32) -> Color {
        let mut color = Color::BLACK;
        
        if let Some((distance, object)) = self.find_nearest(ray, scene) {
            if distance > self.max_distance {
                return color;
            }

            let hit_pos = ray.origin + ray.direction * distance;
            let hit_normal = object.normal(hit_pos);
            color = color + self.color_at(object, hit_pos, hit_normal, scene);

            // Handle reflections
            if depth < self.max_depth {
                let new_ray_dir = ray.direction - hit_normal * (2.0 * ray.direction.dot(&hit_normal));
                let new_ray = Ray::new(
                    hit_pos + hit_normal * self.min_displace,
                    new_ray_dir
                );
                color = color + self.ray_trace(&new_ray, scene, depth + 1) * object.material.reflection;
            }
        }

        color
    }

    fn find_nearest(&self, ray: &Ray, scene: &Scene) -> Option<(f64, &Sphere)> {
        let mut nearest: Option<(f64, &Sphere)> = None;

        for object in &scene.objects {
            if let Some(dist) = object.intersects(ray) {
                match nearest {
                    None => nearest = Some((dist, object)),
                    Some((min_dist, _)) if dist < min_dist => nearest = Some((dist, object)),
                    _ => {}
                }
            }
        }

        nearest
    }

    fn color_at(&self, object: &Sphere, hit_pos: Point, normal: Vector3, scene: &Scene) -> Color {
        let material = &object.material;
        let obj_color = material.color_at(hit_pos);
        let to_cam = scene.camera - hit_pos;
        let mut color = Color::WHITE * material.ambient;

        for light in &scene.lights {
            let to_light = Ray::new(
                hit_pos,
                light.position - hit_pos
            );

            // Diffuse shading (Lambert)
            let light_power = normal.dot(&to_light.direction).max(0.0);
            color = color + obj_color * material.diffuse * light_power;

            // Specular shading (Blinn-Phong)
            let half_vector = (to_light.direction + to_cam).normalize();
            let specular_power = normal.dot(&half_vector).max(0.0).powi(self.specular_k);
            color = color + light.color * material.specular * specular_power;
        }

        color
    }
}