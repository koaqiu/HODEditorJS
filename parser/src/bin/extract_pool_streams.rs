/// Extracts decompressed POOL streams from a HOD file into raw binary files.
/// Used for Path 2 validation — feeding data to RtlCompressBuffer via Wine.
///
/// Usage: extract_pool_streams <hod_file> <output_dir>
///
/// Creates:
///   <output_dir>/decomp_mesh.bin
///   <output_dir>/decomp_face.bin
///   <output_dir>/decomp_tex.bin

use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::iff::IffChunk;
use std::io::{Cursor, Seek, SeekFrom};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: extract_pool_streams <hod_file> <output_dir>");
        std::process::exit(1);
    }

    let bytes = std::fs::read(&args[1])?;
    let output_dir = &args[2];

    let mut cursor = Cursor::new(&bytes);
    while cursor.position() < bytes.len() as u64 {
        let chunk = IffChunk::read_chunk(&mut cursor)?;
        if chunk.id == "POOL" {
            let mut pc = Cursor::new(&chunk.data);
            let _pool_type = pc.read_u32::<LittleEndian>()?;

            // Texture pool
            let comp_tex_len = pc.read_u32::<LittleEndian>()? as usize;
            let decomp_tex_len = pc.read_u32::<LittleEndian>()? as usize;
            let tex_start = pc.position() as usize;
            let comp_tex = &chunk.data[tex_start..tex_start + comp_tex_len];
            pc.seek(SeekFrom::Start((tex_start + comp_tex_len) as u64))?;

            // Mesh pool
            let comp_mesh_len = pc.read_u32::<LittleEndian>()? as usize;
            let decomp_mesh_len = pc.read_u32::<LittleEndian>()? as usize;
            let mesh_start = pc.position() as usize;
            let comp_mesh = &chunk.data[mesh_start..mesh_start + comp_mesh_len];
            pc.seek(SeekFrom::Start((mesh_start + comp_mesh_len) as u64))?;

            // Face pool
            let comp_face_len = pc.read_u32::<LittleEndian>()? as usize;
            let decomp_face_len = pc.read_u32::<LittleEndian>()? as usize;
            let face_start = pc.position() as usize;
            let comp_face = &chunk.data[face_start..face_start + comp_face_len];

            // Decompress each pool
            let decomp_tex = if comp_tex_len == decomp_tex_len {
                comp_tex.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_tex, decomp_tex_len)?
            };

            let decomp_mesh = if comp_mesh_len == decomp_mesh_len {
                comp_mesh.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_mesh, decomp_mesh_len)?
            };

            let decomp_face = if comp_face_len == decomp_face_len {
                comp_face.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_face, decomp_face_len)?
            };

            // Write output files
            std::fs::create_dir_all(output_dir)?;
            std::fs::write(format!("{}/decomp_tex.bin", output_dir), &decomp_tex)?;
            std::fs::write(format!("{}/decomp_mesh.bin", output_dir), &decomp_mesh)?;
            std::fs::write(format!("{}/decomp_face.bin", output_dir), &decomp_face)?;

            // Also write the HODOR compressed bytes for comparison
            std::fs::write(format!("{}/hodor_comp_tex.bin", output_dir), comp_tex)?;
            std::fs::write(format!("{}/hodor_comp_mesh.bin", output_dir), comp_mesh)?;
            std::fs::write(format!("{}/hodor_comp_face.bin", output_dir), comp_face)?;

            println!("Extracted from: {}", args[1]);
            println!("Output dir: {}", output_dir);
            println!();
            println!("Decompressed pools:");
            println!("  Texture: {} bytes (comp: {})", decomp_tex.len(), comp_tex_len);
            println!("  Mesh:    {} bytes (comp: {})", decomp_mesh.len(), comp_mesh_len);
            println!("  Face:    {} bytes (comp: {})", decomp_face.len(), comp_face_len);
            println!();
            println!("Files created:");
            println!("  {}/decomp_tex.bin", output_dir);
            println!("  {}/decomp_mesh.bin", output_dir);
            println!("  {}/decomp_face.bin", output_dir);
            println!("  {}/hodor_comp_tex.bin", output_dir);
            println!("  {}/hodor_comp_mesh.bin", output_dir);
            println!("  {}/hodor_comp_face.bin", output_dir);

            return Ok(());
        }
    }

    eprintln!("No POOL chunk found in {}", args[1]);
    Ok(())
}
