use serde::Deserialize;
use std::fs;
use crate::{Color, Material, material::MaterialType, Point, Scene, Sphere, Light, Vector3};

#[derive(Debug, Deserialize)]
struct SceneData {
    name: String,
    width: u32,
    height: u32,
    camera: Vec<f64>,
    objects: Vec<ObjectData>,
    lights: Vec<LightData>,
}

#[derive(Debug, Deserialize)]
struct ObjectData {
    #[serde(rename = "type")]
    obj_type: String,
    center: Vec<f64>,
    radius: f64,
    material: MaterialData,
}

#[derive(Debug, Deserialize)]
struct MaterialData {
    #[serde(rename = "type")]
    material_type: String,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    color1: Option<String>,
    #[serde(default)]
    color2: Option<String>,
    #[serde(default = "default_ambient")]
    ambient: f64,
    #[serde(default = "default_diffuse")]
    diffuse: f64,
    #[serde(default = "default_specular")]
    specular: f64,
}

#[derive(Debug, Deserialize)]
struct LightData {
    position: Vec<f64>,
    color: String,
}

fn default_ambient() -> f64 { 0.05 }
fn default_diffuse() -> f64 { 1.0 }
fn default_specular() -> f64 { 0.7 }

pub fn load_scene(yaml_file: &str) -> Result<Scene, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(yaml_file)?;
    let data: SceneData = serde_yaml::from_str(&contents)?;
    
    let camera = parse_vector(&data.camera)?;
    let objects = parse_objects(&data.objects)?;
    let lights = parse_lights(&data.lights)?;

    Ok(Scene::new(
        camera,
        objects,
        lights,
        data.width,
        data.height,
        data.name,
    ))
}

fn parse_vector(data: &[f64]) -> Result<Vector3, Box<dyn std::error::Error>> {
    if data.len() != 3 {
        return Err("Vector must have exactly 3 components".into());
    }
    Ok(Vector3::new(data[0], data[1], data[2]))
}

fn parse_color(hex: &str) -> Result<Color, Box<dyn std::error::Error>> {
    Ok(Color::from_hex(hex))
}

fn parse_material(data: &MaterialData) -> Result<Material, Box<dyn std::error::Error>> {
    match data.material_type.as_str() {
        "solid" => {
            let color = parse_color(data.color.as_ref().ok_or("Color is required for solid material")?)?;
            Ok(Material::new(
                color,
                color,
                MaterialType::Simple,
                data.ambient,
                data.diffuse,
                data.specular,
                0.5, // default reflection
            ))
        }
        "checker" => {
            let color1 = parse_color(data.color1.as_ref().ok_or("color1 is required for checker material")?)?;
            let color2 = parse_color(data.color2.as_ref().ok_or("color2 is required for checker material")?)?;
            Ok(Material::new(
                color1, // We'll need to modify the Material struct to handle checker pattern
                color2,
                MaterialType::Checker,
                data.ambient,
                data.diffuse,
                data.specular,
                0.5,
            ))
        }
        _ => Err(format!("Unknown material type: {}", data.material_type).into())
    }
}

fn parse_objects(objects_data: &[ObjectData]) -> Result<Vec<Sphere>, Box<dyn std::error::Error>> {
    let mut objects = Vec::new();
    for obj in objects_data {
        match obj.obj_type.as_str() {
            "sphere" => {
                let center = parse_vector(&obj.center)?;
                let material = parse_material(&obj.material)?;
                objects.push(Sphere::new(center, obj.radius, material));
            }
            _ => return Err(format!("Unknown object type: {}", obj.obj_type).into())
        }
    }
    Ok(objects)
}

fn parse_lights(lights_data: &[LightData]) -> Result<Vec<Light>, Box<dyn std::error::Error>> {
    let mut lights = Vec::new();
    for light in lights_data {
        let position = parse_vector(&light.position)?;
        let color = parse_color(&light.color)?;
        lights.push(Light::new(position, color));
    }
    Ok(lights)
} 