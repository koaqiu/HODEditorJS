use std::fs::File;
use std::io::{self, Read, Cursor};
use hwr_hod_parser::iff::IffChunk;

fn main() -> io::Result<()> {
    let path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/ship/hgn_mothership/hgn_mothership.hod";
    println!("Reading {}...", path);
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut cursor = Cursor::new(&bytes);
    let mut chunks = Vec::new();
    while cursor.position() < bytes.len() as u64 {
        if let Ok(chunk) = IffChunk::read_chunk(&mut cursor) {
            chunks.push(chunk);
        } else {
            break;
        }
    }

    for chunk in &chunks {
        if chunk.id == "HVMD" {
            let stat_chunks: Vec<&IffChunk> = chunk.children.iter().filter(|c| c.id == "STAT").collect();
            println!("Found {} STAT chunks.", stat_chunks.len());
            for (idx, stat) in stat_chunks.iter().enumerate().take(3) {
                println!("\n--- STAT Chunk #{} (Size: {}, Version: {}) ---", idx, stat.data.len(), stat.version);
                // Dump hex
                let limit = stat.data.len();
                for chunk_slice in stat.data[..limit].chunks(16) {
                    let hex_strs: Vec<String> = chunk_slice.iter().map(|b| format!("{:02X}", b)).collect();
                    let ascii_strs: String = chunk_slice.iter().map(|&b| {
                        if b >= 32 && b <= 126 { b as char } else { '.' }
                    }).collect();
                    println!("  {:48} | {}", hex_strs.join(" "), ascii_strs);
                }
            }
        }
    }

    Ok(())
}
