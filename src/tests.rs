use super::convert::{raster_to_mesh_native, raster_to_mesh_styled_native};
use super::style::{StyleSpec, BandStyle, ColorRamp};
use ndarray::{array, ArrayViewD, Array3, Array2};
use std::collections::HashMap;

#[test]
fn test_raster_to_mesh() {
    let arr = array![[0f32, 1.0], [2.0, 3.0]];
    let mesh = raster_to_mesh_native(arr.view(), None::<ArrayViewD<u8>>);
    assert_eq!(mesh.vertices.len(), 4);
    assert_eq!(mesh.faces.len(), 2);
    assert_eq!(mesh.vertices[0].x, 0.0);
    assert_eq!(mesh.vertices[0].y, 0.0);
    assert_eq!(mesh.vertices[0].z, 0.0);
}

#[test]
fn test_raster_to_mesh_time_colors() {
    let elev = array![[0f32, 0.0], [0.0, 0.0]];
    let colors = Array3::<u8>::from_shape_fn((2, 2, 6), |(i, j, c)| {
        let frame = c / 3;
        (frame + i + j + (c % 3)) as u8
    });
    let mesh = raster_to_mesh_native(elev.view(), Some(colors.view().into_dyn()));
    assert_eq!(mesh.vertices[0].colors.len(), 2);
}

#[test]
fn test_raster_to_mesh_styled() {
    let elev = array![[0f32, 0.0], [0.0, 0.0]];
    let land = array![[0f32, 1.0], [1.0, 0.0]];
    let overlay = Array3::<f32>::zeros((1, 2, 2));

    let mut map = HashMap::new();
    map.insert(0u8, [0u8, 0u8, 0u8]);
    map.insert(1u8, [255u8, 0u8, 0u8]);
    let base_style = BandStyle::Categorical(map);
    let overlay_style = BandStyle::Continuous(ColorRamp::new([0,0,255],[255,255,255]), 0.0, 1.0);

    let spec = StyleSpec { base_layer: Some(base_style), overlays: vec![("o".into(), overlay_style)] };

    let mesh = raster_to_mesh_styled_native(elev.view(), Some(land.view()), Some(overlay.view()), &spec);
    assert_eq!(mesh.vertices[0].colors.len(), 2);
}

#[test]
fn test_categorical_rounding() {
    let elev = array![[0f32]];
    let land = array![[9.6f32]];

    let mut map = HashMap::new();
    map.insert(10u8, [1u8, 2u8, 3u8]);
    let spec = StyleSpec {
        base_layer: Some(BandStyle::Categorical(map)),
        overlays: Vec::new(),
    };

    let mesh = raster_to_mesh_styled_native(elev.view(), Some(land.view()), None, &spec);
    assert_eq!(mesh.vertices[0].colors[0], [1, 2, 3]);
}
