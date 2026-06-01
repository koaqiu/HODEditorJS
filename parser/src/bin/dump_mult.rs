use hwr_hod_parser::hod::HODModel;
use std::fs;

fn main() {
    let bytes1 = fs::read("../testing/ter_zephyrus/ter_zephyrus_1.0_original.hod").unwrap();
    let model1 = HODModel::parse(&bytes1).unwrap();
    println!("HOD 1.0 MESHES:");
    for mesh in &model1.meshes {
        println!("  Mesh: {} (parent: {})", mesh.name, mesh.parent_name);
    }
}
