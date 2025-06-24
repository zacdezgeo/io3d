import rasterio
from meshup import raster_to_mesh_styled, export_ply

# Path to the DEM file
path = "data/dem_world_cover.tif"

with rasterio.open(path) as src:
    elev = src.read(1).astype("float32")
    landcover = src.read(2).astype("float32")
    nodata = src.nodata
    if nodata is not None:
        landcover[landcover == nodata] = 0
    landcover = landcover.round()

# ESA WorldCover hex color table converted to RGB values
worldcover_hex = {
    10: "006400",  # Tree cover
    20: "ffbb22",  # Shrubland
    30: "ffff4c",  # Grassland
    40: "f096ff",  # Cropland
    50: "fa0000",  # Built-up
    60: "b4b4b4",  # Bare / sparse vegetation
    70: "f0f0f0",  # Snow and ice
    80: "0064c8",  # Permanent water bodies
    90: "0096a0",  # Herbaceous wetland
    95: "00cf75",  # Mangroves
    100: "e1e1e1", # Moss and lichen
}

worldcover_colors = {
    k: [int(v[i : i + 2], 16) for i in (0, 2, 4)]
    for k, v in worldcover_hex.items()
}

style = {
    "base_layer": {
        "type": "categorical",
        "mapping": worldcover_colors,
    },
    "overlays": [],
}

mesh = raster_to_mesh_styled(elev, style, landcover, None)
export_ply(mesh, "squamish_color.ply")