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
    /// Per-vertex colors for each time step.
    ///
    /// The inner array stores an RGB triplet for a given frame.
    #[pyo3(get)]
    pub colors: Vec<[u8; 3]>,
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

