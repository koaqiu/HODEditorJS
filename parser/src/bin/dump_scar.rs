use hwr_hod_parser::hod;
use hwr_hod_parser::iff;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Write, self};
use hwr_hod_parser::xpress;
use std::env;
use std::fs::File;

fn extract_scar_from_chunk(chunk: &iff::IffChunk, file_idx: &mut usize) -> io::Result<()> {
    if chunk.id == "SCAR" {
        let filename = format!("scar_dump_{}.bin", file_idx);
        let mut out = File::create(&filename)?;
        out.write_all(&chunk.data)?;
        println!("Extracted {} bytes to {}", chunk.data.len(), filename);
        *file_idx += 1;
    }
    for child in &chunk.children {
        extract_scar_from_chunk(child, file_idx)?;
    }
    Ok(())
}

fn extract_pool_from_chunk(chunk: &iff::IffChunk) -> io::Result<()> {
    if chunk.id == "POOL" {
        let filename = "pool_dump.bin";
        let mut out = File::create(filename)?;
        out.write_all(&chunk.data)?;
        println!("Extracted {} bytes to {}", chunk.data.len(), filename);
        
        let mut p_cursor = Cursor::new(&chunk.data);
        let pool_type = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        
        let comp_tex_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let decomp_tex_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let mut comp_tex = vec![0u8; comp_tex_len as usize];
        let _ = p_cursor.read_exact(&mut comp_tex);
        let decomp_tex = xpress::decompress(&comp_tex, decomp_tex_len as usize).unwrap_or_default();
        
        let comp_mesh_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let decomp_mesh_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let mut comp_mesh = vec![0u8; comp_mesh_len as usize];
        let _ = p_cursor.read_exact(&mut comp_mesh);
        let decomp_mesh = xpress::decompress(&comp_mesh, decomp_mesh_len as usize).unwrap_or_default();
        
        let comp_face_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let decomp_face_len = p_cursor.read_u32::<LittleEndian>().unwrap_or(0);
        let mut comp_face = vec![0u8; comp_face_len as usize];
        let _ = p_cursor.read_exact(&mut comp_face);
        let decomp_face = xpress::decompress(&comp_face, decomp_face_len as usize).unwrap_or_default();
        
        std::fs::write("decompressed_mesh.bin", &decomp_mesh)?;
        std::fs::write("decompressed_face.bin", &decomp_face)?;
        println!("Saved decompressed_mesh.bin ({} bytes) and decompressed_face.bin ({} bytes).", decomp_mesh.len(), decomp_face.len());
    }
    for child in &chunk.children {
        extract_pool_from_chunk(child)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let file_path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: dump_scar <file_path>");
        std::process::exit(1);
    });
    
    println!("Reading {}...", file_path);
    let mut file = File::open(file_path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

    let mut cursor = std::io::Cursor::new(&bytes);
    let mut chunks = Vec::new();
    while cursor.position() < bytes.len() as u64 {
        if let Ok(chunk) = iff::IffChunk::read_chunk(&mut cursor) {
            chunks.push(chunk);
        } else {
            break;
        }
    }
    
    let mut scar_idx = 0;
    for chunk in &chunks {
        extract_scar_from_chunk(chunk, &mut scar_idx)?;
        extract_pool_from_chunk(chunk)?;
    }
    
    Ok(())
}
