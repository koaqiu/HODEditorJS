use hwr_hod_parser::iff::IffChunk;
use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

fn extract_mult(chunk: &IffChunk) {
    if chunk.id.trim() == "MULT" {
        println!("Found MULT chunk, size: {}", chunk.data.len());
        let mut reader = Cursor::new(&chunk.data);
        let mut len_bytes = [0u8; 4];
        if reader.read_exact(&mut len_bytes).is_ok() {
            let len = u32::from_le_bytes(len_bytes) as usize;
            let mut name_bytes = vec![0u8; len];
            let _ = reader.read_exact(&mut name_bytes);
            println!("  Name: {}", String::from_utf8_lossy(&name_bytes));
            
            if reader.read_exact(&mut len_bytes).is_ok() {
                let parent_len = u32::from_le_bytes(len_bytes) as usize;
                let mut parent_bytes = vec![0u8; parent_len];
                let _ = reader.read_exact(&mut parent_bytes);
                println!("  Parent: {}", String::from_utf8_lossy(&parent_bytes));
                
                let lod_count = reader.read_u32::<LittleEndian>().unwrap_or(0);
                println!("  LOD Count: {}", lod_count);
                
                // Now read children chunks!
                while reader.position() < reader.get_ref().len() as u64 {
                    if let Ok(c) = IffChunk::read_chunk(&mut reader) {
                        println!("    Child: {} (Size: {}, Version: {})", c.id, c.data.len(), c.version);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    for child in &chunk.children {
        extract_mult(child);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "asteroid_3.hod" };
    let mut f = File::open(path)?;
    let root = IffChunk::read_chunk(&mut f).unwrap();
    extract_mult(&root);
    Ok(())
}
