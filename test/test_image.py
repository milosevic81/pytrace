from color import Color
from image import Image


def test_image():
    img = Image(3, 2)
    img.pixels = [
        Color(1, 0, 0), Color(0, 1, 0), Color(0, 0, 1),
        Color(1, 1, 0), Color(1, 1, 1), Color(0.001, 0, 0),
    ]
    img.save("test.ppm")