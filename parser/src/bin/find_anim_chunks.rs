use std::fs::File;
use std::io::{self, Read, Cursor};
use hwr_hod_parser::iff::IffChunk;

fn print_chunk_tree(chunk: &IffChunk, depth: usize) {
    let indent = "  ".repeat(depth);
    let chunk_type_str = match chunk.chunk_type {
        hwr_hod_parser::iff::ChunkType::Form => "FORM",
        hwr_hod_parser::iff::ChunkType::Normal => "NRML",
        hwr_hod_parser::iff::ChunkType::Default => "DEFT",
    };
    println!(
        "{}{} (Type: {}, Version: {}, Data Size: {}, Children: {})",
        indent, chunk.id, chunk_type_str, chunk.version, chunk.data.len(), chunk.children.len()
    );

    if ["KEYF", "LTTN", "EVAL", "MAD"].contains(&chunk.id.trim()) {
        println!("{}  -> INTERESTING DATA: {:?}", indent, &chunk.data[..std::cmp::min(64, chunk.data.len())]);
    }

    for child in &chunk.children {
        print_chunk_tree(child, depth + 1);
    }
}

fn main() -> io::Result<()> {
    let path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/ship/hgn_mothership/hgn_mothership.mad";
    println!("Analyzing {} using IffChunk...", path);
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut cursor = Cursor::new(&bytes);
    while cursor.position() < bytes.len() as u64 {
        match IffChunk::read_chunk(&mut cursor) {
            Ok(chunk) => {
                print_chunk_tree(&chunk, 0);
            }
            Err(e) => {
                println!("Error reading chunk: {}", e);
                break;
            }
        }
    }

    Ok(())
}
