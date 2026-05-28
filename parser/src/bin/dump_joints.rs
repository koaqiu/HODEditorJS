use hwr_hod_parser::hod::HODModel;
use std::fs;

fn main() {
    let bytes = fs::read("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod_generated.hod").unwrap();
    let model = HODModel::parse(&bytes).unwrap();

    for (i, joint) in model.joints.iter().enumerate() {
        if joint.name.contains("EngineNozzle") {
            println!(
                "Joint {}: {} rot={:?} scale={:?}",
                i, joint.name, joint.rotation, joint.scale
            );
        }
    }
}
