use hwr_hod_parser::iff::IffChunk;
use std::fs::File;
use std::io::{Cursor, Read, Seek};

fn extract_chunks(chunk: &IffChunk, indent: usize) {
    let ind = " ".repeat(indent);
    println!("{}Chunk: {} (Size: {}, Version: {})", ind, chunk.id, chunk.data.len(), chunk.version);
    if chunk.id.trim() == "DOCK" {
        let mut r = Cursor::new(&chunk.data);
        if let Ok(first) = byteorder::ReadBytesExt::read_u32::<byteorder::LittleEndian>(&mut r) {
            println!("{}  DOCK first u32 (LE): {}", ind, first);
        }
        if let Ok(second) = byteorder::ReadBytesExt::read_u32::<byteorder::LittleEndian>(&mut r) {
            println!("{}  DOCK second u32 (LE): {}", ind, second);
        }
    }
    for child in &chunk.children {
        extract_chunks(child, indent + 2);
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "asteroid_3.hod" };
    let mut f = File::open(path)?;
    while let Ok(root) = IffChunk::read_chunk(&mut f) {
        extract_chunks(&root, 0);
    }
    Ok(())
}
