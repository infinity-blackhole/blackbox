use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Go up from crates/octo-proto/ to workspace root
    let workspace_root = manifest_dir
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists() && p.join("proto").exists())
        .unwrap_or(&manifest_dir)
        .to_path_buf();

    let proto_file = workspace_root.join("proto").join("octo.proto");
    let proto_dir = workspace_root.join("proto");

    prost_build::Config::new()
        .compile_protos(
            &[&proto_file],
            &[&proto_dir],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    println!("cargo:rerun-if-changed=proto/octo.proto");
}
