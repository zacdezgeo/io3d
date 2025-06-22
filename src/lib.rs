use pyo3::prelude::*;
use numpy::{PyArray2, PyArrayDyn};

mod mesh;
mod export;
mod convert;
mod style;
#[cfg(test)]
mod tests;

pub use crate::mesh::{Vertex, Face, Mesh, MeshFrame};
pub use crate::style::{ColorRamp, StyleSpec};
use crate::export::export_ply as export_ply_rs;
use crate::convert::{raster_to_mesh as raster_to_mesh_rs, raster_to_mesh_styled};

#[pyfunction]
fn raster_to_mesh(
    elevation: &PyArray2<f32>,
    rgb: Option<&PyArrayDyn<u8>>,
) -> PyResult<Mesh> {
    raster_to_mesh_rs(elevation, rgb)
}

#[pyfunction(signature = (elevation, style, base=None, overlays=None))]
fn raster_to_mesh_styled_py(
    elevation: &PyArray2<f32>,
    style: &pyo3::types::PyDict,
    base: Option<&PyArray2<f32>>,
    overlays: Option<&numpy::PyArray3<f32>>,
) -> PyResult<Mesh> {
    raster_to_mesh_styled(elevation, style, base, overlays)
}

#[pyfunction]
fn export_ply(mesh: &Mesh, path: &str) -> PyResult<()> {
    export_ply_rs(mesh, path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(e.to_string()))
}

#[pymodule]
fn io3d(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vertex>()?;
    m.add_class::<Face>()?;
    m.add_class::<Mesh>()?;
    m.add_class::<MeshFrame>()?;
    m.add_class::<ColorRamp>()?;
    m.add_class::<StyleSpec>()?;
    m.add_function(wrap_pyfunction!(raster_to_mesh, m)?)?;
    m.add_function(wrap_pyfunction!(raster_to_mesh_styled_py, m)?)?;
    m.add_function(wrap_pyfunction!(export_ply, m)?)?;
    Ok(())
}

