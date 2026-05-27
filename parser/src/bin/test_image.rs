use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::imageops::FilterType;
use base64::{Engine as _, engine::general_purpose};

fn main() {
    let img_data = vec![137, 80, 78, 71, 13, 10, 26, 10]; // PNG header
    let encoded = general_purpose::STANDARD.encode(&img_data);
    let decoded = general_purpose::STANDARD.decode(&encoded).unwrap();
    println!("Image crate check {} bytes", decoded.len());
}
