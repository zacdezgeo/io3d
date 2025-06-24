import rasterio
from meshup import raster_to_mesh, export_ply

# Path to the DEM file
path = "data/squamish.tif"

with rasterio.open(path) as src:
    elev = src.read(1).astype('float32')

mesh = raster_to_mesh(elev, None)
export_ply(mesh, "squamish.ply")
