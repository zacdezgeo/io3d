import numpy as np
from rust_core_mesh import raster_to_mesh, export_ply

height, width = 100, 100
elev = np.random.rand(height, width).astype(np.float32) * 10
colors = np.zeros((height, width, 3), dtype=np.uint8)
colors[..., 0] = np.linspace(0, 255, width, dtype=np.uint8)
colors[..., 1] = np.linspace(0, 255, height, dtype=np.uint8)[:, None]

mesh = raster_to_mesh(elev, colors)
export_ply(mesh, "demo.ply")
print("Exported demo.ply")
