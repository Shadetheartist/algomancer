fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./src/proto/gameservice.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[proto_file], &["."])?;

    println!("cargo:rerun-if-changed={}", proto_file);

    Ok(())
}
