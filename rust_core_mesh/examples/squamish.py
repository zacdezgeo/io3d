import numpy as np
import rasterio
from rust_core_mesh import raster_to_mesh, export_ply

p = "dem_world_cover.tif"

color_map = {
    10: (0x00, 0x64, 0x00),   # Tree cover
    20: (0xff, 0xbb, 0x22),   # Shrubland
    30: (0xff, 0xff, 0x4c),   # Grassland
    40: (0xf0, 0x96, 0xff),   # Cropland
    50: (0xfa, 0x00, 0x00),   # Built-up
    60: (0xb4, 0xb4, 0xb4),   # Bare / sparse vegetation
    70: (0xf0, 0xf0, 0xf0),   # Snow and ice
    80: (0x00, 0x64, 0xc8),   # Permanent water bodies
    90: (0x00, 0x96, 0xa0),   # Herbaceous wetland
    95: (0x00, 0xcf, 0x75),   # Mangroves
    100: (0xfa, 0xe6, 0xa0),  # Moss and lichen
}

with rasterio.open(p) as src:
    elevation = src.read(1).astype(np.float32)
    landcover = src.read(2)

# Build an RGB image according to the color map
colors = np.zeros((landcover.shape[0], landcover.shape[1], 3), dtype=np.uint8)
for value, rgb in color_map.items():
    colors[landcover == value] = rgb

mesh = raster_to_mesh(elevation, colors)
export_ply(mesh, "output.ply")
