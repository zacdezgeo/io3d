import numpy as np
from rust_core_mesh import raster_to_mesh


def test_raster_to_mesh_simple():
    elev = np.array([[0.0, 1.0], [2.0, 3.0]], dtype=np.float32)
    mesh = raster_to_mesh(elev, None)
    assert len(mesh.vertices) == 4
    assert len(mesh.faces) == 2
    v0 = mesh.vertices[0]
    assert (v0.x, v0.y, v0.z, v0.r, v0.g, v0.b) == (0.0, 0.0, 0.0, 255, 255, 255)
