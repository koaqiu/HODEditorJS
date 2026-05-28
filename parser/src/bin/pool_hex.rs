use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::xpress;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: pool_hex <hodor.hod> <generated.hod>");
        std::process::exit(1);
    }

    let hodor_bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let gen_bytes = fs::read(&args[2]).map_err(|e| e.to_string())?;

    let hodor_pool = extract_pool(&hodor_bytes)?;
    let gen_pool = extract_pool(&gen_bytes)?;

    // Show the structure of both mesh pools
    println!("=== HODOR Mesh Pool Structure ===");
    println!("Total size: {} bytes", hodor_pool.decomp_mesh.len());
    
    // Parse BMSH headers from the pool
    // Each LOD has: TAGS (optional) + BMSH
    // BMSH header: lod(4) + part_count(4) + parts...
    // Part: material_index(4) + vertex_mask(4) + vertex_count(4) + index_count(4) + vertex_data + index_data
    
    let mut offset = 0;
    let mut lod = 0;
    while offset + 16 <= hodor_pool.decomp_mesh.len() {
        let chunk_id = String::from_utf8_lossy(&hodor_pool.decomp_mesh[offset..offset+4]).to_string();
        let chunk_size = u32::from_le_bytes([
            hodor_pool.decomp_mesh[offset+4],
            hodor_pool.decomp_mesh[offset+5],
            hodor_pool.decomp_mesh[offset+6],
            hodor_pool.decomp_mesh[offset+7],
        ]) as usize;
        
        if chunk_id == "NRML" || chunk_id == "FORM" {
            let real_id = String::from_utf8_lossy(&hodor_pool.decomp_mesh[offset+8..offset+12]).to_string();
            println!("  offset {}: {} {} (payload={})", offset, chunk_id, real_id, chunk_size - 4);
            offset += 8 + (chunk_size - 4);
        } else if chunk_id == "BMSH" {
            let bmsh_size = chunk_size;
            let bmsh_data = &hodor_pool.decomp_mesh[offset+8..offset+8+bmsh_size];
            if bmsh_data.len() >= 16 {
                let lod_val = i32::from_le_bytes([bmsh_data[0], bmsh_data[1], bmsh_data[2], bmsh_data[3]]);
                let part_count = i32::from_le_bytes([bmsh_data[4], bmsh_data[5], bmsh_data[6], bmsh_data[7]]);
                println!("  offset {}: BMSH lod={} parts={} (size={})", offset, lod_val, part_count, bmsh_size);
                
                let mut pos = 8;
                for p in 0..part_count {
                    if pos + 12 <= bmsh_data.len() {
                        let mat_idx = i32::from_le_bytes([bmsh_data[pos], bmsh_data[pos+1], bmsh_data[pos+2], bmsh_data[pos+3]]);
                        let vert_mask = u32::from_le_bytes([bmsh_data[pos+4], bmsh_data[pos+5], bmsh_data[pos+6], bmsh_data[pos+7]]);
                        let vert_count = i32::from_le_bytes([bmsh_data[pos+8], bmsh_data[pos+9], bmsh_data[pos+10], bmsh_data[pos+11]]);
                        println!("    Part {}: mat={} mask=0x{:04X} verts={}", p, mat_idx, vert_mask, vert_count);
                        pos += 12;
                    }
                }
            }
            offset += 8 + chunk_size;
        } else {
            println!("  offset {}: {} (size={})", offset, chunk_id, chunk_size);
            offset += 8 + chunk_size;
        }
        
        if offset > hodor_pool.decomp_mesh.len() {
            break;
        }
    }

    println!("\n=== Generated Mesh Pool Structure ===");
    println!("Total size: {} bytes", gen_pool.decomp_mesh.len());
    
    let mut offset = 0;
    while offset + 16 <= gen_pool.decomp_mesh.len() {
        let chunk_id = String::from_utf8_lossy(&gen_pool.decomp_mesh[offset..offset+4]).to_string();
        let chunk_size = u32::from_le_bytes([
            gen_pool.decomp_mesh[offset+4],
            gen_pool.decomp_mesh[offset+5],
            gen_pool.decomp_mesh[offset+6],
            gen_pool.decomp_mesh[offset+7],
        ]) as usize;
        
        if chunk_id == "NRML" || chunk_id == "FORM" {
            let real_id = String::from_utf8_lossy(&gen_pool.decomp_mesh[offset+8..offset+12]).to_string();
            println!("  offset {}: {} {} (payload={})", offset, chunk_id, real_id, chunk_size - 4);
            offset += 8 + (chunk_size - 4);
        } else if chunk_id == "BMSH" {
            let bmsh_size = chunk_size;
            let bmsh_data = &gen_pool.decomp_mesh[offset+8..offset+8+bmsh_size];
            if bmsh_data.len() >= 16 {
                let lod_val = i32::from_le_bytes([bmsh_data[0], bmsh_data[1], bmsh_data[2], bmsh_data[3]]);
                let part_count = i32::from_le_bytes([bmsh_data[4], bmsh_data[5], bmsh_data[6], bmsh_data[7]]);
                println!("  offset {}: BMSH lod={} parts={} (size={})", offset, lod_val, part_count, bmsh_size);
                
                let mut pos = 8;
                for p in 0..part_count {
                    if pos + 12 <= bmsh_data.len() {
                        let mat_idx = i32::from_le_bytes([bmsh_data[pos], bmsh_data[pos+1], bmsh_data[pos+2], bmsh_data[pos+3]]);
                        let vert_mask = u32::from_le_bytes([bmsh_data[pos+4], bmsh_data[pos+5], bmsh_data[pos+6], bmsh_data[pos+7]]);
                        let vert_count = i32::from_le_bytes([bmsh_data[pos+8], bmsh_data[pos+9], bmsh_data[pos+10], bmsh_data[pos+11]]);
                        println!("    Part {}: mat={} mask=0x{:04X} verts={}", p, mat_idx, vert_mask, vert_count);
                        pos += 12;
                    }
                }
            }
            offset += 8 + chunk_size;
        } else {
            println!("  offset {}: {} (size={})", offset, chunk_id, chunk_size);
            offset += 8 + chunk_size;
        }
        
        if offset > gen_pool.decomp_mesh.len() {
            break;
        }
    }

    Ok(())
}

struct PoolData {
    decomp_tex: Vec<u8>,
    decomp_mesh: Vec<u8>,
    decomp_face: Vec<u8>,
}

fn extract_pool(bytes: &[u8]) -> Result<PoolData, String> {
    let mut cursor = Cursor::new(bytes);
    let mut pool_offset = None;

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

    pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;
    pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

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
        decomp_tex,
        decomp_mesh,
        decomp_face,
    })
}
