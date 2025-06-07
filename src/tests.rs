use super::convert::raster_to_mesh_native;
use ndarray::array;

#[test]
fn test_raster_to_mesh() {
    let arr = array![[0f32, 1.0], [2.0, 3.0]];
    let mesh = raster_to_mesh_native(arr.view(), None);    
    assert_eq!(mesh.vertices.len(), 4);
    assert_eq!(mesh.faces.len(), 2);
    assert_eq!(mesh.vertices[0].x, 0.0);
    assert_eq!(mesh.vertices[0].y, 0.0);
    assert_eq!(mesh.vertices[0].z, 0.0);
}
