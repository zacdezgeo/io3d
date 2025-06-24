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

#[pymethods]
impl Vertex {
    /// Return the color for the given frame as a hexadecimal string.
    ///
    /// If the requested frame does not exist, white (`ffffff`) is returned.
    #[pyo3(signature = (frame=0))]
    fn color_hex(&self, frame: usize) -> String {
        let c = self.colors.get(frame).copied().unwrap_or([255, 255, 255]);
        format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2])
    }

    /// Return all colors for this vertex as hexadecimal strings.
    fn colors_hex(&self) -> Vec<String> {
        self
            .colors
            .iter()
            .map(|c| format!("{:02x}{:02x}{:02x}", c[0], c[1], c[2]))
            .collect()
    }
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

