use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

use crate::core;

pub fn run() -> Result<(), Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    //let class = runtime.open_assembly("UnityEngine.CoreModule".to_string())?.get_class(runtime, "UnityEngine", "Application")?;

    let path = PathBuf::from("Ferrex").join("Bindings");

    if path.exists() {
        fs::remove_dir_all(&path)?;
    }

    fs::create_dir(&path)?;

    let cargo_toml_path = path.join("Cargo.toml");

    let content = "# This file has been generated by Ferrex
[package]
name = \"bindings\"
version = \"0.1.0\"
edition = \"2021\"
    
# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    
[dependencies]
unity-rs = { path = \"../unity\" }";

    fs::write(cargo_toml_path, content)?;

    let path = path.join("src");
    fs::create_dir(&path)?;

    let lib_path = path.join("lib.rs");

    let mut assemblies = runtime.get_assemblies()?;

    assemblies.sort_by(|a, b| {
        a.get_name(runtime)
            .unwrap()
            .to_lowercase()
            .cmp(&b.get_name(runtime).unwrap().to_lowercase())
    });

    for asm in assemblies.iter() {
        let name = asm
            .get_name(runtime)?
            .to_lowercase()
            .replace(".", "_")
            .replace("-", "_");
        let folder = path.join(&name);

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&lib_path)?;

        file.write_all(format!("pub mod {};\n", name).as_bytes())?;

        fs::create_dir(folder)?;
    }

    Ok(())
}
