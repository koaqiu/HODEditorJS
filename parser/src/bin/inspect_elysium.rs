use hwr_hod_parser::hod::HODModel;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod");

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to read file: {:?}", e);
            return;
        }
    };

    let mut model = match HODModel::parse(&bytes) {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to parse: {:?}", e);
            return;
        }
    };

    println!("--- BEFORE CLEAN_HIERARCHY ---");
    for burn in &model.engine_burns {
        println!("Burn: {}, Parent: {}", burn.name, burn.parent_name);
    }

    model.clean_hierarchy();

    println!("--- AFTER CLEAN_HIERARCHY ---");
    for burn in &model.engine_burns {
        println!("Burn: {}, Parent: {}", burn.name, burn.parent_name);
    }
}
