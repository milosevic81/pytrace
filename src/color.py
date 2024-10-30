from vector import Vector3


class Color(Vector3):
    """Color RGB triplets. An alias for vector"""

    @classmethod
    def from_hex(cls, param: str):
        return cls(int(param[1:3], 16) / Color.COLOR_MAX,
                   int(param[3:5], 16) / Color.COLOR_MAX,
                   int(param[5:7], 16) / Color.COLOR_MAX)


Color.WHITE = Color(1, 1, 1)
Color.RED = Color(1, 0, 0)
Color.GREEN = Color(0, 1, 0)
Color.BLUE = Color(0, 0, 1)
Color.BLACK = Color(0, 0, 0)

Color.COLOR_MAX = 255.0
