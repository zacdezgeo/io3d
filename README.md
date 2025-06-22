# io3d 

This repository provides a Rust library with Python bindings for converting elevation rasters to triangle meshes. The library exposes functions to build a mesh and export it to the PLY format for use in Blender or other tools.

## Building

Create a virtual environment and install `maturin` to build the extension module:

```bash
uv venv
uv add numpy maturin
source .venv/bin/activate
maturin develop --release
```

## Demo

Run the example script to convert a DEM to a mesh and write `squamish.ply`:

```bash
python examples/squamish_to_ply.py
```

The resulting `squamish.ply` can be imported into Blender with vertex colors.
