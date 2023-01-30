use std::{
    error::Error,
    fs::{self, DirEntry}, env::consts::DLL_EXTENSION,
};

use libloading::Library;

use crate::{log, bindgen};

pub struct ModManager {
    pub plugins: Vec<Library>
}

impl ModManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        log!("Running Bindgen")?;

        bindgen::generator::run()?;

        log!("Initializing Wasi Mods")?;

        let mods_dir = std::env::current_dir()?.join("Ferrex").join("Mods");

        if !mods_dir.exists() {
            fs::create_dir(&mods_dir)?;
        }

        let directory = fs::read_dir(mods_dir)?;

        let wasm_files: Vec<DirEntry> = directory
            .filter_map(Result::ok)
            .filter_map(|d| {
                d.path()
                    .to_str()
                    .and_then(|f| if f.ends_with(DLL_EXTENSION) { Some(d) } else { None })
            })
            .collect();

        let mut plugins = Vec::new();

        for file in wasm_files {
            log!("Loading mod from: {}", file.path().display())?;
            let plugin = unsafe {
                Library::new(file.path())?
            };

            plugins.push(plugin);
        }

        Ok(ModManager {
            plugins
        })
    }
}
