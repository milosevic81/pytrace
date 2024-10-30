from color import Color


class Image:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.pixels = [Color(0, 0, 0) for _ in range(width * height)]

    def set_pixel(self, x: int, y: int, color: Color):
        self.pixels[y * self.width + x] = color

    def save(self, file_path:str):
        with open(file_path, 'w') as f:
            self.write(f)

    def write(self, f):
        # header
        f.write(f"P3 {self.width} {self.height}" + EOL)
        f.write(f"{COLORS}" + EOL)

        def clamp(v, low, high):
            return max(min(v, high), low)

        def to_byte(c):
            return clamp(int(COLORS * c), 0, 255)

        for i, c in enumerate(self.pixels):
            f.write(f"{to_byte(c.x)} {to_byte(c.y)} {to_byte(c.z)} ")
            if (i + 1) % self.width == 0:
                f.write(EOL)
            else:
                f.write("\t")


EOL = "\n"
COLORS = 255
