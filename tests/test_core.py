import numpy as np
from io3d import raster_to_mesh


def test_raster_to_mesh_simple():
    elev = np.array([[0.0, 1.0], [2.0, 3.0]], dtype=np.float32)
    mesh = raster_to_mesh(elev, None)
    assert len(mesh.vertices) == 4
    assert len(mesh.faces) == 2
    v0 = mesh.vertices[0]
    assert (v0.x, v0.y, v0.z) == (0.0, 0.0, 0.0)
    assert v0.colors == [[255, 255, 255]]


def test_raster_to_mesh_time_colors():
    elev = np.zeros((2, 2), dtype=np.float32)
    rgb = np.array(
        [
            [
                [[255, 0, 0], [0, 255, 0]],
                [[0, 0, 255], [255, 255, 0]],
            ],
            [
                [[0, 0, 0], [1, 1, 1]],
                [[2, 2, 2], [3, 3, 3]],
            ],
        ],
        dtype=np.uint8,
    )
    mesh = raster_to_mesh(elev, rgb)
    assert mesh.vertices[0].colors[0] == [255, 0, 0]
    assert mesh.vertices[0].colors[1] == [0, 0, 0]
