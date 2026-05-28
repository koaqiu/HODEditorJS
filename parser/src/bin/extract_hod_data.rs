use hwr_hod_parser::hod::HODModel;
use serde_json;
use std::fs;
use std::path::Path;

fn main() -> Result<(), String> {
    let test_cases = vec![
        ("ter_pharos", "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_pharos/ter_pharos_hodor.hod"),
        ("ter_centaur", "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_centaur/ter_centaur_hodor.hod"),
    ];

    for (name, hod_path) in &test_cases {
        println!("--- Extracting data from {} ---", name);

        let bytes = fs::read(hod_path).map_err(|e| format!("Failed to read HOD: {}", e))?;
        let model = HODModel::parse(&bytes).map_err(|e| format!("Failed to parse HOD: {}", e))?;

        // Create output directory
        let output_dir = Path::new(hod_path).parent().unwrap();

        // Save joints
        let joints_path = output_dir.join("joints.json");
        let joints_json = serde_json::to_string_pretty(&model.joints).map_err(|e| e.to_string())?;
        fs::write(&joints_path, joints_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} joints to {}",
            model.joints.len(),
            joints_path.display()
        );

        // Save navlights
        let navlights_path = output_dir.join("navlights.json");
        let navlights_json =
            serde_json::to_string_pretty(&model.nav_lights).map_err(|e| e.to_string())?;
        fs::write(&navlights_path, navlights_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} navlights to {}",
            model.nav_lights.len(),
            navlights_path.display()
        );

        // Save engine burns
        let engine_burns_path = output_dir.join("engine_burns.json");
        let engine_burns_json =
            serde_json::to_string_pretty(&model.engine_burns).map_err(|e| e.to_string())?;
        fs::write(&engine_burns_path, engine_burns_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} engine burns to {}",
            model.engine_burns.len(),
            engine_burns_path.display()
        );

        // Save markers
        let markers_path = output_dir.join("markers.json");
        let markers_json =
            serde_json::to_string_pretty(&model.markers).map_err(|e| e.to_string())?;
        fs::write(&markers_path, markers_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} markers to {}",
            model.markers.len(),
            markers_path.display()
        );

        // Save collision meshes
        let collision_path = output_dir.join("collision_meshes.json");
        let collision_json =
            serde_json::to_string_pretty(&model.collision_meshes).map_err(|e| e.to_string())?;
        fs::write(&collision_path, collision_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} collision meshes to {}",
            model.collision_meshes.len(),
            collision_path.display()
        );

        // Save materials
        let materials_path = output_dir.join("materials.json");
        let materials_json =
            serde_json::to_string_pretty(&model.materials).map_err(|e| e.to_string())?;
        fs::write(&materials_path, materials_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} materials to {}",
            model.materials.len(),
            materials_path.display()
        );

        // Save textures
        let textures_path = output_dir.join("textures.json");
        let textures_json =
            serde_json::to_string_pretty(&model.textures).map_err(|e| e.to_string())?;
        fs::write(&textures_path, textures_json).map_err(|e| e.to_string())?;
        println!(
            "  Saved {} textures to {}",
            model.textures.len(),
            textures_path.display()
        );

        // Save full model
        let model_path = output_dir.join("model.json");
        let model_json = serde_json::to_string_pretty(&model).map_err(|e| e.to_string())?;
        fs::write(&model_path, model_json).map_err(|e| e.to_string())?;
        println!("  Saved full model to {}", model_path.display());

        println!();
    }

    Ok(())
}
