use std::fs::File;
use std::io::Read;
use hwr_hod_parser::hod::Hod;

fn main() {
    let path = "../../testing/ter_fenris/ter_fenris_1.3_original.hod";
    let mut file = File::open(path).expect("Failed to open HOD");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read");
    
    let hod = Hod::deserialize(&buffer).expect("Failed to parse");
    
    println!("Textures:");
    for tex in hod.textures {
        println!(" - {}", tex.name);
    }
    
    println!("\nMaterials:");
    for mat in hod.materials {
        println!(" - {}: shader={}, maps={:?}", mat.name, mat.shader, mat.texture_maps);
    }
}
