use std::fs::File;
use std::io::{self, Cursor, Read};
use hwr_hod_parser::hod::HODModel;
use hwr_hod_parser::iff::IffChunk;
use hwr_hod_parser::xpress;
use byteorder::{LittleEndian, ReadBytesExt};

fn decompress_pools(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), String> {
    let mut cursor = Cursor::new(bytes);
    let mut chunks = Vec::new();
    while cursor.position() < bytes.len() as u64 {
        match IffChunk::read_chunk(&mut cursor) {
            Ok(chunk) => chunks.push(chunk),
            Err(e) => return Err(format!("IFF read error: {}", e)),
        }
    }

    for chunk in &chunks {
        if chunk.id == "POOL" {
            let mut pool_cursor = Cursor::new(&chunk.data);
            let _pool_type = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())?;

            let comp_tex_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let decomp_tex_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let mut comp_tex = vec![0u8; comp_tex_len];
            pool_cursor.read_exact(&mut comp_tex).map_err(|e| e.to_string())?;
            let decomp_tex = xpress::decompress(&comp_tex, decomp_tex_len)?;

            let comp_mesh_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let decomp_mesh_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let mut comp_mesh = vec![0u8; comp_mesh_len];
            pool_cursor.read_exact(&mut comp_mesh).map_err(|e| e.to_string())?;
            let decomp_mesh = xpress::decompress(&comp_mesh, decomp_mesh_len)?;

            let comp_face_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let decomp_face_len = pool_cursor.read_u32::<LittleEndian>().map_err(|e| e.to_string())? as usize;
            let mut comp_face = vec![0u8; comp_face_len];
            pool_cursor.read_exact(&mut comp_face).map_err(|e| e.to_string())?;
            let decomp_face = xpress::decompress(&comp_face, decomp_face_len)?;

            return Ok((decomp_tex, decomp_mesh, decomp_face));
        }
    }
    Err("POOL chunk not found".to_string())
}

fn main() -> io::Result<()> {
    let orig_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0.hod";
    let edited_path = "/run/media/system/Data/SteamLibrary/steamapps/common/Homeworld 347380/GBXTools/WorkshopTool/uncompressed_bigs/HWRM/pebble/pebble_0/pebble_0_edited.hod";

    println!("=== Comparing Pebble Original vs Edited HOD ===");

    let mut file1 = File::open(orig_path)?;
    let mut bytes1 = Vec::new();
    file1.read_to_end(&mut bytes1)?;

    let mut file2 = File::open(edited_path)?;
    let mut bytes2 = Vec::new();
    file2.read_to_end(&mut bytes2)?;

    println!("1. Comparing File Sizes:");
    println!("   Original size: {} bytes", bytes1.len());
    println!("   Edited size:   {} bytes", bytes2.len());

    let (orig_tex, orig_mesh, orig_face) = match decompress_pools(&bytes1) {
        Ok(pools) => pools,
        Err(e) => {
            println!("Error decompressing original pools: {}", e);
            return Ok(());
        }
    };

    let (edited_tex, edited_mesh, edited_face) = match decompress_pools(&bytes2) {
        Ok(pools) => pools,
        Err(e) => {
            println!("Error decompressing edited pools: {}", e);
            return Ok(());
        }
    };

    println!("\n1.5. Comparing Decompressed Texture Pools:");
    println!("   Original Texture Pool: {} bytes", orig_tex.len());
    println!("   Edited Texture Pool:   {} bytes", edited_tex.len());
    let mut tex_diffs = 0;
    for i in 0..orig_tex.len().min(edited_tex.len()) {
        if orig_tex[i] != edited_tex[i] {
            if tex_diffs < 10 {
                println!("   Texture pool mismatch at index {}: original={:02X}, edited={:02X}", i, orig_tex[i], edited_tex[i]);
            }
            tex_diffs += 1;
        }
    }
    if orig_tex.len() != edited_tex.len() {
        println!("   [FAIL] Texture pool size difference!");
    } else if tex_diffs == 0 {
        println!("   => Texture Pools are 100% identical!");
    } else {
        println!("   [FAIL] Texture Pools have {} mismatching bytes!", tex_diffs);
    }

    println!("\n2. Comparing Decompressed Mesh Pools:");
    println!("   Original Mesh Pool: {} bytes", orig_mesh.len());
    println!("   Edited Mesh Pool:   {} bytes", edited_mesh.len());
    let mut mesh_diffs = 0;
    for i in 0..orig_mesh.len().min(edited_mesh.len()) {
        if orig_mesh[i] != edited_mesh[i] {
            if mesh_diffs < 10 {
                println!("   Mesh pool mismatch at index {}: original={:02X}, edited={:02X}", i, orig_mesh[i], edited_mesh[i]);
            }
            mesh_diffs += 1;
        }
    }
    if orig_mesh.len() != edited_mesh.len() {
        println!("   [FAIL] Mesh pool size difference!");
    } else if mesh_diffs == 0 {
        println!("   => Mesh Pools are 100% identical!");
    } else {
        println!("   [FAIL] Mesh Pools have {} mismatching bytes!", mesh_diffs);
    }

    println!("\n3. Comparing Decompressed Face Pools:");
    println!("   Original Face Pool: {} bytes", orig_face.len());
    println!("   Edited Face Pool:   {} bytes", edited_face.len());
    let mut face_diffs = 0;
    for i in 0..orig_face.len().min(edited_face.len()) {
        if orig_face[i] != edited_face[i] {
            if face_diffs < 10 {
                println!("   Face pool mismatch at index {}: original={:02X}, edited={:02X}", i, orig_face[i], edited_face[i]);
            }
            face_diffs += 1;
        }
    }
    if orig_face.len() != edited_face.len() {
        println!("   [FAIL] Face pool size difference!");
    } else if face_diffs == 0 {
        println!("   => Face Pools are 100% identical!");
    } else {
        println!("   [FAIL] Face Pools have {} mismatching bytes!", face_diffs);
    }

    let model1 = HODModel::parse_with_external(&bytes1, Some(orig_path), None).unwrap();
    let model2 = HODModel::parse_with_external(&bytes2, Some(edited_path), None).unwrap();

    println!("\n4. Checking Mesh BMSH Headers:");
    for m in &model1.meshes {
        println!("   Original Mesh Name: '{}', Lod: {}", m.name, m.lod);
    }
    for m in &model2.meshes {
        println!("   Edited Mesh Name:   '{}', Lod: {}", m.name, m.lod);
    }

    Ok(())
}