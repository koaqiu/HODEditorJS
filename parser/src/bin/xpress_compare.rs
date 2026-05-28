use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::iff::IffChunk;
use std::io::{Cursor, Read, Seek, SeekFrom};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: xpress_compare <hodor_hod_path>");
        std::process::exit(1);
    }

    let bytes = std::fs::read(&args[1])?;
    let mut cursor = Cursor::new(&bytes);

    // Find POOL chunk
    while cursor.position() < bytes.len() as u64 {
        let chunk = IffChunk::read_chunk(&mut cursor)?;
        if chunk.id == "POOL" {
            let mut pc = Cursor::new(&chunk.data);
            let pool_type = pc.read_u32::<LittleEndian>()?;
            let comp_tex = pc.read_u32::<LittleEndian>()?;
            let decomp_tex = pc.read_u32::<LittleEndian>()?;
            println!("POOL type={}, comp_tex={}, decomp_tex={}", pool_type, comp_tex, decomp_tex);

            // Read compressed texture data
            let tex_start = pc.position() as usize;
            let tex_end = tex_start + comp_tex as usize;
            let comp_tex_data = &chunk.data[tex_start..tex_end];
            pc.seek(SeekFrom::Start(tex_end as u64))?;

            // Decompress texture pool
            let decomp_tex_data = if comp_tex == decomp_tex {
                comp_tex_data.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_tex_data, decomp_tex as usize)?
            };

            // Now re-compress with our compressor
            let our_compressed = hwr_hod_parser::xpress::compress(&decomp_tex_data);

            // Decompress both to verify they produce the same output
            let our_decomp = hwr_hod_parser::xpress::decompress(&our_compressed, decomp_tex as usize)?;
            assert_eq!(decomp_tex_data, our_decomp, "Round-trip mismatch!");

            println!("\n=== Texture Pool ===");
            println!("HODOR compressed: {} bytes", comp_tex);
            println!("Our compressed:   {} bytes", our_compressed.len());
            println!("Decompressed:     {} bytes", decomp_tex);

            // Compare compressed bytes
            let min_len = comp_tex.min(our_compressed.len() as u32) as usize;
            let mut first_diff = None;
            for i in 0..min_len {
                if comp_tex_data[i] != our_compressed[i] {
                    first_diff = Some(i);
                    break;
                }
            }
            match first_diff {
                Some(pos) => {
                    println!("\nFirst difference at byte offset: 0x{:06X} ({})", pos, pos);
                    let start = pos.saturating_sub(8);
                    let end = (pos + 16).min(min_len);
                    println!("HODOR [0x{:06X}..0x{:06X}]:", start, end);
                    print_hex(&comp_tex_data[start..end]);
                    println!("Ours  [0x{:06X}..0x{:06X}]:", start, end);
                    print_hex(&our_compressed[start..end]);

                    // Decode the match type at this position
                    let h_b0 = comp_tex_data[pos];
                    let o_b0 = our_compressed[pos];
                    println!("\nHODOR byte[{}]: 0x{:02X} ({:08b})", pos, h_b0, h_b0);
                    println!("Ours  byte[{}]: 0x{:02X} ({:08b})", pos, o_b0, o_b0);
                    println!("HODOR match type: {}", classify_match(h_b0));
                    println!("Ours  match type: {}", classify_match(o_b0));
                }
                None => {
                    if comp_tex as usize != our_compressed.len() {
                        println!("Bytes match up to {}, but lengths differ ({} vs {})", min_len, comp_tex, our_compressed.len());
                        if comp_tex as usize > min_len {
                            println!("HODOR has extra bytes:");
                            print_hex(&comp_tex_data[min_len..]);
                        } else {
                            println!("We have extra bytes:");
                            print_hex(&our_compressed[min_len..]);
                        }
                    } else {
                        println!("Compressed bytes are IDENTICAL!");
                    }
                }
            }

            // Now do mesh pool
            let comp_mesh = pc.read_u32::<LittleEndian>()?;
            let decomp_mesh = pc.read_u32::<LittleEndian>()?;
            let mesh_start = pc.position() as usize;
            let mesh_end = mesh_start + comp_mesh as usize;
            let comp_mesh_data = &chunk.data[mesh_start..mesh_end];
            pc.seek(SeekFrom::Start(mesh_end as u64))?;

            let decomp_mesh_data = if comp_mesh == decomp_mesh {
                comp_mesh_data.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_mesh_data, decomp_mesh as usize)?
            };

            let our_mesh_compressed = hwr_hod_parser::xpress::compress(&decomp_mesh_data);
            let our_mesh_decomp = hwr_hod_parser::xpress::decompress(&our_mesh_compressed, decomp_mesh as usize)?;
            assert_eq!(decomp_mesh_data, our_mesh_decomp, "Mesh round-trip mismatch!");

            println!("\n=== Mesh Pool ===");
            println!("HODOR compressed: {} bytes", comp_mesh);
            println!("Our compressed:   {} bytes", our_mesh_compressed.len());
            println!("Decompressed:     {} bytes", decomp_mesh);

            let min_len = (comp_mesh as usize).min(our_mesh_compressed.len());
            let mut first_diff = None;
            for i in 0..min_len {
                if comp_mesh_data[i] != our_mesh_compressed[i] {
                    first_diff = Some(i);
                    break;
                }
            }
            match first_diff {
                Some(pos) => {
                    println!("\nFirst difference at byte offset: 0x{:06X} ({})", pos, pos);
                    let start = pos.saturating_sub(8);
                    let end = (pos + 16).min(min_len);
                    println!("HODOR [0x{:06X}..0x{:06X}]:", start, end);
                    print_hex(&comp_mesh_data[start..end]);
                    println!("Ours  [0x{:06X}..0x{:06X}]:", start, end);
                    print_hex(&our_mesh_compressed[start..end]);

                    let h_b0 = comp_mesh_data[pos];
                    let o_b0 = our_mesh_compressed[pos];
                    println!("\nHODOR byte[{}]: 0x{:02X} ({:08b})", pos, h_b0, h_b0);
                    println!("Ours  byte[{}]: 0x{:02X} ({:08b})", pos, o_b0, o_b0);
                    println!("HODOR match type: {}", classify_match(h_b0));
                    println!("Ours  match type: {}", classify_match(o_b0));
                }
                None => {
                    if comp_mesh as usize != our_mesh_compressed.len() {
                        println!("Bytes match up to {}, but lengths differ ({} vs {})", min_len, comp_mesh, our_mesh_compressed.len());
                    } else {
                        println!("Compressed bytes are IDENTICAL!");
                    }
                }
            }

            // Face pool
            let comp_face = pc.read_u32::<LittleEndian>()?;
            let decomp_face = pc.read_u32::<LittleEndian>()?;
            let face_start = pc.position() as usize;
            let face_end = face_start + comp_face as usize;
            let comp_face_data = &chunk.data[face_start..face_end];

            let decomp_face_data = if comp_face == decomp_face {
                comp_face_data.to_vec()
            } else {
                hwr_hod_parser::xpress::decompress(comp_face_data, decomp_face as usize)?
            };

            let our_face_compressed = hwr_hod_parser::xpress::compress(&decomp_face_data);

            println!("\n=== Face Pool ===");
            println!("HODOR compressed: {} bytes", comp_face);
            println!("Our compressed:   {} bytes", our_face_compressed.len());
            println!("Decompressed:     {} bytes", decomp_face);

            if comp_face as usize == our_face_compressed.len() && comp_face_data == our_face_compressed.as_slice() {
                println!("Compressed bytes are IDENTICAL!");
            } else {
                let min_len = (comp_face as usize).min(our_face_compressed.len());
                let mut first_diff = None;
                for i in 0..min_len {
                    if comp_face_data[i] != our_face_compressed[i] {
                        first_diff = Some(i);
                        break;
                    }
                }
                match first_diff {
                    Some(pos) => println!("First difference at byte offset: 0x{:06X}", pos),
                    None => println!("Bytes match up to {}, lengths differ", min_len),
                }
            }

            return Ok(());
        }
    }

    eprintln!("No POOL chunk found");
    Ok(())
}

fn classify_match(byte: u8) -> &'static str {
    if (byte & 0b11) == 0 {
        "Type 0 (1-byte, offset<64, len=3)"
    } else if (byte & 0b11) == 0b10 {
        "Type 1 (2-byte, offset<1024, len 3-18)"
    } else if (byte & 0b11) == 0b01 {
        "Type 2 (2-byte, offset<16384, len=3)"
    } else if (byte & 0b111) == 0b111 {
        "Type 3 (4-byte, offset up to 2MB)"
    } else if (byte & 0b11) == 0b11 {
        "Type 4 (3-byte, offset up to 65535)"
    } else {
        "UNKNOWN"
    }
}

fn print_hex(data: &[u8]) {
    for (i, chunk) in data.chunks(16).enumerate() {
        print!("  {:06X}: ", i * 16);
        for b in chunk {
            print!("{:02X} ", b);
        }
        println!();
    }
}
