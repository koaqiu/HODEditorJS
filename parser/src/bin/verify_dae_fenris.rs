use hwr_hod_parser::hod::HODModel;
use hwr_hod_parser::dae::parse_dae;

fn main() {
    let hod_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_fenris/ter_fenris_2.0_original.hod";
    let dae_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_fenris/ter_fenris.DAE";
    
    let hod_bytes = std::fs::read(hod_path).unwrap();
    let hod_model = HODModel::parse_with_external(&hod_bytes, Some(hod_path), None).unwrap();
    
    let dae_str = std::fs::read_to_string(dae_path).unwrap();
    let dae_model = parse_dae(&dae_str).unwrap();
    
    println!("--- MESHES ---");
    println!("HOD Meshes: {}", hod_model.meshes.len());
    println!("DAE Meshes: {}", dae_model.meshes.len());
    
    let mut hod_meshes = hod_model.meshes.clone();
    hod_meshes.sort_by_key(|m| (m.name.clone(), m.lod));
    
    let mut dae_meshes = dae_model.meshes.clone();
    dae_meshes.sort_by_key(|m| (m.name.clone(), m.lod));
    
    for (i, (h_mesh, d_mesh)) in hod_meshes.iter().zip(dae_meshes.iter()).enumerate() {
        let h_verts: usize = h_mesh.parts.iter().map(|p| p.vertices.len()).sum();
        let h_faces: usize = h_mesh.parts.iter().map(|p| p.indices.len() / 3).sum();
        let d_verts: usize = d_mesh.parts.iter().map(|p| p.vertices.len()).sum();
        let d_faces: usize = d_mesh.parts.iter().map(|p| p.indices.len() / 3).sum();
        
        let h_mat = h_mesh.parts.iter().map(|p| p.material_index).collect::<Vec<_>>();
        let d_mat = d_mesh.parts.iter().map(|p| p.material_index).collect::<Vec<_>>();
        
        println!("Mesh {}: HOD name={} lod={} vs DAE name={} lod={}", i, h_mesh.name, h_mesh.lod, d_mesh.name, d_mesh.lod);
        if h_verts != d_verts || h_faces != d_faces {
            println!("  MISMATCH! HOD (v: {}, f: {}) vs DAE (v: {}, f: {})", h_verts, h_faces, d_verts, d_faces);
        }
        if h_mat != d_mat {
            println!("  MAT MISMATCH! HOD part mats: {:?} vs DAE part mats: {:?}", h_mat, d_mat);
        }
    }
    
    println!("--- MATERIALS ---");
    println!("HOD Materials: {}", hod_model.materials.len());
    println!("DAE Materials: {}", dae_model.materials.len());
    for m in &dae_model.materials {
        println!("  DAE Material: {} (shader: {})", m.name, m.shader_name);
    }
    for m in &hod_model.materials {
        println!("  HOD Material: {} (shader: {})", m.name, m.shader_name);
    }
    
    println!("--- JOINTS (NODES) ---");
    println!("HOD Joints: {}", hod_model.joints.len());
    println!("DAE Joints: {}", dae_model.joints.len());
    
    println!("--- MARKERS ---");
    println!("HOD Markers: {}", hod_model.markers.len());
    println!("DAE Markers: {}", dae_model.markers.len());
    
    println!("--- ANIMATIONS ---");
    println!("HOD Animations: {}", hod_model.animations.len());
    println!("DAE Animations: {}", dae_model.animations.len());
    for a in &dae_model.animations {
        println!("  DAE Animation: {} (duration: {}, tracks: {})", a.name, a.duration, a.tracks.len());
    }
    for a in &hod_model.animations {
        println!("  HOD Animation: {} (duration: {}, tracks: {})", a.name, a.duration, a.tracks.len());
    }
}
