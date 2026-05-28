use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom};

fn read_len_string(cursor: &mut Cursor<&[u8]>) -> std::io::Result<String> {
    let len = cursor.read_u32::<LittleEndian>()? as usize;
    let mut buf = vec![0u8; len];
    cursor.read_exact(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("scar_dump_0.bin")?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    let mut cursor = Cursor::new(bytes.as_slice());

    let _name = read_len_string(&mut cursor)?;
    let _parent = read_len_string(&mut cursor)?;
    let _lod_count = cursor.read_u32::<LittleEndian>()?;

    for _ in 0..6 {
        cursor.read_f32::<LittleEndian>()?;
    } // bbox
    for _ in 0..4 {
        cursor.read_f32::<LittleEndian>()?;
    } // bsph

    let index_count = cursor.read_u32::<LittleEndian>()?;
    let vertex_count = cursor.read_u32::<LittleEndian>()?;
    let v_offset = cursor.read_u32::<LittleEndian>()?;
    let n_offset = cursor.read_u32::<LittleEndian>()?;

    println!(
        "Indices: {}, Vertices: {}, V-Off: {}, N-Off: {}",
        index_count, vertex_count, v_offset, n_offset
    );

    // Skip indices (index_count * 3 * 2 bytes)
    // Wait, is Val1 the triangle count or index count?
    // Let's assume Val1 is triangle count:
    let tri_count = index_count;
    cursor.seek(SeekFrom::Current((tri_count * 3 * 2) as i64))?;

    println!("Offset after indices: 0x{:X}", cursor.position());
    println!(
        "Remaining bytes: {}",
        cursor.get_ref().len() as u64 - cursor.position()
    );

    println!("Dumping next 64 u32s:");
    for _ in 0..64 {
        if let Ok(val) = cursor.read_u32::<LittleEndian>() {
            print!("{:08X} ", val);
        }
    }
    println!();

    Ok(())
}
