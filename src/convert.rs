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
                        let channels = col.shape()[2];
                        if channels % 3 != 0 {
                            vec![[255u8, 255u8, 255u8]]
                        } else {
                            let frames = channels / 3;
                            (0..frames)
                                .map(|t| {
                                    [
                                        col[[i, j, 3 * t]],
                                        col[[i, j, 3 * t + 1]],
                                        col[[i, j, 3 * t + 2]],
                                    ]
                                })
                                .collect()
                        }
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


use crate::style::{parse_style_spec, StyleSpec, BandStyle};
use numpy::{PyArray3};
use ndarray::ArrayView3;

#[pyfunction(signature = (elevation, style_dict, base=None, overlays=None))]
pub fn raster_to_mesh_styled<'py>(
    elevation: &'py PyArray2<f32>,
    style_dict: &'py pyo3::types::PyDict,
    base: Option<&'py PyArray2<f32>>,
    overlays: Option<&'py PyArray3<f32>>,
) -> PyResult<Mesh> {
    let elev = unsafe { elevation.as_array() };
    let base_arr = base.map(|b| unsafe { b.as_array() });
    let overlay_arr = overlays.map(|o| unsafe { o.as_array() });
    let spec = parse_style_spec(style_dict)?;
    Ok(build_mesh_styled(elev, base_arr, overlay_arr, &spec))
}

pub fn raster_to_mesh_styled_native(
    elev: ArrayView2<f32>,
    base: Option<ArrayView2<f32>>,
    overlays: Option<ArrayView3<f32>>,
    spec: &StyleSpec,
) -> Mesh {
    build_mesh_styled(elev, base, overlays, spec)
}

fn apply_style(value: f32, style: &BandStyle) -> [u8; 3] {
    style.color(value)
}

fn build_mesh_styled(
    elev: ArrayView2<f32>,
    base: Option<ArrayView2<f32>>,
    overlays: Option<ArrayView3<f32>>,
    spec: &StyleSpec,
) -> Mesh {
    let (h, w) = (elev.shape()[0], elev.shape()[1]);
    let overlay_frames = overlays.as_ref().map(|o| o.shape()[0]).unwrap_or(0);
    let mut vertices = Vec::with_capacity(h * w);
    for i in 0..h {
        for j in 0..w {
            let z = elev[[i, j]];
            let mut colors = Vec::new();
            if let Some(ref base_style) = spec.base_layer {
                let val = base
                    .as_ref()
                    .map(|b| b[[i, j]])
                    .unwrap_or(0.0);
                colors.push(apply_style(val, base_style));
            } else {
                colors.push([255, 255, 255]);
            }
            if let Some(ref overlays_arr) = overlays {
                for (idx, (_, style)) in spec.overlays.iter().enumerate() {
                    if idx < overlay_frames {
                        let val = overlays_arr[[idx, i, j]];
                        colors.push(apply_style(val, style));
                    }
                }
            }
            vertices.push(Vertex { x: j as f32, y: i as f32, z, colors });
        }
    }

    let mut faces = Vec::with_capacity((h - 1) * (w - 1) * 2);
    for i in 0..h - 1 {
        for j in 0..w - 1 {
            let idx = |row: usize, col: usize| row * w + col;
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
