use std::io::Result;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    // Create generated directory if it doesn't exist
    let generated_dir = Path::new("src/generated");
    if !generated_dir.exists() {
        fs::create_dir_all(generated_dir)?;
    }

    // Compile the original schema for the main program
    prost_build::compile_protos(&["proto/person.proto"], &["proto"])?;
    
    // Compile the evolved schema with an extern path to map it to a different module
    let mut evolved_config = prost_build::Config::new();
    evolved_config.out_dir("src/generated");
    
    // Use extern_path to map the test package to the evolved module
    evolved_config.extern_path(".test", "crate::test_data::evolved");
    evolved_config.compile_protos(&["proto/person_evolved.proto"], &["proto"])?;
    
    // Tell cargo to rerun this build script if proto files change
    println!("cargo:rerun-if-changed=proto/person.proto");
    println!("cargo:rerun-if-changed=proto/person_evolved.proto");
    
    Ok(())
}