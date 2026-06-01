use hwr_hod_parser::hod::{HODModel, save_edits};
use std::fs;

fn main() {
    let hod_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_zephyrus/ter_zephyrus_2.0_original.hod";
    
    println!("=== Loading ter_zephyrus_2.0_original.hod ===\n");
    
    let hod_bytes = fs::read(hod_path).expect("Failed to read HOD file");
    let model = HODModel::parse(&hod_bytes).expect("Failed to parse HOD");
    
    println!("\n=== Parse Complete ===");
    println!("Textures: {}", model.textures.len());
    for (i, tex) in model.textures.iter().enumerate() {
        println!("  Texture {}: {} ({}x{}, {}, preview={}, data={})", 
                i, tex.name, tex.width, tex.height, tex.format,
                tex.png_preview.is_some(), tex.png_data.is_some());
    }
    
    println!("\n=== Saving as HOD 2.0 ===\n");
    let output_bytes = save_edits(&hod_bytes, &model).expect("Failed to save");
    
    println!("\n=== Save Complete ===");
    println!("Output size: {} bytes", output_bytes.len());
    
    let output_path = "/tmp/ter_zephyrus_debug_output.hod";
    fs::write(output_path, &output_bytes).expect("Failed to write output");
    println!("Written to: {}", output_path);
    
    println!("\n=== Re-parsing output ===\n");
    let reparsed = HODModel::parse(&output_bytes).expect("Failed to reparse");
    println!("Re-parsed textures: {}", reparsed.textures.len());
    for (i, tex) in reparsed.textures.iter().enumerate() {
        println!("  Texture {}: {} ({}x{}, {}, preview={}, data={})", 
                i, tex.name, tex.width, tex.height, tex.format,
                tex.png_preview.is_some(), tex.png_data.is_some());
    }
}
