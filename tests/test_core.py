import numpy as np
from io3d import raster_to_mesh, raster_to_mesh_styled_py


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
<<<<<<< ours
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
=======
    frame0 = np.array(
        [
            [[255, 0, 0], [0, 255, 0]],
            [[0, 0, 255], [255, 255, 0]],
        ],
        dtype=np.uint8,
    )
    frame1 = np.array(
        [
            [[0, 0, 0], [1, 1, 1]],
            [[2, 2, 2], [3, 3, 3]],
        ],
        dtype=np.uint8,
    )
    rgb = np.concatenate([frame0, frame1], axis=-1)
    mesh = raster_to_mesh(elev, rgb)
    assert mesh.vertices[0].colors[0] == [255, 0, 0]
    assert mesh.vertices[0].colors[1] == [0, 0, 0]


def test_raster_to_mesh_styled():
    elev = np.zeros((2, 2), dtype=np.float32)
    land = np.array([[0, 1], [1, 0]], dtype=np.float32)
    overlay = np.zeros((1, 2, 2), dtype=np.float32)

    style = {
        "base_layer": {
            "type": "categorical",
            "mapping": {0: [0, 0, 0], 1: [255, 0, 0]},
        },
        "overlays": [
            (
                "o",
                {
                    "type": "continuous",
                    "ramp": [[0, 0, 255], [255, 255, 255]],
                    "min": 0.0,
                    "max": 1.0,
                },
            )
        ],
    }
    mesh = raster_to_mesh_styled_py(elev, style, land, overlay)
    assert mesh.vertices[0].colors[0] == [0, 0, 0]
    assert len(mesh.vertices[0].colors) == 2
>>>>>>> theirs
