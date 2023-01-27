use std::{
    error::Error,
    fs::{self, DirEntry},
};

use scotch_host::{WasmPlugin, make_imports, make_exports, guest_functions};

use crate::log;

use super::exports;

#[guest_functions]
extern "C" {
    // The name must match with the name of the plugin function.
    pub fn init();
}

pub struct ModManager {
    pub plugins: Vec<WasmPlugin>
}

impl ModManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
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
                    .and_then(|f| if f.ends_with(".wasm") { Some(d) } else { None })
            })
            .collect();

        let mut plugins = Vec::new();

        for file in wasm_files {
            log!("Loading mod from: {}", file.path().display())?;
            let file = fs::read(file.path())?;
            let plugin = WasmPlugin::builder()
                .with_state(())
                .from_binary(&file)?
                .with_imports(make_imports![
                    exports::logger::fx_log_str,

                    exports::domain::fx_get_domain, 

                    exports::assemblies::fx_get_assemblies,
                    exports::assemblies::fx_get_assembly,
                    exports::assemblies::fx_get_assembly_name,

                    exports::classes::fx_get_class,
                    exports::classes::fx_get_class_name,

                    exports::properties::fx_get_property,
                    exports::properties::fx_get_property_name,
                    exports::properties::fx_get_property_value,
                    exports::properties::fx_set_property_value,

                    exports::objects::fx_unbox_i32,
                ])
                .with_exports(make_exports![init])
                .finish()?;

            plugin.function_unwrap::<init>()()?;

            plugins.push(plugin);
        }

        Ok(ModManager {
            plugins
        })
    }
}
