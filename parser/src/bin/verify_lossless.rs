use std::fs;
use hwr_hod_parser::hod::{HODModel, generate_v2_from_model, save_edits};
use hwr_hod_parser::dae::parse_dae;

fn main() {
    let pebble_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod";
    let elysium_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod";
    let fenris_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_fenris/ter_fenris.hod";
    let asteroid_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/freespace_remastered/resource/asteroid/asteroid_3/asteroid_3.hod";
    let dae_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/effect/galaxymap/hodsunpacked/mapgalaxy/galaxymapgalaxy.dae";

    println!("--- Testing Pebble (HOD 2.0) ---");
    test_file(pebble_path, true);

    println!("\n--- Testing Ter Elysium (HOD 2.0) ---");
    test_file(elysium_path, true);

    println!("\n--- Testing Ter Fenris (HOD 2.0, With Animation) ---");
    test_file(fenris_path, true);

    println!("\n--- Testing Asteroid 3 (HOD 1.0) ---");
    test_file(asteroid_path, false);

    println!("\n--- Testing DAE Fallback ---");
    let dae_bytes = fs::read(dae_path).unwrap();
    let dae_str = String::from_utf8(dae_bytes).unwrap();
    let mut model = parse_dae(&dae_str).unwrap();
    model.is_v2 = true;
    let new_bytes = generate_v2_from_model(&[], &model).unwrap();
    println!("DAE generation succeeded! Output size: {} bytes", new_bytes.len());
}

fn test_file(path: &str, is_v2: bool) {
    let bytes = fs::read(path).unwrap();
    println!("Original size: {} bytes", bytes.len());

    let model = HODModel::parse(&bytes).unwrap();

    let new_bytes = if is_v2 {
        generate_v2_from_model(&bytes, &model).unwrap()
    } else {
        save_edits(&bytes, &model).unwrap()
    };
    
    let out_path = format!("{}_generated.hod", path);
    fs::write(&out_path, &new_bytes).unwrap();
    println!("Generated size: {} bytes, saved to {}", new_bytes.len(), out_path);
    
    if bytes.len() == new_bytes.len() {
        println!("SUCCESS: Sizes match exactly!");
        
        let mut diffs = 0;
        for i in 0..bytes.len() {
            if bytes[i] != new_bytes[i] {
                diffs += 1;
            }
        }
        if diffs == 0 {
            println!("SUCCESS: Files are 100% byte-for-byte identical!");
        } else {
            println!("WARNING: Sizes match, but {} bytes differ.", diffs);
        }
    } else {
        println!("FAILED: Sizes do not match! Diff: {}", new_bytes.len() as isize - bytes.len() as isize);
    }
    
    println!("Re-parsing generated file to verify structural integrity...");
    match HODModel::parse(&new_bytes) {
        Ok(reparsed) => {
            println!("SUCCESS: Re-parsed generated file!");
            println!("  Original: Meshes={}, Joints={}, NavLights={}, Markers={}, EngineBurns={}", 
                     model.meshes.len(), model.joints.len(), model.nav_lights.len(), model.markers.len(), model.engine_burns.len());
            println!("  Reparsed: Meshes={}, Joints={}, NavLights={}, Markers={}, EngineBurns={}", 
                     reparsed.meshes.len(), reparsed.joints.len(), reparsed.nav_lights.len(), reparsed.markers.len(), reparsed.engine_burns.len());
        }
        Err(e) => {
            println!("FAILED: Could not re-parse generated file! {}", e);
        }
    }
}
