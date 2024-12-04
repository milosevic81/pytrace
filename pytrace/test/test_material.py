import pytest

from material import Material
from point import Point


@pytest.mark.parametrize("value,expected", [
    (Point(0,0,0), Color.WHITE),
])
def test_color_at(value, expected):
    material = CheckerMaterial(
        color1=Color.WHITE,
        color2=Color.BLACK,
        ambient=0.05,
        diffuse=1.0,
        specular=1.0,
        reflection=0.5)
    assert material.color_at(value) == expected
