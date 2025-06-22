import rasterio
from io3d import raster_to_mesh_styled_py, export_ply

"""Example converting `squamish_dem.tif` to a colored mesh.

The raster contains two bands:
    1. SRTM elevation values
    2. ESA WorldCover categorical codes

The style map below assigns RGB colors to each WorldCover value.
"""

path = "data/squamish_dem.tif"

with rasterio.open(path) as src:
    elevation = src.read(1).astype("float32")
    landcover = src.read(2).astype("float32")

# Mapping from ESA WorldCover value to RGB color
worldcover_colors = {
    10: [0, 100, 0],     # Tree cover
    20: [255, 187, 34],  # Shrubland
    30: [255, 255, 76],  # Grassland
    40: [240, 150, 255], # Cropland
    50: [250, 0, 0],     # Built-up
    60: [180, 180, 180], # Bare / sparse vegetation
    70: [240, 240, 240], # Snow and ice
    80: [0, 100, 200],   # Permanent water bodies
    90: [0, 150, 160],   # Herbaceous wetland
    95: [0, 207, 117],   # Mangroves
    100: [250, 230, 160],# Moss and lichen
}

style = {
    "base_layer": {
        "type": "categorical",
        "mapping": worldcover_colors,
    },
    "overlays": [],
}

mesh = raster_to_mesh_styled_py(elevation, style, landcover, None)
export_ply(mesh, "squamish.ply")
print("Wrote squamish.ply")
