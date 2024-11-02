// use serde::{Serialize, Deserialize};
use std::fs;
use crate::{Point, Light, Sphere};

#[derive(Debug, Clone)]
pub struct Scene {
    pub camera: Point,
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub width: u32,
    pub height: u32,
    pub scene_name: String,
}

impl Scene {
    pub fn new(
        camera: Point,
        objects: Vec<Sphere>,
        lights: Vec<Light>,
        width: u32,
        height: u32,
        scene_name: String,
    ) -> Self {
        Scene {
            camera,
            objects,
            lights,
            width,
            height,
            scene_name,
        }
    }

    // pub fn from_yaml(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
    //     let contents = fs::read_to_string(path)?;
    //     let scene: Scene = serde_yaml::from_str(&contents)?;
    //     Ok(scene)
    // }

    // pub fn save_yaml(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    //     let yaml = serde_yaml::to_string(&self)?;
    //     fs::write(path, yaml)?;
    //     Ok(())
    // }
} 