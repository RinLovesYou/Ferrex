use std::{error::Error, path::PathBuf};

use unity_rs::common::assembly::UnityAssembly;

use crate::core;

pub fn run(asm: &UnityAssembly) -> Result<(), Box<dyn Error>> {
    let runtime = core::get_runtime()?;
    let path = PathBuf::from("Ferrex").join("Bindings");
    
    let name = asm.get_name(runtime)?.to_lowercase().replace(".", "_").replace("-", "_");
    let folder = path.join(&name);

    let mod_path = folder.join("mod.rs");

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&mod_path)?;

    

    Ok(())
}