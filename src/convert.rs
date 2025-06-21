use numpy::{PyArray2, PyArrayDyn};
use pyo3::prelude::*;
use ndarray::{ArrayView2, ArrayViewD};

use crate::mesh::{Mesh, Vertex};

pub fn raster_to_mesh<'py>(
    elevation: &'py PyArray2<f32>,
    rgb: Option<&'py PyArrayDyn<u8>>,
) -> PyResult<Mesh> {
    let elev = unsafe { elevation.as_array() };
    let colors = rgb.map(|arr| unsafe { arr.as_array() });
    Ok(build_mesh(elev, colors))
}

pub fn raster_to_mesh_native(elev: ArrayView2<f32>, rgb: Option<ArrayViewD<u8>>) -> Mesh {
    build_mesh(elev, rgb)
}

fn build_mesh(elev: ArrayView2<f32>, rgb: Option<ArrayViewD<u8>>) -> Mesh {
    let (height, width) = (elev.shape()[0], elev.shape()[1]);

    let mut vertices = Vec::with_capacity(height * width);
    for i in 0..height {
        for j in 0..width {
            let z = elev[[i, j]];
            let colors = if let Some(ref col) = rgb {
                match col.ndim() {
                    3 => {
                        vec![[
                            col[[i, j, 0]],
                            col[[i, j, 1]],
                            col[[i, j, 2]],
                        ]]
                    }
                    4 => {
                        let frames = col.shape()[0];
                        (0..frames)
                            .map(|t| {
                                [
                                    col[[t, i, j, 0]],
                                    col[[t, i, j, 1]],
                                    col[[t, i, j, 2]],
                                ]
                            })
                            .collect()
                    }
                    _ => vec![[255u8, 255u8, 255u8]],
                }
            } else {
                vec![[255u8, 255u8, 255u8]]
            };
            vertices.push(Vertex { x: j as f32, y: i as f32, z, colors });
        }
    }

    let mut faces = Vec::with_capacity((height - 1) * (width - 1) * 2);
    for i in 0..height - 1 {
        for j in 0..width - 1 {
            let idx = |row: usize, col: usize| row * width + col;
            let v0 = idx(i, j);
            let v1 = idx(i, j + 1);
            let v2 = idx(i + 1, j);
            let v3 = idx(i + 1, j + 1);
            faces.push([v0, v2, v1]);
            faces.push([v2, v3, v1]);
        }
    }

    Mesh { vertices, faces }
}

