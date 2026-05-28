use hwr_hod_parser::hod::HODModel;
use std::fs;

fn main() {
    let path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/freespace_remastered/ship/ter_elysium/ter_elysium.hod";
    println!("Testing load of: {}", path);

    let bytes = fs::read(path).expect("Failed to read HOD file");
    let mut model = HODModel::parse(&bytes).expect("Failed to parse HOD");

    println!(
        "Loaded HODModel. Joint count: {}, Mesh count: {}",
        model.joints.len(),
        model.meshes.len()
    );

    // Check for any remaining duplicates
    let mut names = std::collections::HashSet::new();
    let mut has_dupes = false;
    for j in &model.joints {
        if !names.insert(j.name.clone()) {
            println!("ERROR: Found remaining duplicate joint name: {}", j.name);
            has_dupes = true;
        }
    }
    if !has_dupes {
        println!("SUCCESS: No duplicate joint names found!");
    }

    // Check hierarchy for nested specials
    let special_prefixes = ["NAVL[", "BURN[", "MARK[", "MULT[", "COL[", "SHAP[", "GLOW["];
    let mut has_invalid_nesting = false;
    for j in &model.joints {
        if let Some(parent) = &j.parent_name {
            let is_special = special_prefixes.iter().any(|&p| parent.starts_with(p));
            let is_allowed_flame = parent.starts_with("BURN[") && j.name.starts_with("Flame[");
            if is_special && !is_allowed_flame {
                println!(
                    "ERROR: Found invalid nesting! Joint {} is child of {}",
                    j.name, parent
                );
                has_invalid_nesting = true;
            }
        }
    }
    if !has_invalid_nesting {
        println!("SUCCESS: No invalid nested special nodes found!");
    }
}
