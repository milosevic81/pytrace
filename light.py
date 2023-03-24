from color import Color
from point import Point


class Light:
    "Ligh representa a point light source of certain color"

    def __init__(self, position: Point, color: Color = Color.WHITE):
        self.position = position
        self.color = color
