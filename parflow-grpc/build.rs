use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("parflow_descriptor.bin"))
        .out_dir(out_dir) // Generate to OUT_DIR where tonic::include_proto! expects them
        .compile(&["proto/parflow.proto"], &["proto/"])?;
        
    println!("cargo:rerun-if-changed=proto/");
    Ok(())
}
