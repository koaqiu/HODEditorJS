use hwr_hod_parser::hod::HODModel;
use hwr_hod_parser::dae::parse_dae;

fn main() {
    let hod_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_zephyrus/ter_zephyrus_2.0_original.hod";
    let dae_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_zephyrus/ter_zephyrus.DAE";
    
    let hod_bytes = std::fs::read(hod_path).unwrap();
    let hod_model = HODModel::parse(&hod_bytes).unwrap();
    
    let dae_str = std::fs::read_to_string(dae_path).unwrap();
    let dae_model = parse_dae(&dae_str).unwrap();
    
    let mut hod_meshes = hod_model.meshes.clone();
    hod_meshes.sort_by_key(|m| (m.name.clone(), m.lod));
    
    let mut dae_meshes = dae_model.meshes.clone();
    dae_meshes.sort_by_key(|m| (m.name.clone(), m.lod));
    
    for (i, (h_mesh, d_mesh)) in hod_meshes.iter().zip(dae_meshes.iter()).enumerate() {
        let h_verts: usize = h_mesh.parts.iter().map(|p| p.vertices.len()).sum();
        let h_faces: usize = h_mesh.parts.iter().map(|p| p.indices.len() / 3).sum();
        let d_verts: usize = d_mesh.parts.iter().map(|p| p.vertices.len()).sum();
        let d_faces: usize = d_mesh.parts.iter().map(|p| p.indices.len() / 3).sum();
        
        println!("Match {}: HOD name={} lod={} vs DAE name={} lod={}", i, h_mesh.name, h_mesh.lod, d_mesh.name, d_mesh.lod);
        if h_verts != d_verts || h_faces != d_faces || h_mesh.lod != d_mesh.lod {
            println!("  MISMATCH! HOD (v: {}, f: {}) vs DAE (v: {}, f: {})", h_verts, h_faces, d_verts, d_faces);
        } else {
            println!("  OK");
        }
    }
}
