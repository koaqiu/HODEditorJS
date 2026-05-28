use hwr_hod_parser::iff::IffChunk;
use std::env;
use std::fs::File;
use std::io::Read;

fn print_chunk_tree(chunk: &IffChunk, indent: usize) {
    let padding = "  ".repeat(indent);
    println!("{} - {} (Size: {})", padding, chunk.id, chunk.data.len());
    for child in &chunk.children {
        print_chunk_tree(child, indent + 1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_chunks <file.hod>");
        return;
    }

    let file_path = &args[1];
    println!("Reading {}...", file_path);

    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return;
        }
    };

    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        println!("Failed to read file: {}", e);
        return;
    }

    let mut cursor = std::io::Cursor::new(buffer);

    while cursor.position() < cursor.get_ref().len() as u64 {
        match IffChunk::read_chunk(&mut cursor) {
            Ok(chunk) => {
                print_chunk_tree(&chunk, 0);
            }
            Err(e) => {
                println!("Failed to read root chunk: {}", e);
                break;
            }
        }
    }
}
