use std::env;
use std::fs;
use std::path::Path;

use hwr_hod_parser::hod::{HODMaterial, HODModel};
use serde::Serialize;

const USAGE: &str = "Usage: hod_semantic_dump <file.hod>\n       or set HOD_ZEPHYRUS_FIXTURE=/path/to/file.hod";

#[derive(Serialize)]
struct SemanticDump<'a> {
    source_path: &'a str,
    model_name: &'a str,
    version: u32,
    is_v2: bool,
    materials: Vec<MaterialDump<'a>>,
    dockpaths: Vec<DockpathDump<'a>>,
}

#[derive(Serialize)]
struct MaterialDump<'a> {
    index: usize,
    name: &'a str,
    shader: &'a str,
    texture_maps: &'a [String],
    texture_slots: Vec<TextureSlotDump<'a>>,
}

#[derive(Serialize)]
struct TextureSlotDump<'a> {
    index: usize,
    texture_name: &'a str,
    role: &'static str,
    param_name: Option<&'a str>,
    role_source: &'static str,
}

#[derive(Serialize)]
struct DockpathDump<'a> {
    index: usize,
    name: &'a str,
    parent: &'a str,
    point_count: usize,
    val1: u32,
    val2: u32,
    val3: u32,
    val4: u32,
    val5: u32,
    compatible_ships: &'a str,
    padding1: u32,
    padding2: u32,
    points: Vec<DockpointDump>,
}

#[derive(Serialize)]
struct DockpointDump {
    index: usize,
    position: [f32; 3],
    rotation: [[f32; 4]; 4],
    tolerance: f32,
    max_speed: f32,
    extra1: u32,
    extra2: u32,
}

fn main() {
    let Some(path) = hod_path_from_args_or_env() else {
        println!("{}", USAGE);
        return;
    };

    let path_ref = Path::new(&path);
    let bytes = match fs::read(path_ref) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to read HOD '{}': {}", path, err);
            std::process::exit(1);
        }
    };

    let model = match HODModel::parse_with_external(&bytes, Some(&path), None) {
        Ok(model) => model,
        Err(err) => {
            eprintln!("Failed to parse HOD '{}': {}", path, err);
            std::process::exit(1);
        }
    };

    let dump = build_dump(&path, &model);
    println!("BEGIN_HOD_SEMANTIC_DUMP_JSON");
    match serde_json::to_string_pretty(&dump) {
        Ok(json) => println!("{}", json),
        Err(err) => {
            eprintln!("Failed to serialize semantic dump: {}", err);
            std::process::exit(1);
        }
    }
    println!("END_HOD_SEMANTIC_DUMP_JSON");
}

fn hod_path_from_args_or_env() -> Option<String> {
    env::args()
        .nth(1)
        .or_else(|| env::var("HOD_ZEPHYRUS_FIXTURE").ok())
        .filter(|path| !path.trim().is_empty())
}

fn build_dump<'a>(source_path: &'a str, model: &'a HODModel) -> SemanticDump<'a> {
    SemanticDump {
        source_path,
        model_name: &model.name,
        version: model.version,
        is_v2: model.is_v2,
        materials: model
            .materials
            .iter()
            .enumerate()
            .map(|(index, material)| build_material_dump(index, material))
            .collect(),
        dockpaths: model
            .dockpaths
            .iter()
            .enumerate()
            .map(|(index, dockpath)| DockpathDump {
                index,
                name: &dockpath.name,
                parent: &dockpath.parent_name,
                point_count: dockpath.points.len(),
                val1: dockpath.val1,
                val2: dockpath.val2,
                val3: dockpath.val3,
                val4: dockpath.val4,
                val5: dockpath.val5,
                compatible_ships: &dockpath.compatible_ships,
                padding1: dockpath.padding1,
                padding2: dockpath.padding2,
                points: dockpath
                    .points
                    .iter()
                    .enumerate()
                    .map(|(point_index, point)| DockpointDump {
                        index: point_index,
                        position: [point.position.x, point.position.y, point.position.z],
                        rotation: point.rotation.m,
                        tolerance: point.tolerance,
                        max_speed: point.max_speed,
                        extra1: point.extra1,
                        extra2: point.extra2,
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn build_material_dump(index: usize, material: &HODMaterial) -> MaterialDump<'_> {
    MaterialDump {
        index,
        name: &material.name,
        shader: &material.shader_name,
        texture_maps: &material.texture_maps,
        texture_slots: material
            .texture_maps
            .iter()
            .enumerate()
            .map(|(texture_index, texture_name)| TextureSlotDump {
                index: texture_index,
                texture_name,
                role: infer_texture_role(texture_name),
                param_name: None,
                role_source: "texture_name_suffix",
            })
            .collect(),
    }
}

fn infer_texture_role(texture_name: &str) -> &'static str {
    let uppercase = texture_name.to_ascii_uppercase();
    let stem = uppercase
        .strip_suffix(".TGA")
        .or_else(|| uppercase.strip_suffix(".DDS"))
        .unwrap_or(&uppercase);

    for (suffix, role) in [
        ("_DIFF", "DIFF"),
        ("_TEAM", "TEAM"),
        ("_GLOW", "GLOW"),
        ("_NORM", "NORM"),
        ("_SPEC", "SPEC"),
        ("_REFL", "REFL"),
        ("_STRP", "STRP"),
        ("_PAIN", "PAIN"),
    ] {
        if stem.ends_with(suffix) {
            return role;
        }
    }

    "UNKNOWN"
}
