import rasterio
from io3d import raster_to_mesh, export_ply

# Path to the DEM file
path = "data/dem_ndsi.tif"

with rasterio.open(path) as src:
    elev = src.read(1).astype('float32')
    ndsi_1 = src.read(2).astype('uint8')  
    ndsi_2 = src.read(3).astype('uint8')

mesh = raster_to_mesh(elev, [ndsi_1, ndsi_2])
export_ply(mesh, "squamish_ndsi.ply")
