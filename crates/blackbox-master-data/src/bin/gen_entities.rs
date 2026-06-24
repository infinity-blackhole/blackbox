//! `gen-entities` — regenerate entity source code from `schemas.json`.
//!
//! Usage: `cargo run -p blackbox-master-data --bin gen-entities -- [options]`
//!
//! Options:
//!   --schema <PATH>   Path to schemas.json (default: ../../schemas.json)
//!   --out-dir <PATH>   Directory to write generated files (default: src/generated)

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut schema_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../schemas.json");
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/generated");

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--schema" => {
                i += 1;
                schema_path = args.get(i).expect("--schema requires a value").into();
            }
            "--out-dir" => {
                i += 1;
                out_dir = args.get(i).expect("--out-dir requires a value").into();
            }
            other => {
                eprintln!("Unknown argument: {}", other);
                std::process::exit(1);
            }
        }
        i += 1;
    }

    if !schema_path.exists() {
        eprintln!("Error: schemas.json not found at {}", schema_path.display());
        std::process::exit(1);
    }

    println!("Reading schema from {}...", schema_path.display());
    let data = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", schema_path.display(), e);
            std::process::exit(1);
        });

    let schemas: blackbox_schema::Schemas = serde_json::from_str(&data)
        .unwrap_or_else(|e| {
            eprintln!("Error parsing schemas.json: {}", e);
            std::process::exit(1);
        });

    let ir = blackbox_schema::SchemaIr::from_schemas(schemas);
    println!("Parsed {} tables, {} enum types", ir.tables.len(), ir.enums.len());

    let files = blackbox_schema::generate_all(&ir);

    std::fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
        eprintln!("Error creating {}: {}", out_dir.display(), e);
        std::process::exit(1);
    });

    for (name, content) in &files {
        let path = out_dir.join(name);
        std::fs::write(&path, content).unwrap_or_else(|e| {
            eprintln!("Error writing {}: {}", path.display(), e);
            std::process::exit(1);
        });
        println!("  → {} ({} bytes)", path.display(), content.len());
    }

    println!("Done. Generated {} files.", files.len());
}
