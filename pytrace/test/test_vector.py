import pytest

import vector
from vector import Vector3


def test_print():
    print()
    print(Vector3(0, 0, 0))


@pytest.mark.parametrize("v1,expected", [(Vector3(0, 0, 0), 0), (Vector3(-2, 2, 1), 3)])
def test_magnitude(v1, expected):
    assert v1.magnitude() == expected


@pytest.mark.parametrize(
    "v1,v2,expected",
    [
        (Vector3(0, 0, 0), Vector3(0, 0, 0), Vector3(0, 0, 0)),
        (Vector3(1, 0, 0), Vector3(0, 0, 0), Vector3(1, 0, 0)),
        (Vector3(1, 1, 1), Vector3(1, 1, 1), Vector3(2, 2, 2)),
        (Vector3(1, 1, 1), Vector3(-1, -1, -1), Vector3(0, 0, 0)),
    ],
)
def test_add(v1, v2, expected):
    assert v1 + v2 == expected


@pytest.mark.parametrize(
    "v1,v2,expected",
    [
        (Vector3(0, 0, 0), 3, Vector3(0, 0, 0)),
        (Vector3(1, 0, 0), 3, Vector3(3, 0, 0)),
        (Vector3(1, 1, 1), 3, Vector3(3, 3, 3)),
        (Vector3(1, 1, 1), 0, Vector3(0, 0, 0)),
    ],
)
def test_mul(v1, v2, expected):
    assert v1 * v2 == expected


@pytest.mark.parametrize(
    "v1,v2,expected",
    [
        (Vector3(0, 0, 0), 3, Vector3(0, 0, 0)),
        (Vector3(3, 0, 0), 3, Vector3(1, 0, 0)),
        (Vector3(9, 9, 9), 3, Vector3(3, 3, 3)),
    ],
)
def test_div(v1, v2, expected):
    assert v1 / v2 == expected


@pytest.mark.parametrize("v1,v2", [(Vector3(0, 0, 0), vector.ZERO), (Vector3(1, 1, 1), Vector3(2, 2, 2))])
def test_normalize(v1, v2):
    assert v1.normalize() == v2.normalize()
