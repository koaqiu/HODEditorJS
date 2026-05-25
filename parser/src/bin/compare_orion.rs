use std::fs::File;
use std::io::{self, Read, Cursor};
use byteorder::{BigEndian, ReadBytesExt};

fn dump_iff_structure(bytes: &[u8], base_offset: usize, depth: usize) -> io::Result<()> {
    let indent = "  ".repeat(depth);
    let mut cursor = Cursor::new(bytes);
    
    while cursor.position() < bytes.len() as u64 {
        let chunk_start = base_offset + cursor.position() as usize;
        
        let mut id_bytes = [0u8; 4];
        if cursor.read_exact(&mut id_bytes).is_err() {
            break;
        }
        let raw_id = String::from_utf8_lossy(&id_bytes).to_string();
        let size = cursor.read_u32::<BigEndian>()? as usize;
        
        match raw_id.as_str() {
            "FORM" => {
                let mut real_id_bytes = [0u8; 4];
                cursor.read_exact(&mut real_id_bytes)?;
                let real_id = String::from_utf8_lossy(&real_id_bytes).to_string();
                
                println!(
                    "{}0x{:08X} | FORM:{} (Size: {})",
                    indent, chunk_start, real_id, size
                );
                
                let payload_size = size.saturating_sub(4);
                let mut payload = vec![0u8; payload_size];
                cursor.read_exact(&mut payload)?;
                
                let id_trimmed = real_id.trim();
                let is_container = matches!(
                    id_trimmed,
                    "HVMD" | "DTRM" | "BGMS" | "BSRM" | "COLD" | "GLOW" | "MRKR" | "KEYF" | "MSHL" | "MAD"
                );
                
                if is_container {
                    let payload_start = chunk_start + 12;
                    if id_trimmed == "COLD" {
                        if payload.len() >= 4 {
                            let mut len_bytes = [0u8; 4];
                            len_bytes.copy_from_slice(&payload[0..4]);
                            let len = u32::from_le_bytes(len_bytes) as usize;
                            
                            let has_extents = if payload.len() >= 4 + len + 40 {
                                let id_at_short = &payload[4 + len .. 4 + len + 4];
                                let is_valid_short = id_at_short.iter().all(|&b| b >= 32 && b <= 126);
                                !is_valid_short
                            } else {
                                false
                            };

                            let prefix_len = if has_extents && payload.len() >= 4 + len + 40 {
                                4 + len + 40
                            } else {
                                4 + len
                            };
                            
                            println!("{}  [COLD Info] len={}, has_extents={}, prefix_len={}, payload_len={}", indent, len, has_extents, prefix_len, payload.len());
                            if payload.len() >= prefix_len {
                                dump_iff_structure(&payload[prefix_len..], payload_start + prefix_len, depth + 1)?;
                            }
                        }
                    } else {
                        dump_iff_structure(&payload, payload_start, depth + 1)?;
                    }
                }
            }
            "NRML" => {
                let mut real_id_bytes = [0u8; 4];
                cursor.read_exact(&mut real_id_bytes)?;
                let real_id = String::from_utf8_lossy(&real_id_bytes).to_string();
                let version = cursor.read_u32::<BigEndian>()?;
                
                println!(
                    "{}0x{:08X} | NRML:{} (Size: {}, Version: {})",
                    indent, chunk_start, real_id, size, version
                );
                
                let payload_size = size.saturating_sub(8);
                let mut data = vec![0u8; payload_size];
                cursor.read_exact(&mut data)?;
            }
            _ => {
                println!(
                    "{}0x{:08X} | DEFT:{} (Size: {})",
                    indent, chunk_start, raw_id, size
                );
                
                let mut data = vec![0u8; size];
                cursor.read_exact(&mut data)?;
            }
        }
    }
    
    Ok(())
}

fn print_hex(label: &str, bytes: &[u8], limit: usize) {
    println!("--- {} (Total Size: {}) ---", label, bytes.len());
    let actual_limit = bytes.len().min(limit);
    for chunk_slice in bytes[..actual_limit].chunks(16) {
        let hex_strs: Vec<String> = chunk_slice.iter().map(|b| format!("{:02X}", b)).collect();
        let ascii_strs: String = chunk_slice.iter().map(|&b| {
            if b >= 32 && b <= 126 { b as char } else { '.' }
        }).collect();
        println!("  {:48} | {}", hex_strs.join(" "), ascii_strs);
    }
}

