use std::path::PathBuf;

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let proto_path = PathBuf::from(project_dir)
        .join("proto")
        .join("authentication.proto");

    tonic_build::compile_protos(proto_path)?;

    Ok(())
}
