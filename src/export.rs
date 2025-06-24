use std::fs::File;
use std::io::{Result, Write};

use crate::mesh::Mesh;

pub fn export_ply(mesh: &Mesh, path: &str) -> Result<()> {
    let mut file = File::create(path)?;

    writeln!(file, "ply")?;
    writeln!(file, "format ascii 1.0")?;
    writeln!(file, "element vertex {}", mesh.vertices.len())?;
    writeln!(file, "property float x")?;
    writeln!(file, "property float y")?;
    writeln!(file, "property float z")?;
    let frame_count = mesh
        .vertices
        .first()
        .map(|v| v.colors.len())
        .unwrap_or(0);
    for t in 0..frame_count.max(1) {
        if frame_count <= 1 {
            writeln!(file, "property uchar red")?;
            writeln!(file, "property uchar green")?;
            writeln!(file, "property uchar blue")?;
        } else {
            writeln!(file, "property uchar red_{}", t)?;
            writeln!(file, "property uchar green_{}", t)?;
            writeln!(file, "property uchar blue_{}", t)?;
        }
    }
    writeln!(file, "element face {}", mesh.faces.len())?;
    writeln!(file, "property list uchar int vertex_indices")?;
    writeln!(file, "end_header")?;

    for v in &mesh.vertices {
        let mut line = format!("{} {} {}", v.x, v.y, v.z);
        if v.colors.is_empty() {
            line.push_str(" 255 255 255");
        } else if v.colors.len() == 1 {
            let c = v.colors[0];
            line.push_str(&format!(" {} {} {}", c[0], c[1], c[2]));
        } else {
            for c in &v.colors {
                line.push_str(&format!(" {} {} {}", c[0], c[1], c[2]));
            }
        }
        writeln!(file, "{}", line)?;
    }

    for f in &mesh.faces {
        writeln!(file, "3 {} {} {}", f[0], f[1], f[2])?;
    }

    Ok(())
}

