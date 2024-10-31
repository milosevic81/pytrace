import yaml
from color import Color
from material import Material, CheckerMaterial
from point import Point
from scene import Scene
from sphere import Sphere
from vector import Vector3
from light import Light

def parse_color(color_hex: str) -> Color:
    return Color.from_hex(color_hex)

def parse_point(point_data: list) -> Point:
    return Point(*point_data)

def parse_vector(vector_data: list) -> Vector3:
    return Vector3(*vector_data)

def parse_material(material_data: dict) -> Material:
    if material_data['type'] == 'solid':
        return Material(
            parse_color(material_data['color']),
            material_data.get('ambient', 0.05),
            material_data.get('diffuse', 1.0),
            material_data.get('specular', 0.7)
        )
    elif material_data['type'] == 'checker':
        return CheckerMaterial(
            color1=parse_color(material_data['color1']),
            color2=parse_color(material_data['color2'])
        )
    else:
        raise ValueError(f"Unknown material type: {material_data['type']}")

def parse_sphere(sphere_data: dict) -> Sphere:
    return Sphere(
        parse_point(sphere_data['center']),
        sphere_data['radius'],
        parse_material(sphere_data['material'])
    )

def parse_light(light_data: dict) -> Light:
    return Light(
        parse_point(light_data['position']),
        parse_color(light_data['color'])
    )

def parse_objects(objects_data: list) -> list:
    objects = []
    for obj in objects_data:
        if obj['type'] == 'sphere':
            objects.append(parse_sphere(obj))
    return objects

def load_scene(yaml_file: str) -> Scene:
    with open(yaml_file, 'r') as f:
        data = yaml.safe_load(f)

    camera = parse_vector(data['camera'])
    objects = parse_objects(data['objects'])
    lights = [parse_light(light) for light in data['lights']]
    width = data['width']
    height = data['height']
    scene_name = data['name']

    return Scene(camera, objects, lights, width, height, scene_name)

# Usage example:
if __name__ == "__main__":
    scene = load_scene("src/examples/scene1.yaml")