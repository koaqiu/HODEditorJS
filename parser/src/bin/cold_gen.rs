use hwr_hod_parser::hod::{HODModel, HODVertex, Vector3};

fn main() -> Result<(), String> {
    // Load the HODOR file to get the actual collision mesh
    let hodor_bytes = std::fs::read(
        "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_centaur/ter_centaur_hodor.hod"
    ).map_err(|e| e.to_string())?;
    let hodor_model = HODModel::parse(&hodor_bytes).map_err(|e| e.to_string())?;

    println!("=== Collision Mesh Comparison ===");
    println!("HODOR collision meshes: {}", hodor_model.collision_meshes.len());

    for (i, cm) in hodor_model.collision_meshes.iter().enumerate() {
        println!("\n--- Collision Mesh {} ---", i);
        println!("  Name: {}", cm.name);
        println!("  Min extents: ({:.4}, {:.4}, {:.4})", cm.min_extents.x, cm.min_extents.y, cm.min_extents.z);
        println!("  Max extents: ({:.4}, {:.4}, {:.4})", cm.max_extents.x, cm.max_extents.y, cm.max_extents.z);
        println!("  Center: ({:.4}, {:.4}, {:.4})", cm.center.x, cm.center.y, cm.center.z);
        println!("  Radius: {:.4}", cm.radius);
        println!("  Mesh parts: {}", cm.mesh.parts.len());

        for (j, part) in cm.mesh.parts.iter().enumerate() {
            println!("  Part {}: material={} mask=0x{:04X} verts={} indices={}",
                j, part.material_index, part.vertex_mask, part.vertices.len(), part.indices.len());

            if !part.vertices.is_empty() {
                println!("    First 10 vertices:");
                for (k, v) in part.vertices.iter().take(10).enumerate() {
                    println!("      vert[{}]: pos=({:.4}, {:.4}, {:.4})", k, v.position.x, v.position.y, v.position.z);
                }
                if part.vertices.len() > 10 {
                    println!("    ... ({} more vertices)", part.vertices.len() - 10);
                }
            }

            if !part.indices.is_empty() {
                println!("    First 30 indices: {:?}", &part.indices[..30.min(part.indices.len())]);
                if part.indices.len() > 30 {
                    println!("    ... ({} more indices)", part.indices.len() - 30);
                }
            }
        }
    }

    // Now generate a box from the extents
    if let Some(cm) = hodor_model.collision_meshes.first() {
        let min = &cm.min_extents;
        let max = &cm.max_extents;

        println!("\n=== Generated Box Vertices ===");
        let box_verts = generate_box(min, max);
        println!("Box vertices: {}", box_verts.len());
        for (i, v) in box_verts.iter().enumerate() {
            println!("  vert[{}]: ({:.4}, {:.4}, {:.4})", i, v.x, v.y, v.z);
        }

        let box_indices = vec![
            0, 1, 2, 0, 2, 3,  // front
            4, 6, 5, 4, 7, 6,  // back
            0, 4, 5, 0, 5, 1,  // left
            2, 6, 7, 2, 7, 3,  // right
            0, 3, 7, 0, 7, 4,  // top
            1, 5, 6, 1, 6, 2,  // bottom
        ];
        println!("Box indices: {}", box_indices.len());
        println!("Box triangles: {}", box_indices.len() / 3);
    }

    Ok(())
}

fn generate_box(min: &Vector3, max: &Vector3) -> Vec<Vector3> {
    vec![
        Vector3 { x: min.x, y: min.y, z: min.z },  // 0: front-bottom-left
        Vector3 { x: max.x, y: min.y, z: min.z },  // 1: front-bottom-right
        Vector3 { x: max.x, y: max.y, z: min.z },  // 2: front-top-right
        Vector3 { x: min.x, y: max.y, z: min.z },  // 3: front-top-left
        Vector3 { x: min.x, y: min.y, z: max.z },  // 4: back-bottom-left
        Vector3 { x: max.x, y: min.y, z: max.z },  // 5: back-bottom-right
        Vector3 { x: max.x, y: max.y, z: max.z },  // 6: back-top-right
        Vector3 { x: min.x, y: max.y, z: max.z },  // 7: back-top-left
    ]
}
