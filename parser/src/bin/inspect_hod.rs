use std::fs::File;
use std::io::Read;
use hwr_hod_parser::hod::HODModel;

fn main() {
    let path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/subsystem/tai_frigatecannon/tai_frigatecannon.hod";
    let mut file = File::open(path).expect("Failed to open HOD");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read");
    let hod = HODModel::parse(&buffer).expect("Failed to parse");
    
    println!("\nJoints:");
    for joint in &hod.joints {
        println!(" - Joint: {}, Parent: {:?}", joint.name, joint.parent_name);
    }
}
