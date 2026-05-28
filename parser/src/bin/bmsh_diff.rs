use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::hod::HODModel;
use std::fs;

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: bmsh_diff <hodor.hod> <generated.hod>");
        std::process::exit(1);
    }

    let hodor_bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let gen_bytes = fs::read(&args[2]).map_err(|e| e.to_string())?;

    let hodor_model = HODModel::parse(&hodor_bytes).map_err(|e| e.to_string())?;
    let gen_model = HODModel::parse(&gen_bytes).map_err(|e| e.to_string())?;

    println!("=== BMSH Comparison ===");
    println!(
        "HODOR: {} meshes, {} materials",
        hodor_model.meshes.len(),
        hodor_model.materials.len()
    );
    println!(
        "Gen:   {} meshes, {} materials",
        gen_model.meshes.len(),
        gen_model.materials.len()
    );

    for hodor_mesh in &hodor_model.meshes {
        let gen_mesh = gen_model
            .meshes
            .iter()
            .find(|m| m.name == hodor_mesh.name && m.lod == hodor_mesh.lod);
        if let Some(gen_mesh) = gen_mesh {
            println!(
                "\n--- {} LOD {} ---",
                hodor_mesh.name, hodor_mesh.lod
            );
            println!(
                "  HODOR: {} parts, {} total verts, {} total indices",
                hodor_mesh.parts.len(),
                hodor_mesh
                    .parts
                    .iter()
                    .map(|p| p.vertices.len())
                    .sum::<usize>(),
                hodor_mesh
                    .parts
                    .iter()
                    .map(|p| p.indices.len())
                    .sum::<usize>()
            );
            println!(
                "  Gen:   {} parts, {} total verts, {} total indices",
                gen_mesh.parts.len(),
                gen_mesh
                    .parts
                    .iter()
                    .map(|p| p.vertices.len())
                    .sum::<usize>(),
                gen_mesh
                    .parts
                    .iter()
                    .map(|p| p.indices.len())
                    .sum::<usize>()
            );

            for (part_idx, (hodor_part, gen_part)) in hodor_mesh
                .parts
                .iter()
                .zip(&gen_mesh.parts)
                .enumerate()
            {
                println!(
                    "  Part {}: material={}/{} verts={}/{} indices={}/{}",
                    part_idx,
                    hodor_part.material_index,
                    gen_part.material_index,
                    hodor_part.vertices.len(),
                    gen_part.vertices.len(),
                    hodor_part.indices.len(),
                    gen_part.indices.len()
                );

                if !hodor_part.vertices.is_empty() && !gen_part.vertices.is_empty() {
                    let h = &hodor_part.vertices[0];
                    let g = &gen_part.vertices[0];
                    println!(
                        "    First vert HODOR: pos={:?} normal={:?} uv={:?} tangent={:?} binormal={:?}",
                        h.position, h.normal, h.uv, h.tangent, h.binormal
                    );
                    println!(
                        "    First vert Gen:   pos={:?} normal={:?} uv={:?} tangent={:?} binormal={:?}",
                        g.position, g.normal, g.uv, g.tangent, g.binormal
                    );
                }
            }
        } else {
            println!(
                "\n--- {} LOD {} --- MISSING in generated",
                hodor_mesh.name, hodor_mesh.lod
            );
        }
    }

    Ok(())
}