fn analyze_file(path: &str, name: &str) -> io::Result<()> {
    println!("\n==================================================");
    println!("ANALYZING: {} ({})", name, path);
    println!("==================================================");
    
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    
    println!("--- Chunk Structure ---");
    dump_iff_structure(&bytes, 0, 0)?;
    
    let mut cursor = Cursor::new(&bytes);
    while cursor.position() < bytes.len() as u64 {
        let chunk_start = cursor.position() as usize;
        let mut id_bytes = [0u8; 4];
        if cursor.read_exact(&mut id_bytes).is_err() {
            break;
        }
        let raw_id = String::from_utf8_lossy(&id_bytes).to_string();
        let size = cursor.read_u32::<BigEndian>()? as usize;
        
        if raw_id == "FORM" {
            let mut real_id_bytes = [0u8; 4];
            cursor.read_exact(&mut real_id_bytes)?;
            let real_id = String::from_utf8_lossy(&real_id_bytes).to_string();
            let real_id_trimmed = real_id.trim();
            
            if real_id_trimmed == "DTRM" {
                let payload_size = size.saturating_sub(4);
                let mut payload = vec![0u8; payload_size];
                cursor.read_exact(&mut payload)?;
                
                let mut sub_cursor = Cursor::new(&payload);
                while sub_cursor.position() < payload.len() as u64 {
                    let mut sub_id_bytes = [0u8; 4];
                    if sub_cursor.read_exact(&mut sub_id_bytes).is_err() {
                        break;
                    }
                    let sub_id = String::from_utf8_lossy(&sub_id_bytes).to_string();
                    let sub_size = sub_cursor.read_u32::<BigEndian>()? as usize;
                    
                    if sub_id == "FORM" {
                        let mut sub_real_id_bytes = [0u8; 4];
                        sub_cursor.read_exact(&mut sub_real_id_bytes)?;
                        let sub_real_id = String::from_utf8_lossy(&sub_real_id_bytes).to_string();
                        let sub_real_id_trimmed = sub_real_id.trim();
                        
                        let sub_payload_size = sub_size.saturating_sub(4);
                        let mut sub_payload = vec![0u8; sub_payload_size];
                        sub_cursor.read_exact(&mut sub_payload)?;
                        
                        if ["HIER", "COLD"].contains(&sub_real_id_trimmed) {
                            print_hex(&format!("FORM:{}", sub_real_id_trimmed), &sub_payload, 64);
                        }
                    } else if sub_id == "NRML" {
                        let mut sub_real_id_bytes = [0u8; 4];
                        sub_cursor.read_exact(&mut sub_real_id_bytes)?;
                        let sub_real_id = String::from_utf8_lossy(&sub_real_id_bytes).to_string();
                        let sub_real_id_trimmed = sub_real_id.trim();
                        let sub_version = sub_cursor.read_u32::<BigEndian>()?;
                        
                        let sub_payload_size = sub_size.saturating_sub(8);
                        let mut sub_payload = vec![0u8; sub_payload_size];
                        sub_cursor.read_exact(&mut sub_payload)?;
                        
                        if ["NAVL", "SCAR"].contains(&sub_real_id_trimmed) {
                            print_hex(&format!("NRML:{} (Version: {})", sub_real_id_trimmed, sub_version), &sub_payload, 64);
                        }
                    } else {
                        let mut sub_payload = vec![0u8; sub_size];
                        sub_cursor.read_exact(&mut sub_payload)?;
                        
                        if ["HIER", "NAVL", "DOCK", "COLD", "MRKS", "KDOP"].contains(&sub_id.trim()) {
                            print_hex(&format!("DEFT:{}", sub_id.trim()), &sub_payload, 64);
                        }
                    }
                }
            } else {
                cursor.set_position((chunk_start + 8 + size) as u64);
            }
        } else {
            cursor.set_position((chunk_start + 8 + size) as u64);
        }
    }
    
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let (path1, path2) = if args.len() >= 3 {
        (args[1].clone(), args[2].clone())
    } else {
        println!("Usage: cargo run --bin compare_orion <file1.hod> <file2.hod>");
        println!("Falling back to default paths...");
        ("/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium.hod".to_string(),
         "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld/HWRM_FSFC/source/ship/ter_elysium/ter_elysium2.hod".to_string())
    };
    
    let name1 = std::path::Path::new(&path1)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file1.hod");
    let name2 = std::path::Path::new(&path2)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file2.hod");

    analyze_file(&path1, name1)?;
    analyze_file(&path2, name2)?;
    
    Ok(())
}
