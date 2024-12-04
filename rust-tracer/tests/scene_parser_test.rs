use std::fs;
use rust_tracer::{
    Color, Point, Vector3, Material, Scene, Sphere, Light,
    scene_parser::{load_scene, parse_color, parse_vector, parse_material, parse_lights},
};

fn setup() -> String {
    let test_path = "test_scene.yaml";
    fs::write(test_path, include_str!("test_scene.yaml")).expect("Failed to write test file");
    test_path.to_string()
}

fn teardown(test_path: &str) {
    fs::remove_file(test_path).expect("Failed to remove test file");
}

#[test]
fn test_parse_color() {
    let color = parse_color("#FF0000").unwrap();
    assert_eq!(color, Color::new(1.0, 0.0, 0.0));
}

#[test]
fn test_parse_vector() {
    let vector = parse_vector(&vec![1.0, 2.0, 3.0]).unwrap();
    assert_eq!(vector, Vector3::new(1.0, 2.0, 3.0));
}

#[test]
fn test_parse_solid_material() {
    let material_data = serde_yaml::from_str(r#"
        type: solid
        color: '#FF0000'
        ambient: 0.1
        diffuse: 0.8
        specular: 0.5
    "#).unwrap();
    
    let material = parse_material(&material_data).unwrap();
    match material {
        Material::Solid { color, ambient, diffuse, specular, .. } => {
            assert_eq!(color, Color::new(1.0, 0.0, 0.0));
            assert_eq!(ambient, 0.1);
            assert_eq!(diffuse, 0.8);
            assert_eq!(specular, 0.5);
        },
        _ => panic!("Expected solid material"),
    }
}

#[test]
fn test_parse_checker_material() {
    let material_data = serde_yaml::from_str(r#"
        type: checker
        color1: '#FFFFFF'
        color2: '#000000'
    "#).unwrap();
    
    let material = parse_material(&material_data).unwrap();
    match material {
        Material::Checker { color1, color2, .. } => {
            assert_eq!(color1, Color::new(1.0, 1.0, 1.0));
            assert_eq!(color2, Color::new(0.0, 0.0, 0.0));
        },
        _ => panic!("Expected checker material"),
    }
}

#[test]
fn test_parse_invalid_material() {
    let material_data = serde_yaml::from_str(r#"
        type: invalid
        color: '#FF0000'
    "#).unwrap();
    
    assert!(parse_material(&material_data).is_err());
}

#[test]
fn test_parse_lights() {
    let lights_data = serde_yaml::from_str(r#"
        - position: [2, 3, -4]
          color: '#FFFFFF'
    "#).unwrap();
    
    let lights = parse_lights(&lights_data).unwrap();
    assert_eq!(lights.len(), 1);
    assert_eq!(lights[0].position, Point::new(2.0, 3.0, -4.0));
    assert_eq!(lights[0].color, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn test_load_scene() {
    let test_path = setup();
    
    let scene = load_scene(&test_path).unwrap();
    assert_eq!(scene.width, 800);
    assert_eq!(scene.height, 600);
    assert_eq!(scene.camera, Vector3::new(0.0, 0.0, -5.0));
    assert_eq!(scene.objects.len(), 2);
    assert_eq!(scene.lights.len(), 1);
    assert_eq!(scene.scene_name, "test_scene");
    
    teardown(&test_path);
} 