use byteorder::{LittleEndian, ReadBytesExt};
use hwr_hod_parser::iff::IffChunk;
use std::io::{Cursor, Seek, SeekFrom};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: xpress_decomp_test <hodor_hod_path>");
        std::process::exit(1);
    }

    let bytes = std::fs::read(&args[1])?;
    let mut cursor = Cursor::new(&bytes);

    // Find POOL chunk
    while cursor.position() < bytes.len() as u64 {
        let chunk = IffChunk::read_chunk(&mut cursor)?;
        if chunk.id == "POOL" {
            let mut pc = Cursor::new(&chunk.data);
            let _pool_type = pc.read_u32::<LittleEndian>()?;
            let comp_mesh = pc.read_u32::<LittleEndian>()?;
            let decomp_mesh = pc.read_u32::<LittleEndian>()?;
            println!("Mesh pool: comp={}, decomp={}", comp_mesh, decomp_mesh);

            let mesh_start = pc.position() as usize;
            let mesh_end = mesh_start + comp_mesh as usize;
            let comp_mesh_data = &chunk.data[mesh_start..mesh_end];

            // Decompress with 31-bit indicator (our current)
            let decomp_31 = decompress_31bit(comp_mesh_data, decomp_mesh as usize)?;

            // Decompress with 32-bit indicator
            let decomp_32 = decompress_32bit(comp_mesh_data, decomp_mesh as usize)?;

            // Compare
            let mut diff_31 = 0;
            let mut diff_32 = 0;
            for i in 0..(decomp_mesh as usize).min(decomp_31.len()).min(decomp_32.len()) {
                if decomp_31[i] != decomp_32[i] {
                    diff_31 += 1;
                }
            }

            println!("31-bit decompressed: {} bytes", decomp_31.len());
            println!("32-bit decompressed: {} bytes", decomp_32.len());
            println!("Differences between 31-bit and 32-bit: {} bytes", diff_31);

            // Now compress with our compressor and decompress with both
            let our_compressed = hwr_hod_parser::xpress::compress(&decomp_31);
            let our_decomp_31 = decompress_31bit(&our_compressed, decomp_mesh as usize)?;
            let our_decomp_32 = decompress_32bit(&our_compressed, decomp_mesh as usize)?;

            println!("\nOur compressed decompressed with 31-bit: {} bytes", our_decomp_31.len());
            println!("Our compressed decompressed with 32-bit: {} bytes", our_decomp_32.len());

            // Check if 32-bit decompression of HODOR matches 32-bit decompression of ours
            let mut match_32 = true;
            for i in 0..(decomp_mesh as usize).min(decomp_32.len()).min(our_decomp_32.len()) {
                if decomp_32[i] != our_decomp_32[i] {
                    match_32 = false;
                    println!("32-bit mismatch at byte {}: HODOR={:02X}, ours={:02X}", i, decomp_32[i], our_decomp_32[i]);
                    break;
                }
            }
            if match_32 {
                println!("\n32-bit decompression of both produces IDENTICAL output!");
            }

            // Check bit 31 in all indicator words
            println!("\n=== Indicator Word Analysis ===");
            let mut idx = 0;
            let mut word_num = 0;
            while idx + 4 <= comp_mesh_data.len() {
                let word = u32::from_le_bytes([
                    comp_mesh_data[idx],
                    comp_mesh_data[idx+1],
                    comp_mesh_data[idx+2],
                    comp_mesh_data[idx+3],
                ]);
                let bit31 = (word >> 31) & 1;
                if bit31 == 1 {
                    println!("Indicator word {} at offset 0x{:06X}: 0x{:08X} (bit 31 SET)", word_num, idx, word);
                }
                word_num += 1;
                idx += 4;
            }

            return Ok(());
        }
    }

    eprintln!("No POOL chunk found");
    Ok(())
}

