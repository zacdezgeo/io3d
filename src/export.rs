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
    writeln!(file, "property uchar red")?;
    writeln!(file, "property uchar green")?;
    writeln!(file, "property uchar blue")?;
    writeln!(file, "element face {}", mesh.faces.len())?;
    writeln!(file, "property list uchar int vertex_indices")?;
    writeln!(file, "end_header")?;

    for v in &mesh.vertices {
        let color = v.colors.get(0).copied().unwrap_or([255, 255, 255]);
        writeln!(file, "{} {} {} {} {} {}", v.x, v.y, v.z, color[0], color[1], color[2])?;
    }

    for f in &mesh.faces {
        writeln!(file, "3 {} {} {}", f[0], f[1], f[2])?;
    }

    Ok(())
}

