from point import Point
from vector import Vector3


class Ray:
    """Ray is defined with an origin and normalized direction"""
    def __init__(self, origin: Point, direction: Vector3):
        self.origin = origin
        self.direction = direction.normalize()
