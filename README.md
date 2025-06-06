# 3D IO Mesh Core

This repository provides a Rust library with Python bindings for converting elevation rasters to triangle meshes. The library exposes functions to build a mesh and export it to the PLY format for use in Blender or other tools.

## Building

Create a virtual environment and install `maturin` to build the extension module:

```bash
python -m venv .venv
source .venv/bin/activate
pip install maturin numpy
maturin develop --release
```

## Demo

Run the example script to generate a sample mesh and write `demo.ply`:

```bash
python rust_core_mesh/examples/demo.py
```

The resulting `demo.ply` can be imported into Blender with vertex colors.
