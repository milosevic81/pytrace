from color import Color
from material import Material
from point import Point
from scene import Scene
from sphere import Sphere
from vector import Vector3
from light import Light

WIDTH, HEIGHT = 320, 240
camera = Vector3(0, 0, -1)
objects = [Sphere(Point(0, 0, 0), 0.5, Material(Color.BLUE, 0.05, 1.0, 0.7))]
lights = [Light(Point(1.5, -10.5, -10.0), Color.WHITE)]

scene = Scene(camera, objects, lights, WIDTH, HEIGHT)
scene_name = "blue-ball"