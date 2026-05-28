use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::xpress;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: pool_diff <hodor.hod> <generated.hod>");
        std::process::exit(1);
    }

    let hodor_bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let gen_bytes = fs::read(&args[2]).map_err(|e| e.to_string())?;

    let hodor_pool = extract_pool(&hodor_bytes)?;
    let gen_pool = extract_pool(&gen_bytes)?;

    println!("=== POOL Comparison ===");
    println!(
        "HODOR:  tex={}/{} mesh={}/{} face={}/{}",
        hodor_pool.comp_tex_len,
        hodor_pool.decomp_tex_len,
        hodor_pool.comp_mesh_len,
        hodor_pool.decomp_mesh_len,
        hodor_pool.comp_face_len,
        hodor_pool.decomp_face_len
    );
    println!(
        "Gen:    tex={}/{} mesh={}/{} face={}/{}",
        gen_pool.comp_tex_len,
        gen_pool.decomp_tex_len,
        gen_pool.comp_mesh_len,
        gen_pool.decomp_mesh_len,
        gen_pool.comp_face_len,
        gen_pool.decomp_face_len
    );

    // Compare decompressed mesh pool
    println!("\n--- Mesh Pool ---");
    if hodor_pool.decomp_mesh != gen_pool.decomp_mesh {
        let first_diff = hodor_pool
            .decomp_mesh
            .iter()
            .zip(gen_pool.decomp_mesh.iter())
            .position(|(a, b)| a != b);
        let diff_count = hodor_pool
            .decomp_mesh
            .iter()
            .zip(gen_pool.decomp_mesh.iter())
            .filter(|(a, b)| a != b)
            .count();
        println!(
            "  MISMATCH: {} bytes differ out of {}",
            diff_count,
            hodor_pool.decomp_mesh.len().max(gen_pool.decomp_mesh.len())
        );
        if let Some(pos) = first_diff {
            println!("  First difference at offset {}", pos);
            let start = pos.saturating_sub(8);
            let end = (pos + 16).min(hodor_pool.decomp_mesh.len().min(gen_pool.decomp_mesh.len()));
            println!("  HODOR bytes[{}..{}]: {:02x?}", start, end, &hodor_pool.decomp_mesh[start..end]);
            println!("  Gen   bytes[{}..{}]: {:02x?}", start, end, &gen_pool.decomp_mesh[start..end]);
            // Show 4-byte groups as floats for context
            if pos + 4 <= hodor_pool.decomp_mesh.len() && pos + 4 <= gen_pool.decomp_mesh.len() {
                let h_val = f32::from_le_bytes([
                    hodor_pool.decomp_mesh[pos],
                    hodor_pool.decomp_mesh[pos + 1],
                    hodor_pool.decomp_mesh[pos + 2],
                    hodor_pool.decomp_mesh[pos + 3],
                ]);
                let g_val = f32::from_le_bytes([
                    gen_pool.decomp_mesh[pos],
                    gen_pool.decomp_mesh[pos + 1],
                    gen_pool.decomp_mesh[pos + 2],
                    gen_pool.decomp_mesh[pos + 3],
                ]);
                println!("  HODOR as f32: {}", h_val);
                println!("  Gen   as f32: {}", g_val);
            }
        }
    } else {
        println!("  MATCH (identical)");
    }

    // Compare decompressed face pool
    println!("\n--- Face Pool ---");
    if hodor_pool.decomp_face != gen_pool.decomp_face {
        let first_diff = hodor_pool
            .decomp_face
            .iter()
            .zip(gen_pool.decomp_face.iter())
            .position(|(a, b)| a != b);
        let diff_count = hodor_pool
            .decomp_face
            .iter()
            .zip(gen_pool.decomp_face.iter())
            .filter(|(a, b)| a != b)
            .count();
        println!(
            "  MISMATCH: {} bytes differ out of {}",
            diff_count,
            hodor_pool.decomp_face.len().max(gen_pool.decomp_face.len())
        );
        if let Some(pos) = first_diff {
            println!("  First difference at offset {}", pos);
            let start = pos.saturating_sub(4);
            let end = (pos + 16).min(hodor_pool.decomp_face.len().min(gen_pool.decomp_face.len()));
            println!("  HODOR bytes[{}..{}]: {:02x?}", start, end, &hodor_pool.decomp_face[start..end]);
            println!("  Gen   bytes[{}..{}]: {:02x?}", start, end, &gen_pool.decomp_face[start..end]);
            if pos + 2 <= hodor_pool.decomp_face.len() && pos + 2 <= gen_pool.decomp_face.len() {
                let h_val = u16::from_le_bytes([hodor_pool.decomp_face[pos], hodor_pool.decomp_face[pos + 1]]);
                let g_val = u16::from_le_bytes([gen_pool.decomp_face[pos], gen_pool.decomp_face[pos + 1]]);
                println!("  HODOR as u16: {}", h_val);
                println!("  Gen   as u16: {}", g_val);
            }
        }
    } else {
        println!("  MATCH (identical)");
    }

    // Compare decompressed texture pool
    println!("\n--- Texture Pool ---");
    if hodor_pool.decomp_tex != gen_pool.decomp_tex {
        let first_diff = hodor_pool
            .decomp_tex
            .iter()
            .zip(gen_pool.decomp_tex.iter())
            .position(|(a, b)| a != b);
        let diff_count = hodor_pool
            .decomp_tex
            .iter()
            .zip(gen_pool.decomp_tex.iter())
            .filter(|(a, b)| a != b)
            .count();
        println!(
            "  MISMATCH: {} bytes differ out of {}",
            diff_count,
            hodor_pool.decomp_tex.len().max(gen_pool.decomp_tex.len())
        );
        if let Some(pos) = first_diff {
            println!("  First difference at offset {}", pos);
            let start = pos.saturating_sub(4);
            let end = (pos + 16).min(hodor_pool.decomp_tex.len().min(gen_pool.decomp_tex.len()));
            println!("  HODOR bytes[{}..{}]: {:02x?}", start, end, &hodor_pool.decomp_tex[start..end]);
            println!("  Gen   bytes[{}..{}]: {:02x?}", start, end, &gen_pool.decomp_tex[start..end]);
        }
    } else {
        println!("  MATCH (identical)");
    }

    Ok(())
}

