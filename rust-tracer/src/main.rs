mod color;
mod ray;
mod material;
mod sphere;
mod light;
mod scene;
mod image;
mod engine;
mod scene_parser;
mod vector;

use std::env;
use std::path::Path;

pub use color::Color;
pub use ray::Ray;
pub use material::Material;
pub use sphere::Sphere;
pub use light::Light;
pub use scene::Scene;
pub use image::Image;
pub use engine::RenderEngine;
pub use vector::{Vector3, Point};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <scene.yaml>", args[0]);
        std::process::exit(1);
    }

    let scene = scene_parser::load_scene(&args[1])?;
    let engine = RenderEngine::new();
    let image = engine.render(&scene);

    let output_path = format!("renders/{}.png", scene.scene_name);
    std::fs::create_dir_all(Path::new("renders"))?;
    image.save_as_png(&output_path)?;
    println!("Image saved to {}", output_path);

    Ok(())
}
