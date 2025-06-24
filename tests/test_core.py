import numpy as np
from meshup import raster_to_mesh, raster_to_mesh_styled


def test_raster_to_mesh_simple():
    elev = np.array([[0.0, 1.0], [2.0, 3.0]], dtype=np.float32)
    mesh = raster_to_mesh(elev, None)
    assert len(mesh.vertices) == 4
    assert len(mesh.faces) == 2
    v0 = mesh.vertices[0]
    assert (
        v0.x,
        v0.y,
        v0.z,
        v0.colors[0][0],
        v0.colors[0][1],
        v0.colors[0][2],
    ) == (0.0, 0.0, 0.0, 255, 255, 255)
    