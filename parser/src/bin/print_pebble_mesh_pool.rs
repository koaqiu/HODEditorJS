use std::fs::File;
use std::io::{self, Read, Cursor};
use hwr_hod_parser::iff::IffChunk;

fn main() -> io::Result<()> {
    let path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/ship/hgn_interceptor/hgn_interceptor.hod";
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut cursor = Cursor::new(&bytes);
    let mut chunks = Vec::new();
    while cursor.position() < bytes.len() as u64 {
        if let Ok(c) = IffChunk::read_chunk(&mut cursor) {
            chunks.push(c);
        } else {
            break;
        }
    }

    for chunk in &chunks {
        if chunk.id == "DTRM" {
            for sub in &chunk.children {
                if sub.id == "ETSH" {
                    println!("Found ETSH size = {}", sub.data.len());
                    println!("Raw Hex: {:02x?}", &sub.data);
                    println!("As ASCII: {}", String::from_utf8_lossy(&sub.data));
                }
            }
        }
    }

    Ok(())
}
