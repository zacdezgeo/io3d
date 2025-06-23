import rasterio
from io3d import raster_to_mesh, export_ply

# Path to the DEM file
path = "data/dem_world_cover.tif"

with rasterio.open(path) as src:
    elev = src.read(1).astype('float32')
    color = src.read(2).astype('uint8')  

mesh = raster_to_mesh(elev, color)
export_ply(mesh, "squamish_color.ply")
