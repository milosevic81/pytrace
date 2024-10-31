import math

from color import Color
from material import Material
from point import Point


class Sphere:
    def __init__(self, center: Point, radius: float, material: Material):
        self.center = center
        self.radius = radius
        self.material = material

    def intersects(self, ray):
        """Checks if ray intersects sphere. Returns distance or None"""
        sphere_to_ray = ray.origin - self.center
        b = 2 * ray.direction.dot(sphere_to_ray)
        c = sphere_to_ray.dot(sphere_to_ray) - self.radius ** 2
        discriminant = b ** 2 - 4 * c
        if discriminant >= 0:
            distance = (-b - math.sqrt(discriminant)) / 2
            if distance > 0:
                return distance
        return None

    def normal(self, surface_point: Point):
        """Returns surface normal to the point on sphere surface"""
        return (surface_point - self.center).normalize()
