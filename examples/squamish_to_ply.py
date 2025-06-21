import rasterio
from io3d import raster_to_mesh, export_ply

# Path to the DEM file
path = "data/squamish_dem.tif"

with rasterio.open(path) as src:
    elev = src.read(1).astype('float32')

mesh = raster_to_mesh(elev, None)
export_ply(mesh, "squamish.ply")
print("Wrote squamish.ply")
