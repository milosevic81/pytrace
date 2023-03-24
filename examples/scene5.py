from color import Color
from material import Material, CheckerMaterial
from point import Point
from scene import Scene
from sphere import Sphere
from vector import Vector3
from light import Light

scale = 1
WIDTH, HEIGHT = scale * 3440, scale * 1440

camera = Vector3(0, -0.35, -1)
objects = [
    Sphere(Point(0, 10000.5, 1), 10000, CheckerMaterial()),
    Sphere(Point(0.75, -0.1, 1), 0.6, Material(Color.BLUE)),
    Sphere(Point(-0.75, -0.1, 2.25), 0.6, Material(Color.RED)),
]
lights = [
    Light(Point(1.5, -0.5, -10.0), Color.WHITE),
    Light(Point(-0.5, -10.5, 0), Color.from_hex("#E6E6E6"))
]
scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
scene_name = "two-ball-wide"
