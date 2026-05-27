use std::env;
use std::fs::File;
use std::io::Read;
use hwr_hod_parser::hod::{HODModel, synthesize_engine_nozzles_v1, validate_marker_parents};

fn print_mappings(model: &HODModel, label: &str) {
    println!("--- {} Mappings ---", label);
    println!("Joints:");
    for j in &model.joints {
        println!("  - {} (parent: {})", j.name, j.parent_name.clone().unwrap_or_else(|| "None".to_string()));
    }
    println!("Navlights:");
    for n in &model.nav_lights {
        println!("  - {} (style: {}, section: {})", n.name, n.style, n.section);
    }
    println!("Engine Burns:");
    for b in &model.engine_burns {
        println!("  - {} (parent: {})", b.name, b.parent_name);
    }
    println!("Engine Glows:");
    for g in &model.engine_glows {
        println!("  - {} (parent: {})", g.name, g.parent_name);
    }
    println!("Animations:");
    for a in &model.animations {
        for t in &a.tracks {
            println!("  - Track for joint: {}", t.joint_name);
        }
    }
    println!("------------------------\n");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-hod>", args[0]);
        return;
    }

    let path = &args[1];
    println!("Parsing HOD: {}", path);

    let mut file = File::open(path).expect("Failed to open file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read file");

    let mut model = HODModel::parse(&bytes).expect("Failed to parse model");

    print_mappings(&model, "PRE-SYNTHESIS");

    println!("Applying synthesis and validation...");
    synthesize_engine_nozzles_v1(&mut model);
    validate_marker_parents(&mut model);

    print_mappings(&model, "POST-SYNTHESIS");
}