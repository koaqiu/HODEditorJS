use hwr_hod_parser::dae::parse_dae;
use std::fs;

fn main() {
    let dae_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/current_project_processing/ship_converted/shi_aeshma/shi_aeshma.DAE";
    
    let xml_str = match fs::read_to_string(dae_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read DAE: {}", e);
            return;
        }
    };
    
    match parse_dae(&xml_str) {
        Ok(model) => {
            println!("Successfully parsed DAE into HODModel!");
            println!("Meshes: {}", model.meshes.len());
            for mesh in &model.meshes {
                println!("  - {} ({} parts)", mesh.name, mesh.parts.len());
                for part in &mesh.parts {
                    println!("      Material Index: {}, Vertices: {}", part.material_index, part.vertices.len());
                }
            }
            
            println!("Joints: {}", model.joints.len());
            println!("Markers: {}", model.markers.len());
            println!("NavLights: {}", model.nav_lights.len());
            println!("EngineBurns: {}", model.engine_burns.len());
        }
        Err(e) => {
            eprintln!("Failed to parse DAE: {}", e);
        }
    }
}
