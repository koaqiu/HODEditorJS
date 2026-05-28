use base64::{engine::general_purpose, Engine as _};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use std::io::Cursor;

fn main() {
    let img_data = vec![137, 80, 78, 71, 13, 10, 26, 10]; // PNG header
    let encoded = general_purpose::STANDARD.encode(&img_data);
    let decoded = general_purpose::STANDARD.decode(&encoded).unwrap();
    println!("Image crate check {} bytes", decoded.len());
}
