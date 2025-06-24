import numpy as np
from io3d import raster_to_mesh, raster_to_mesh_styled_py, write_frame_ply


def test_write_frame_ply(tmp_path):
    elev = np.array([[0.0, 1.0], [2.0, 3.0]], dtype=np.float32)
    rgb = np.zeros((2, 2, 6), dtype=np.uint8)
    rgb[:, :, :3] = [10, 20, 30]
    rgb[:, :, 3:] = [40, 50, 60]
    mesh = raster_to_mesh(elev, rgb)
    path = tmp_path / "frame1.ply"
    write_frame_ply(mesh, str(path), 1)
    assert path.exists()
    with open(path, "r") as f:
        lines = [next(f) for _ in range(10)]
    assert lines[0].strip() == "ply"
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
