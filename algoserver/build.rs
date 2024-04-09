use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./src/proto/algomancer.proto";

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());


    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("algomancer_descriptor.bin"))
        .build_server(true)
        .compile(&[proto_file], &["."])?;

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
