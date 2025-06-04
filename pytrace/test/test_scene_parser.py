import unittest
from scene_parser import (
    parse_color,
    parse_point,
    parse_vector,
    parse_material,
    parse_sphere,
    parse_light,
    load_scene,
)
from color import Color
from point import Point
from vector import Vector3
from material import Material, CheckerMaterial
from sphere import Sphere
from light import Light
from scene import Scene
import os


class TestSceneParser(unittest.TestCase):
    def setUp(self):
        # Create a sample YAML content for testing
        self.test_yaml_path = "test/test_scene.yaml"
        self.yaml_content = """
name: test_scene
width: 800
height: 600
camera: [0, 0, -5]
lights:
  - type: point
    position: [2, 3, -4]
    color: "#FFFFFF"
objects:
  - type: sphere
    center: [0, 0, 0]
    radius: 1
    material:
      type: solid
      color: "#FF0000"
      ambient: 0.1
      diffuse: 0.8
      specular: 0.5
  - type: sphere
    center: [2, 0, 0]
    radius: 1.5
    material:
      type: checker
      color1: "#FFFFFF"
      color2: "#000000"
"""
        # Write test YAML file
        with open(self.test_yaml_path, "w") as f:
            f.write(self.yaml_content)

    def tearDown(self):
        # Clean up the test YAML file
        if os.path.exists(self.test_yaml_path):
            os.remove(self.test_yaml_path)

    def test_parse_color(self):
        color = parse_color("#FF0000")
        self.assertEqual(color, Color(1.0, 0.0, 0.0))

    def test_parse_point(self):
        point = parse_point([1, 2, 3])
        self.assertEqual(point, Point(1, 2, 3))

    def test_parse_vector(self):
        vector = parse_vector([1, 2, 3])
        self.assertEqual(vector, Vector3(1, 2, 3))

    def test_parse_solid_material(self):
        material_data = {"type": "solid", "color": "#FF0000", "ambient": 0.1, "diffuse": 0.8, "specular": 0.5}
        material = parse_material(material_data)
        self.assertIsInstance(material, Material)
        self.assertEqual(material.color, Color(1.0, 0.0, 0.0))
        self.assertEqual(material.ambient, 0.1)
        self.assertEqual(material.diffuse, 0.8)
        self.assertEqual(material.specular, 0.5)

    def test_parse_checker_material(self):
        material_data = {"type": "checker", "color1": "#FFFFFF", "color2": "#000000"}
        material = parse_material(material_data)
        self.assertIsInstance(material, CheckerMaterial)
        self.assertEqual(material.color1, Color(1.0, 1.0, 1.0))
        self.assertEqual(material.color2, Color(0.0, 0.0, 0.0))

    def test_parse_invalid_material(self):
        material_data = {"type": "invalid", "color": "#FF0000"}
        with self.assertRaises(ValueError):
            parse_material(material_data)

    def test_parse_sphere(self):
        sphere_data = {"center": [0, 0, 0], "radius": 1, "material": {"type": "solid", "color": "#FF0000"}}
        sphere = parse_sphere(sphere_data)
        self.assertIsInstance(sphere, Sphere)
        self.assertEqual(sphere.center, Point(0, 0, 0))
        self.assertEqual(sphere.radius, 1)

    def test_parse_light(self):
        light_data = {"position": [2, 3, -4], "color": "#FFFFFF"}
        light = parse_light(light_data)
        self.assertIsInstance(light, Light)
        self.assertEqual(light.position, Point(2, 3, -4))
        self.assertEqual(light.color, Color(1.0, 1.0, 1.0))

    def test_load_scene(self):
        scene = load_scene(self.test_yaml_path)
        self.assertIsInstance(scene, Scene)
        self.assertEqual(scene.width, 800)
        self.assertEqual(scene.height, 600)
        self.assertEqual(scene.camera, Vector3(0, 0, -5))
        self.assertEqual(len(scene.objects), 2)
        self.assertEqual(len(scene.lights), 1)
        self.assertEqual(scene.scene_name, "test_scene")


if __name__ == "__main__":
    unittest.main()