struct PoolData {
    comp_tex_len: usize,
    decomp_tex_len: usize,
    comp_mesh_len: usize,
    decomp_mesh_len: usize,
    comp_face_len: usize,
    decomp_face_len: usize,
    decomp_tex: Vec<u8>,
    decomp_mesh: Vec<u8>,
    decomp_face: Vec<u8>,
}

fn extract_pool(bytes: &[u8]) -> Result<PoolData, String> {
    let mut cursor = Cursor::new(bytes);
    let mut pool_offset = None;

    // Find POOL chunk
    while cursor.position() < bytes.len() as u64 {
        let start = cursor.position();
        let mut id_bytes = [0u8; 4];
        if cursor.read_exact(&mut id_bytes).is_err() {
            break;
        }
        let id = String::from_utf8_lossy(&id_bytes).to_string();
        let mut size_bytes = [0u8; 4];
        cursor.read_exact(&mut size_bytes).map_err(|e| e.to_string())?;
        let size = u32::from_be_bytes(size_bytes);

        if id == "FORM" {
            let mut real_id_bytes = [0u8; 4];
            cursor.read_exact(&mut real_id_bytes).map_err(|e| e.to_string())?;
            let payload_size = (size - 4) as usize;
            let _ = cursor.read_exact(&mut vec![0u8; payload_size]);
        } else if id == "NRML" {
            let mut _real_id_bytes = [0u8; 4];
            cursor.read_exact(&mut _real_id_bytes).map_err(|e| e.to_string())?;
            let mut _version_bytes = [0u8; 4];
            cursor.read_exact(&mut _version_bytes).map_err(|e| e.to_string())?;
            let payload_size = (size - 8) as usize;
            let _ = cursor.read_exact(&mut vec![0u8; payload_size]);
        } else {
            if id == "POOL" {
                pool_offset = Some(start as usize);
            }
            let _ = cursor.read_exact(&mut vec![0u8; size as usize]);
        }
    }

    let pool_offset = pool_offset.ok_or_else(|| "No POOL chunk found".to_string())?;
    let mut pool_cursor = Cursor::new(&bytes[pool_offset..]);

    // Skip POOL chunk header (4 bytes id + 4 bytes size)
    pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?; // id
    pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?; // size

    let _pool_type = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

    let comp_tex_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let decomp_tex_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let mut comp_tex = vec![0u8; comp_tex_len];
    pool_cursor.read_exact(&mut comp_tex).map_err(|e| e.to_string())?;
    let decomp_tex = if comp_tex_len == decomp_tex_len {
        comp_tex.clone()
    } else {
        xpress::decompress(&comp_tex, decomp_tex_len)?
    };

    let comp_mesh_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let decomp_mesh_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let mut comp_mesh = vec![0u8; comp_mesh_len];
    pool_cursor.read_exact(&mut comp_mesh).map_err(|e| e.to_string())?;
    let decomp_mesh = if comp_mesh_len == decomp_mesh_len {
        comp_mesh.clone()
    } else {
        xpress::decompress(&comp_mesh, decomp_mesh_len)?
    };

    let comp_face_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let decomp_face_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
    let mut comp_face = vec![0u8; comp_face_len];
    pool_cursor.read_exact(&mut comp_face).map_err(|e| e.to_string())?;
    let decomp_face = if comp_face_len == decomp_face_len {
        comp_face.clone()
    } else {
        xpress::decompress(&comp_face, decomp_face_len)?
    };

    Ok(PoolData {
        comp_tex_len,
        decomp_tex_len,
        comp_mesh_len,
        decomp_mesh_len,
        comp_face_len,
        decomp_face_len,
        decomp_tex,
        decomp_mesh,
        decomp_face,
    })
}
