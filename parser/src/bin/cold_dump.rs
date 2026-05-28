use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::iff::IffChunk;
use std::fs;
use std::io::{Cursor, Read};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cold_dump <hod_file>");
        std::process::exit(1);
    }

    let bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let mut cursor = Cursor::new(&bytes);

    // Find DTRM chunk
    while cursor.position() < bytes.len() as u64 {
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
            let real_id = String::from_utf8_lossy(&real_id_bytes).to_string();
            let payload_size = (size - 4) as usize;

            if real_id == "DTRM" {
                let mut payload = vec![0u8; payload_size];
                cursor.read_exact(&mut payload).map_err(|e| e.to_string())?;
                let mut sub_cursor = Cursor::new(&payload);

                while sub_cursor.position() < payload.len() as u64 {
                    let sub_start = sub_cursor.position();
                    let mut sub_id_bytes = [0u8; 4];
                    if sub_cursor.read_exact(&mut sub_id_bytes).is_err() {
                        break;
                    }
                    let sub_id = String::from_utf8_lossy(&sub_id_bytes).to_string();
                    let mut sub_size_bytes = [0u8; 4];
                    sub_cursor.read_exact(&mut sub_size_bytes).map_err(|e| e.to_string())?;
                    let sub_size = u32::from_be_bytes(sub_size_bytes);

                    if sub_id == "FORM" {
                        let mut sub_real_id_bytes = [0u8; 4];
                        sub_cursor.read_exact(&mut sub_real_id_bytes).map_err(|e| e.to_string())?;
                        let sub_real_id = String::from_utf8_lossy(&sub_real_id_bytes).to_string();
                        let sub_payload_size = (sub_size - 4) as usize;

                        if sub_real_id == "COLD" {
                            println!("=== COLD chunk (payload={} bytes) ===", sub_payload_size);
                            let mut cold_payload = vec![0u8; sub_payload_size];
                            sub_cursor.read_exact(&mut cold_payload).map_err(|e| e.to_string())?;

                            // Parse COLD children
                            let mut cold_cursor = Cursor::new(&cold_payload);
                            while cold_cursor.position() < cold_payload.len() as u64 {
                                let child_start = cold_cursor.position();
                                let mut child_id_bytes = [0u8; 4];
                                if cold_cursor.read_exact(&mut child_id_bytes).is_err() {
                                    break;
                                }
                                let child_id = String::from_utf8_lossy(&child_id_bytes).to_string();
                                let mut child_size_bytes = [0u8; 4];
                                cold_cursor.read_exact(&mut child_size_bytes).map_err(|e| e.to_string())?;
                                let child_size = u32::from_be_bytes(child_size_bytes);

                                if child_id == "FORM" {
                                    let mut child_real_id_bytes = [0u8; 4];
                                    cold_cursor.read_exact(&mut child_real_id_bytes).map_err(|e| e.to_string())?;
                                    let child_real_id = String::from_utf8_lossy(&child_real_id_bytes).to_string();
                                    let child_payload_size = (child_size - 4) as usize;

                                    if child_real_id == "TRIS" {
                                        println!("\n--- TRIS chunk (payload={} bytes) ---", child_payload_size);
                                        let mut tris_payload = vec![0u8; child_payload_size];
                                        cold_cursor.read_exact(&mut tris_payload).map_err(|e| e.to_string())?;

                                        let mut tris_cursor = Cursor::new(&tris_payload);
                                        let vertex_count = tris_cursor.read_i32::<LittleEndian>().map_err(|e| e.to_string())?;
                                        println!("Vertex count: {}", vertex_count);

                                        let mut vertices = Vec::new();
                                        for i in 0..vertex_count {
                                            let vx = tris_cursor.read_f32::<LittleEndian>().map_err(|e| e.to_string())?;
                                            let vy = tris_cursor.read_f32::<LittleEndian>().map_err(|e| e.to_string())?;
                                            let vz = tris_cursor.read_f32::<LittleEndian>().map_err(|e| e.to_string())?;
                                            vertices.push((vx, vy, vz));
                                            if i < 20 {
                                                println!("  vert[{}]: ({:.4}, {:.4}, {:.4})", i, vx, vy, vz);
                                            }
                                        }
                                        if vertex_count > 20 {
                                            println!("  ... ({} more vertices)", vertex_count - 20);
                                        }

                                        let idx_count = tris_cursor.read_i32::<LittleEndian>().map_err(|e| e.to_string())?;
                                        println!("Index count: {}", idx_count);

                                        let mut indices = Vec::new();
                                        for i in 0..idx_count {
                                            let idx = tris_cursor.read_u16::<LittleEndian>().map_err(|e| e.to_string())?;
                                            indices.push(idx);
                                            if i < 30 {
                                                print!("{}", idx);
                                                if i < idx_count - 1 {
                                                    print!(", ");
                                                }
                                            }
                                        }
                                        if idx_count > 30 {
                                            print!("... ({} more)", idx_count - 30);
                                        }
                                        println!();

                                        // Analyze the collision mesh structure
                                        println!("\n--- Collision Mesh Analysis ---");
                                        println!("Vertices: {}", vertex_count);
                                        println!("Indices: {}", idx_count);
                                        println!("Triangles: {}", idx_count / 3);
                                        println!("Bytes per vertex: 12 (3 floats)");
                                        println!("Total vertex data: {} bytes", vertex_count * 12);
                                        println!("Total index data: {} bytes", idx_count * 2);
                                    } else {
                                        let mut child_data = vec![0u8; child_payload_size];
                                        cold_cursor.read_exact(&mut child_data).map_err(|e| e.to_string())?;
                                        println!("  FORM {} ({} bytes)", child_real_id, child_payload_size);
                                    }
                                } else {
                                    let mut child_data = vec![0u8; child_size as usize];
                                    cold_cursor.read_exact(&mut child_data).map_err(|e| e.to_string())?;
                                    println!("  {} ({} bytes)", child_id, child_size);
                                }
                            }
                        } else {
                            let mut sub_data = vec![0u8; sub_payload_size];
                            sub_cursor.read_exact(&mut sub_data).map_err(|e| e.to_string())?;
                        }
                    } else {
                        let mut sub_data = vec![0u8; sub_size as usize];
                        sub_cursor.read_exact(&mut sub_data).map_err(|e| e.to_string())?;
                    }
                }
            } else {
                let mut payload = vec![0u8; payload_size];
                cursor.read_exact(&mut payload).map_err(|e| e.to_string())?;
            }
        } else {
            let mut data = vec![0u8; size as usize];
            cursor.read_exact(&mut data).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}
