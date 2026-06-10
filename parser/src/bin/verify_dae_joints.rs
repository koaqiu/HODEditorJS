use hwr_hod_parser::hod::HODModel;
use hwr_hod_parser::dae::parse_dae;
use std::collections::HashSet;

fn main() {
    let hod_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_fenris/ter_fenris_2.0_original.hod";
    let dae_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/mod-tools/HODEditorJS/testing/ter_fenris/ter_fenris.DAE";
    
    let hod_bytes = std::fs::read(hod_path).unwrap();
    let hod_model = HODModel::parse_with_external(&hod_bytes, Some(hod_path), None).unwrap();
    
    let dae_str = std::fs::read_to_string(dae_path).unwrap();
    let dae_model = parse_dae(&dae_str).unwrap();
    
    let hod_joint_names: HashSet<_> = hod_model.joints.iter().map(|j| j.name.clone()).collect();
    let dae_joint_names: HashSet<_> = dae_model.joints.iter().map(|j| j.name.clone()).collect();
    
    println!("Joints in HOD but not in DAE:");
    for name in &hod_joint_names {
        if !dae_joint_names.contains(name) {
            println!("  - {}", name);
        }
    }
    
    println!("Joints in DAE but not in HOD:");
    for name in &dae_joint_names {
        if !hod_joint_names.contains(name) {
            println!("  - {}", name);
        }
    }
}
