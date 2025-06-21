use super::convert::raster_to_mesh_native;
use ndarray::{array, ArrayViewD, Array4};

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
    let colors = Array4::<u8>::from_shape_fn((2, 2, 2, 3), |(t, i, j, c)| {
        (t + i + j + c) as u8
    });
    let mesh = raster_to_mesh_native(elev.view(), Some(colors.view().into_dyn()));
    assert_eq!(mesh.vertices[0].colors.len(), 2);
}
