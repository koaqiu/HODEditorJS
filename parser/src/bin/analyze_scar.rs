use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Cursor, Read};

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

    let name = read_len_string(&mut cursor)?;
    println!("Mesh Name: {}", name);

    let parent = read_len_string(&mut cursor)?;
    println!("Parent Name: {}", parent);

    let lod_count = cursor.read_u32::<LittleEndian>()?;
    println!("LOD Count (maybe?): {}", lod_count);

    // Read 6 floats (BBOX)
    let min_x = cursor.read_f32::<LittleEndian>()?;
    let min_y = cursor.read_f32::<LittleEndian>()?;
    let min_z = cursor.read_f32::<LittleEndian>()?;
    let max_x = cursor.read_f32::<LittleEndian>()?;
    let max_y = cursor.read_f32::<LittleEndian>()?;
    let max_z = cursor.read_f32::<LittleEndian>()?;
    println!(
        "BBOX: Min({}, {}, {}) Max({}, {}, {})",
        min_x, min_y, min_z, max_x, max_y, max_z
    );

    // Read 4 floats (BSPH)
    let cx = cursor.read_f32::<LittleEndian>()?;
    let cy = cursor.read_f32::<LittleEndian>()?;
    let cz = cursor.read_f32::<LittleEndian>()?;
    let r = cursor.read_f32::<LittleEndian>()?;
    println!("BSPH: Center({}, {}, {}) Radius({})", cx, cy, cz, r);

    let val3 = cursor.read_u32::<LittleEndian>()?;
    let val4 = cursor.read_u32::<LittleEndian>()?;
    println!("Val3: {}, Val4: {}", val3, val4);

    println!("Offset now: 0x{:X}", cursor.position());

    println!("Dumping next 32 u32s in hex:");
    for _ in 0..32 {
        if let Ok(val) = cursor.read_u32::<LittleEndian>() {
            print!("{:08X} ", val);
        }
    }
    println!();

    Ok(())
}
