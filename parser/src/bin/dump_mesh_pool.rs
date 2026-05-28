use hwr_hod_parser::hod::HODModel;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| {
        "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod".to_string()
    });
    let bytes = fs::read(&path)?;
    let model = HODModel::parse(&bytes)?;

    println!("Parsed HOD: {}", path);
    println!("Meshes: {}", model.meshes.len());
    for (i, mesh) in model.meshes.iter().enumerate() {
        println!("Mesh {}: {}", i, mesh.name);
        for (j, part) in mesh.parts.iter().enumerate() {
            println!(
                "  Part {}: Vertices: {}, Indices: {}",
                j,
                part.vertices.len(),
                part.indices.len()
            );

            // Print first few vertices
            if !part.vertices.is_empty() {
                println!(
                    "    First vertex: pos={:?}, normal={:?}, uv={:?}, tangent={:?}, binormal={:?}",
                    part.vertices[0].position,
                    part.vertices[0].normal,
                    part.vertices[0].uv,
                    part.vertices[0].tangent,
                    part.vertices[0].binormal
                );
            }
        }
    }

    Ok(())
}
