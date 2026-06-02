use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::hod::HODModel;
use hwr_hod_parser::iff::IffChunk;
use std::fs;
use std::io::Cursor;

fn read_len_string(r: &mut Cursor<&[u8]>) -> String {
    let len = r.read_u32::<LittleEndian>().unwrap() as usize;
    let mut buf = vec![0u8; len];
    std::io::Read::read_exact(r, &mut buf).unwrap();
    String::from_utf8_lossy(&buf).to_string()
}

fn dump_stat_chunks(bytes: &[u8], label: &str) {
    println!("\n=== {} STAT Chunks ===", label);
    let mut cursor = Cursor::new(bytes);
    while cursor.position() < bytes.len() as u64 {
        if let Ok(chunk) = IffChunk::read_chunk(&mut cursor) {
            process_stat(&chunk, label);
        }
    }
}

fn process_stat(chunk: &IffChunk, label: &str) {
    if chunk.id == "HVMD" {
        for child in &chunk.children {
            if child.id == "STAT" || child.id == "MATT" {
                let mut r = Cursor::new(child.data.as_slice());
                let mat_name = read_len_string(&mut r);
                let shader_name = read_len_string(&mut r);
                let param_count = r.read_u32::<LittleEndian>().unwrap_or(0);
                
                println!("\n[{}] Material: {} (shader: {})", label, mat_name, shader_name);
                println!("[{}]   Param count: {}", label, param_count);
                
                if param_count > 0 {
                    let _extra1 = r.read_u32::<LittleEndian>().unwrap_or(0);
                    let _extra2 = r.read_u32::<LittleEndian>().unwrap_or(0);
                    
                    for i in 0..param_count {
                        let tex_idx = if i == 0 {
                            r.read_u32::<LittleEndian>().unwrap_or(999)
                        } else {
                            let _e3 = r.read_u32::<LittleEndian>().unwrap_or(0);
                            let _e4 = r.read_u32::<LittleEndian>().unwrap_or(0);
                            r.read_u32::<LittleEndian>().unwrap_or(999)
                        };
                        let param_name = read_len_string(&mut r);
                        println!("[{}]   Slot {}: texture_index={}, param_name={}", label, i, tex_idx, param_name);
                    }
                }
            }
        }
    }
    for child in &chunk.children {
        process_stat(child, label);
    }
}

fn main() {
    let orig_path = "../testing/ter_zephyrus/ter_zephyrus_2.0_original.hod";
    let saved_path = "../testing/ter_zephyrus/ter_zephyrus_from_2.0_to_2.0.hod";
    
    println!("Loading original file: {}", orig_path);
    let orig_bytes = fs::read(orig_path).unwrap();
    
    println!("Loading saved file: {}", saved_path);
    let saved_bytes = fs::read(saved_path).unwrap();
    
    println!("\nParsing original file...");
    let mut orig_model = HODModel::parse(&orig_bytes).unwrap();
    
    println!("\n=== Original model.textures order ===");
    for (i, tex) in orig_model.textures.iter().enumerate() {
        println!("  [{}] {}", i, tex.name);
    }
    
    println!("\n=== Original model.materials texture_maps ===");
    for mat in &orig_model.materials {
        println!("  Material: {}", mat.name);
        for (i, tex_name) in mat.texture_maps.iter().enumerate() {
            println!("    Slot {}: {}", i, tex_name);
        }
    }
    
    println!("\n=== Calling auto_assign_and_resize_textures ===");
    orig_model.auto_assign_and_resize_textures();
    
    println!("\n=== After auto_assign: model.textures order ===");
    for (i, tex) in orig_model.textures.iter().enumerate() {
        println!("  [{}] {}", i, tex.name);
    }
    
    println!("\n=== After auto_assign: model.materials texture_maps ===");
    for mat in &orig_model.materials {
        println!("  Material: {}", mat.name);
        for (i, tex_name) in mat.texture_maps.iter().enumerate() {
            println!("    Slot {}: {}", i, tex_name);
        }
    }
    
    println!("\n=== Calling generate_v2_from_model ===");
    let generated_bytes = hwr_hod_parser::hod::generate_v2_from_model(&orig_bytes, &orig_model).unwrap();
    
    dump_stat_chunks(&orig_bytes, "ORIGINAL");
    dump_stat_chunks(&generated_bytes, "GENERATED");
}
