from color import Color
from PIL import Image as PILImage

class Image:
    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.pixels = [Color(0, 0, 0) for _ in range(width * height)]

    def set_pixel(self, x: int, y: int, color: Color):
        self.pixels[y * self.width + x] = color

    def save_as_ppm(self, file_path:str):
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

    def save_as_png(self, file_path:str):
        img = PILImage.new("RGB", (self.width, self.height))
        for y in range(self.height):
            for x in range(self.width):
                c = self.pixels[y * self.width + x]
                img.putpixel((x, y), ((int(255 * c.x), int(255 * c.y), int(255 * c.z))))
        img.save(file_path)

EOL = "\n"
COLORS = 255
