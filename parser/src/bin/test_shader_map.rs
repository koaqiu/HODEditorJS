use hwr_hod_parser::shader_map::ShadersMap;
use std::path::Path;

fn main() {
    let shader_map_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/HODOR/SHADERS.MAP";

    println!("Testing SHADERS.MAP parser...");
    println!("Path: {}", shader_map_path);

    match ShadersMap::from_file(Path::new(shader_map_path)) {
        Ok(shaders_map) => {
            println!("\n✅ Successfully parsed SHADERS.MAP");
            println!("Found {} pipeline mappings", shaders_map.mappings.len());

            // Print all pipeline names
            println!("\nPipeline names:");
            let mut names: Vec<&String> = shaders_map.mappings.keys().collect();
            names.sort();
            for name in names {
                println!("  - {}", name);
            }

            // Test ship shader
            if let Some(ship) = shaders_map.get_mapping("ship") {
                println!("\n=== Ship Shader ===");
                println!("Parameters: {}", ship.parameters.len());
                for param in &ship.parameters {
                    println!("  ${} [{}]", param.name, param.format);
                    for channel in &param.channel_mappings {
                        println!("    {} = {}", channel.texture_role, channel.channels);
                    }
                }
            }

            // Test thruster shader
            if let Some(thruster) = shaders_map.get_mapping("thruster") {
                println!("\n=== Thruster Shader ===");
                println!("Parameters: {}", thruster.parameters.len());
                for param in &thruster.parameters {
                    println!("  ${} [{}]", param.name, param.format);
                    for channel in &param.channel_mappings {
                        println!("    {} = {}", channel.texture_role, channel.channels);
                    }
                }
            }

            // Test texture detection
            println!("\n=== Texture Detection Tests ===");

            let test_cases = vec![
                vec!["Pebble_DIFF.TGA".to_string()],
                vec!["Pebble_DIFF.TGA".to_string(), "Pebble_GLOW.TGA".to_string()],
                vec![
                    "Pebble_DIFF.TGA".to_string(),
                    "Pebble_GLOW.TGA".to_string(),
                    "Pebble_TEAM.TGA".to_string(),
                ],
                vec![
                    "Pebble_DIFF.TGA".to_string(),
                    "Pebble_GLOW.TGA".to_string(),
                    "Pebble_TEAM.TGA".to_string(),
                    "Pebble_NORM.TGA".to_string(),
                ],
            ];

            for (i, textures) in test_cases.iter().enumerate() {
                let shader_type = ShadersMap::detect_shader_type(textures);
                println!("Test {}: {:?} -> {}", i + 1, textures, shader_type);
            }
        }
        Err(e) => {
            println!("\n❌ Failed to parse SHADERS.MAP: {}", e);
        }
    }
}
