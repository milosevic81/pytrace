from color import Color
from point import Point


class Material:
    def __init__(
            self,
            color: Color = Color.RED,
            ambient: float = 0.05,
            diffuse: float = 1.0,
            specular: float = 1.0,
            reflection: float = 0.5):
        self.reflection = reflection
        self.color = color
        self.ambient = ambient
        self.diffuse = diffuse
        self.specular = specular

    def color_at(self, position: Point):
        return self.color


class CheckerMaterial:
    def __init__(
            self,
            color1: Color = Color.WHITE,
            color2: Color = Color.BLACK,
            ambient: float = 0.05,
            diffuse: float = 1.0,
            specular: float = 1.0,
            reflection: float = 0.5):
        self.reflection = reflection
        self.color1 = color1
        self.color2 = color2
        self.ambient = ambient
        self.diffuse = diffuse
        self.specular = specular

    def color_at(self, position: Point):
        # print(position)
        if int((position.x + 5.0) * 3.0) % 2 == int(position.z * 3.0) % 2:
            return self.color1
        else:
            return self.color2
