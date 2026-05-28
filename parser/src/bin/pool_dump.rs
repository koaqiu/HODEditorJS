use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::xpress;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: pool_dump <hodor.hod> <generated.hod>");
        std::process::exit(1);
    }

    let hodor_bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let gen_bytes = fs::read(&args[2]).map_err(|e| e.to_string())?;

    let hodor_pool = extract_pool(&hodor_bytes)?;
    let gen_pool = extract_pool(&gen_bytes)?;

    // Show first 256 bytes of mesh pool for comparison
    println!("=== HODOR Mesh Pool (first 512 bytes) ===");
    for i in (0..512.min(hodor_pool.decomp_mesh.len())).step_by(64) {
        let end = (i + 64).min(hodor_pool.decomp_mesh.len());
        let chunk = &hodor_pool.decomp_mesh[i..end];
        // Decode as vertex: pos(12) + normal(12) + UV(8) + tangent(12) + binormal(12)
        if chunk.len() >= 56 {
            let px = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let py = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            let pz = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
            let pw = f32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]);
            println!("  vert[{}]: pos=({:.4}, {:.4}, {:.4}, {:.4}) raw={:02x?}", i/64, px, py, pz, pw, &chunk[0..16]);
        }
    }

    println!("\n=== Generated Mesh Pool (first 512 bytes) ===");
    for i in (0..512.min(gen_pool.decomp_mesh.len())).step_by(64) {
        let end = (i + 64).min(gen_pool.decomp_mesh.len());
        let chunk = &gen_pool.decomp_mesh[i..end];
        if chunk.len() >= 56 {
            let px = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let py = f32::from_le_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            let pz = f32::from_le_bytes([chunk[8], chunk[9], chunk[10], chunk[11]]);
            let pw = f32::from_le_bytes([chunk[12], chunk[13], chunk[14], chunk[15]]);
            println!("  vert[{}]: pos=({:.4}, {:.4}, {:.4}, {:.4}) raw={:02x?}", i/64, px, py, pz, pw, &chunk[0..16]);
        }
    }

    // Show where differences start
    println!("\n=== First 20 differences in mesh pool ===");
    let mut diff_count = 0;
    for i in 0..hodor_pool.decomp_mesh.len().min(gen_pool.decomp_mesh.len()) {
        if hodor_pool.decomp_mesh[i] != gen_pool.decomp_mesh[i] {
            println!("  offset {}: HODOR={:02x} Gen={:02x}", i, hodor_pool.decomp_mesh[i], gen_pool.decomp_mesh[i]);
            diff_count += 1;
            if diff_count >= 20 {
                break;
            }
        }
    }

    // Check if HODOR has more data after the vertex section
    println!("\n=== Pool size comparison ===");
    println!("HODOR mesh pool: {} bytes", hodor_pool.decomp_mesh.len());
    println!("Gen mesh pool:   {} bytes", gen_pool.decomp_mesh.len());
    println!("HODOR extra:     {} bytes", hodor_pool.decomp_mesh.len() - gen_pool.decomp_mesh.len());

    // Check if HODOR has extra data at the end
    if hodor_pool.decomp_mesh.len() > gen_pool.decomp_mesh.len() {
        let extra_start = gen_pool.decomp_mesh.len();
        let extra_end = hodor_pool.decomp_mesh.len().min(extra_start + 256);
        println!("\n=== HODOR extra data at end (offset {}..{}) ===", extra_start, extra_end);
        for i in (extra_start..extra_end).step_by(64) {
            let end = (i + 64).min(extra_end);
            let chunk = &hodor_pool.decomp_mesh[i..end];
            println!("  bytes[{}..{}]: {:02x?}", i, end, chunk);
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
