use hwr_hod_parser::hod::{generate_v2_from_model, HODModel};
use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let in_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod";
    let out_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium_rebuilt.hod";

    println!("Loading {}...", in_path);
    let bytes = fs::read(in_path)?;
    let mut model =
        HODModel::parse(&bytes).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!(
        "Loaded model! Meshes: {}, Materials: {}",
        model.meshes.len(),
        model.materials.len()
    );

    // Auto-transform legacy HOD 1.0 models on open (just like load_hod does in lib.rs)
    if !model.is_v2 {
        println!("HOD 1.0 detected on open. Automatically applying backward compatibility transformations...");
        hwr_hod_parser::hod::synthesize_engine_nozzles_v1(&mut model);
        hwr_hod_parser::hod::validate_marker_parents(&mut model);
        model.is_v2 = true;
        model.version = 512;
    }

    println!("Generating V2...");
    let out_bytes = generate_v2_from_model(&bytes, &model)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    println!("Writing to {}...", out_path);
    fs::write(out_path, &out_bytes)?;

    println!("Done!");
    Ok(())
}