/// Decompress using 31 bits per indicator (current behavior)
fn decompress_31bit(input: &[u8], output_size: usize) -> Result<Vec<u8>, String> {
    let mut output = vec![0u8; output_size];
    let mut output_idx = 0;
    let mut input_idx = 0;
    let mut indicator = 0u32;
    let mut indicator_bit = 30;

    while output_idx < output_size && input_idx < input.len() {
        indicator_bit += 1;
        if indicator_bit == 31 {
            if input_idx + 4 > input.len() { break; }
            indicator = u32::from_le_bytes([input[input_idx], input[input_idx+1], input[input_idx+2], input[input_idx+3]]);
            input_idx += 4;
            indicator_bit = 0;
        }

        if ((indicator >> indicator_bit) & 1) == 0 {
            if input_idx >= input.len() { break; }
            output[output_idx] = input[input_idx];
            input_idx += 1;
            output_idx += 1;
        } else {
            if input_idx + 1 >= input.len() { break; }
            let byte1 = input[input_idx];
            let (length, offset, consumed) = decode_match(byte1, input, input_idx)?;
            input_idx += consumed;

            for _ in 0..length {
                if output_idx >= output_size { break; }
                let src = output_idx.checked_sub(offset);
                if let Some(s) = src {
                    output[output_idx] = output[s];
                }
                output_idx += 1;
            }
        }
    }
    output.truncate(output_idx);
    Ok(output)
}

/// Decompress using 32 bits per indicator
fn decompress_32bit(input: &[u8], output_size: usize) -> Result<Vec<u8>, String> {
    let mut output = vec![0u8; output_size];
    let mut output_idx = 0;
    let mut input_idx = 0;
    let mut indicator = 0u32;
    let mut indicator_bit = 31; // Start at 31 so first iteration reads immediately

    while output_idx < output_size && input_idx < input.len() {
        indicator_bit += 1;
        if indicator_bit == 32 {
            if input_idx + 4 > input.len() { break; }
            indicator = u32::from_le_bytes([input[input_idx], input[input_idx+1], input[input_idx+2], input[input_idx+3]]);
            input_idx += 4;
            indicator_bit = 0;
        }

        if ((indicator >> indicator_bit) & 1) == 0 {
            if input_idx >= input.len() { break; }
            output[output_idx] = input[input_idx];
            input_idx += 1;
            output_idx += 1;
        } else {
            if input_idx + 1 >= input.len() { break; }
            let byte1 = input[input_idx];
            let (length, offset, consumed) = decode_match(byte1, input, input_idx)?;
            input_idx += consumed;

            for _ in 0..length {
                if output_idx >= output_size { break; }
                let src = output_idx.checked_sub(offset);
                if let Some(s) = src {
                    output[output_idx] = output[s];
                }
                output_idx += 1;
            }
        }
    }
    output.truncate(output_idx);
    Ok(output)
}

fn decode_match(byte1: u8, input: &[u8], idx: usize) -> Result<(usize, usize, usize), String> {
    if (byte1 & 0b11) == 0 {
        Ok((3, (byte1 >> 2) as usize, 1))
    } else if (byte1 & 0b11) == 0b10 {
        let byte2 = input[idx + 1] as usize;
        let length = (((byte1 >> 2) & 0b1111) + 3) as usize;
        let offset = (byte2 << 2) | ((byte1 >> 6) as usize);
        Ok((length, offset, 2))
    } else if (byte1 & 0b11) == 0b01 {
        let byte2 = input[idx + 1] as usize;
        let offset = (byte2 << 6) | ((byte1 >> 2) as usize);
        Ok((3, offset, 2))
    } else if (byte1 & 0b111) == 0b111 {
        let byte2 = input[idx + 1] as usize;
        let byte3 = input[idx + 2] as usize;
        let byte4 = input[idx + 3] as usize;
        let length = (((byte2 & 0b111) << 5) | ((byte1 >> 3) as usize)) + 3;
        let offset = (byte4 << 13) | (byte3 << 5) | (byte2 >> 3);
        Ok((length, offset, 4))
    } else if (byte1 & 0b11) == 0b11 {
        let byte2 = input[idx + 1] as usize;
        let byte3 = input[idx + 2] as usize;
        let length = ((byte1 >> 3) + 3) as usize;
        let offset = (byte3 << 8) | byte2;
        Ok((length, offset, 3))
    } else {
        Err("Invalid match header".to_string())
    }
}
