use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Vertex {
    #[pyo3(get)]
    pub x: f32,
    #[pyo3(get)]
    pub y: f32,
    #[pyo3(get)]
    pub z: f32,
    #[pyo3(get)]
    pub r: u8,
    #[pyo3(get)]
    pub g: u8,
    #[pyo3(get)]
    pub b: u8,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Face {
    #[pyo3(get)]
    pub vertex_indices: [usize; 3],
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Mesh {
    #[pyo3(get)]
    pub vertices: Vec<Vertex>,
    #[pyo3(get)]
    pub faces: Vec<[usize; 3]>,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct MeshFrame {
    #[pyo3(get)]
    pub time: String,
    #[pyo3(get)]
    pub mesh: Mesh,
}

