import rasterio
import numpy as np
from io3d import raster_to_mesh_styled_py, write_frame_ply

"""Example converting `dem_ndsi.tif` with two NDSI layers to a mesh."""

path = "data/dem_ndsi.tif"

with rasterio.open(path) as src:
    elevation = src.read(1).astype("float32")
    ndsi_may = src.read(2).astype("float32")
    ndsi_june = src.read(3).astype("float32")

style = {
    "overlays": [
        (
            "ndsi_may",
            {
                "type": "continuous",
                "ramp": [[0, 0, 255], [255, 255, 255]],
                "min": 0.0,
                "max": 1.0,
            },
        ),
        (
            "ndsi_june",
            {
                "type": "continuous",
                "ramp": [[0, 0, 255], [255, 255, 255]],
                "min": 0.0,
                "max": 1.0,
            },
        ),
    ],
}

overlays = np.stack([ndsi_may, ndsi_june], axis=0)
for idx in range(overlays.shape[0]):
    out = f"dem_ndsi_t{idx}.ply"
    write_frame_ply(mesh, out, idx)
    print(f"Wrote {out}")
print("Wrote dem_ndsi.ply")
