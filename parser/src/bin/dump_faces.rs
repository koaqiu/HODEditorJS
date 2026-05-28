use hwr_hod_parser::hod::HODModel;
use std::fs;

fn main() {
    let v1_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/freespace_remastered/ship/ter_orion/ter_orion.hod";
    let v2_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_orion/ter_orion.hod";

    println!("=== v1 HOD radar01 Vertices ===");
    let data1 = fs::read(v1_path).expect("Could not read v1 HOD file");
    if let Ok(model1) = HODModel::parse(&data1) {
        if let Some(m) = model1.meshes.iter().find(|mesh| mesh.name == "radar01") {
            if let Some(part) = m.parts.first() {
                for (i, v) in part.vertices.iter().take(10).enumerate() {
                    println!(
                        "  [{}] Pos: [{:.3}, {:.3}, {:.3}]",
                        i, v.position.x, v.position.y, v.position.z
                    );
                }
            }
        }
    }

    println!("\n=== v2 HOD radar01 Vertices ===");
    let data2 = fs::read(v2_path).expect("Could not read v2 HOD file");
    if let Ok(model2) = HODModel::parse(&data2) {
        if let Some(m) = model2.meshes.iter().find(|mesh| mesh.name == "radar01") {
            if let Some(part) = m.parts.first() {
                for (i, v) in part.vertices.iter().take(10).enumerate() {
                    println!(
                        "  [{}] Pos: [{:.3}, {:.3}, {:.3}]",
                        i, v.position.x, v.position.y, v.position.z
                    );
                }
            }
        }
    }
}
