import pytest

from color import Color


@pytest.mark.parametrize("value,expected", [
    ("#FF0000", Color(1, 0, 0)),
    ("#FFFFFF", Color(1, 1, 1)),
    ("#000000", Color(0, 0, 0))
])
def test_from_hex(value, expected):
    assert Color.from_hex(value) == expected
