use hwr_hod_parser::hod::{HODModel, generate_v2_from_model};
use std::fs;

fn main() -> std::io::Result<()> {
    let in_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod";
    let out_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/parser/pebble_0_rebuilt.hod";

    println!("Loading {}...", in_path);
    let bytes = fs::read(in_path)?;
    let mut model = HODModel::parse(&bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("Loaded model! Meshes: {}, Materials: {}", model.meshes.len(), model.materials.len());
    
    if !model.is_v2 {
        println!("HOD 1.0 detected on open. Automatically applying backward compatibility transformations...");
        hwr_hod_parser::hod::synthesize_engine_nozzles_v1(&mut model);
        hwr_hod_parser::hod::validate_marker_parents(&mut model);
        model.is_v2 = true;
        model.version = 512;
    }
    
    println!("Generating V2...");
    let out_bytes = generate_v2_from_model(&bytes, &model).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    println!("Writing to {}...", out_path);
    fs::write(out_path, &out_bytes)?;
    
    println!("Done!");
    Ok(())
}
