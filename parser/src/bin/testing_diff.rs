use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::hod::{generate_v2_from_model, HODModel};
use hwr_hod_parser::iff::IffChunk;
use std::fs;
use std::io::Cursor;
use std::path::Path;

fn main() -> Result<(), String> {
    let root = Path::new("../testing");
    for name in ["pebble_0", "pebble_1", "pebble_2"] {
        let dir = root.join(name);
        println!("\n=== {} ===", name);
        let vanilla_path = dir.join(format!("{}_vanilla.hod", name));
        let assets_path = dir.join(format!("{}_from_assets.hod", name));
        let vanilla_bytes = fs::read(&vanilla_path).map_err(|e| e.to_string())?;
        let vanilla_model = HODModel::parse_with_external(
            &vanilla_bytes,
            Some(vanilla_path.to_string_lossy().as_ref()),
            Some(dir.to_string_lossy().as_ref()),
        )?;
        let roundtrip_bytes = generate_v2_from_model(&[], &vanilla_model)?;
        let roundtrip_path = dir.join(format!("{}_roundtrip.hod", name));
        fs::write(&roundtrip_path, &roundtrip_bytes).map_err(|e| e.to_string())?;
        let assets_bytes = fs::read(&assets_path).map_err(|e| e.to_string())?;

        print_file_summary("vanilla", &vanilla_bytes)?;
        print_file_summary("roundtrip", &roundtrip_bytes)?;
        print_file_summary("from_assets", &assets_bytes)?;
        print_first_diff("vanilla_vs_roundtrip", &vanilla_bytes, &roundtrip_bytes);
        print_first_diff("vanilla_vs_assets", &vanilla_bytes, &assets_bytes);
    }
    Ok(())
}

fn print_file_summary(label: &str, bytes: &[u8]) -> Result<(), String> {
    let chunks = read_chunks(bytes)?;
    println!("{}: size={} chunks={}", label, bytes.len(), chunks.len());
    for chunk in &chunks {
        println!(
            "  {} {} data={} children={}",
            label,
            chunk.id,
            chunk.data.len(),
            chunk.children.len()
        );
        if chunk.id == "POOL" {
            print_pool(label, &chunk.data)?;
        }
        if chunk.id == "HVMD" || chunk.id == "DTRM" || chunk.id == "INFO" {
            for child in &chunk.children {
                println!(
                    "    {} child {} data={} children={}",
                    label,
                    child.id,
                    child.data.len(),
                    child.children.len()
                );
                if child.id == "KDOP" {
                    println!(
                        "      {} KDOP hash={} first16={:02x?}",
                        label,
                        hash_bytes(&child.data),
                        &child.data[..child.data.len().min(16)]
                    );
                }
                for grandchild in &child.children {
                    println!(
                        "      {} grandchild {} data={} children={}",
                        label,
                        grandchild.id,
                        grandchild.data.len(),
                        grandchild.children.len()
                    );
                }
            }
        }
    }
    Ok(())
}

fn read_chunks(bytes: &[u8]) -> Result<Vec<IffChunk>, String> {
    let mut cursor = Cursor::new(bytes);
    let mut chunks = Vec::new();
    while cursor.position() < bytes.len() as u64 {
        chunks.push(IffChunk::read_chunk(&mut cursor).map_err(|e| e.to_string())?);
    }
    Ok(chunks)
}

fn print_pool(label: &str, data: &[u8]) -> Result<(), String> {
    let mut cursor = Cursor::new(data);
    let pool_type = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())?;
    let tex_comp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let tex_decomp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let tex_bytes = &data[cursor.position() as usize..cursor.position() as usize + tex_comp];
    cursor.set_position(cursor.position() + tex_comp as u64);
    let mesh_comp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let mesh_decomp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let mesh_bytes = &data[cursor.position() as usize..cursor.position() as usize + mesh_comp];
    cursor.set_position(cursor.position() + mesh_comp as u64);
    let face_comp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let face_decomp = cursor
        .read_u32::<LittleEndian>()
        .map_err(|e| e.to_string())? as usize;
    let face_bytes = &data[cursor.position() as usize..cursor.position() as usize + face_comp];

    let tex_stats = analyze_xpress_stream(tex_bytes);
    let mesh_stats = analyze_xpress_stream(mesh_bytes);
    let face_stats = analyze_xpress_stream(face_bytes);
    println!(
        "    {} POOL type={} tex={}/{}(inds={} lits={} matches={}) mesh={}/{}(inds={} lits={} matches={}) face={}/{}(inds={} lits={} matches={})",
        label, pool_type,
        tex_comp, tex_decomp, tex_stats.0, tex_stats.1, tex_stats.2,
        mesh_comp, mesh_decomp, mesh_stats.0, mesh_stats.1, mesh_stats.2,
        face_comp, face_decomp, face_stats.0, face_stats.1, face_stats.2
    );
    Ok(())
}

/// Returns (indicator_word_count, literal_count, match_count)
fn analyze_xpress_stream(stream: &[u8]) -> (usize, usize, usize) {
    if stream.len() < 4 {
        return (0, 0, 0);
    }
    let mut indicators = 0;
    let mut lits = 0;
    let mut matches = 0;
    let mut pos = 0;
    while pos + 4 <= stream.len() {
        indicators += 1;
        let indicator = u32::from_le_bytes([
            stream[pos],
            stream[pos + 1],
            stream[pos + 2],
            stream[pos + 3],
        ]);
        pos += 4;
        for bit in 0..31 {
            if pos >= stream.len() {
                break;
            }
            if ((indicator >> bit) & 1) == 0 {
                pos += 1;
                lits += 1;
            } else {
                if pos >= stream.len() {
                    break;
                }
                let byte1 = stream[pos];
                let nmatch_bytes = match byte1 & 0b11 {
                    0 => 1,
                    1 => 2,
                    2 => 2,
                    3 => {
                        if (byte1 & 0b111) == 0b111 {
                            4
                        } else {
                            3
                        }
                    }
                    _ => 3,
                };
                pos += nmatch_bytes;
                matches += 1;
            }
        }
    }
    (indicators, lits, matches)
}

fn print_first_diff(label: &str, a: &[u8], b: &[u8]) {
    let min_len = a.len().min(b.len());
    let diff = (0..min_len).find(|&idx| a[idx] != b[idx]);
    match diff {
        Some(idx) => println!(
            "{}: first_diff={} a={:02x} b={:02x} len {}->{}",
            label,
            idx,
            a[idx],
            b[idx],
            a.len(),
            b.len()
        ),
        None => println!(
            "{}: common_prefix={} len {}->{}",
            label,
            min_len,
            a.len(),
            b.len()
        ),
    }
}

fn hash_bytes(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
