use std::fs::File;
use std::io::{self, Read};
use hwr_hod_parser::hod::HODModel;

fn main() -> io::Result<()> {
    let v2_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_fenris/ter_fenris.hod";
    println!("=== Testing v2 ===");
    let mut file2 = File::open(v2_path)?;
    let mut bytes2 = Vec::new();
    file2.read_to_end(&mut bytes2)?;

    match HODModel::parse_with_external(&bytes2, Some(v2_path), None) {
        Ok(model) => {
            println!("Successfully parsed HOD 2.0. Meshes count: {}", model.meshes.len());
            for m in &model.meshes {
                println!("  Mesh Name: '{}', Parent Joint: '{}'", m.name, m.parent_name);
            }
            println!("Successfully parsed HOD 2.0. Meshes count: {}", model.meshes.len());
            for m in &model.meshes {
                println!("  Mesh Name: '{}', Parent Joint: '{}'", m.name, m.parent_name);
            }
            println!("Parsed companion animations: {}", model.animations.len());
            for anim in &model.animations {
                println!("  Animation: '{}' (duration={:.3}s, tracks={})", anim.name, anim.duration, anim.tracks.len());
                for track in &anim.tracks {
                    println!("    Track Joint: '{}', Keyframes={}", track.joint_name, track.keyframes.len());
                    for (k_idx, kf) in track.keyframes.iter().enumerate() {
                        println!("      Keyframe {}: time={:.3}s", k_idx, kf.time);
                        if let Some(pos) = &kf.position {
                            println!("        position: [{:.3}, {:.3}, {:.3}]", pos.x, pos.y, pos.z);
                        }
                        if let Some(rot) = &kf.rotation {
                            println!("        rotation: [{:.4}, {:.4}, {:.4}, {:.4}]", rot.x, rot.y, rot.z, rot.w);
                        }
                        if let Some(euler) = &kf.rotation_euler {
                            println!("        rotation_euler: [{:.4}, {:.4}, {:.4}]", euler.x, euler.y, euler.z);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Error parsing HOD: {}", e);
        }
    }

    Ok(())
}
