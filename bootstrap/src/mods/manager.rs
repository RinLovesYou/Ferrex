use std::{
    error::Error,
    fs::{self, DirEntry},
    path::{Path},
};

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::add_to_linker;

use crate::{err, log};

use super::exports::{self};

#[allow(dead_code)]
pub struct FerrexMod {
    module: Module,
    instance: Instance,
}

#[allow(dead_code)]
pub struct ModManager {
    engine: Engine,
    store: Store<()>,
    mods: Vec<FerrexMod>,
}

impl ModManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        log!("Initializing Wasi Mods")?;

        let mods_dir = std::env::current_dir()?.join("Ferrex").join("Mods");

        if !mods_dir.exists() {
            fs::create_dir(&mods_dir)?;
        }
        let engine = Engine::default();
        let mut store = Store::new(&engine, ());
        let mut mods: Vec<FerrexMod> = Vec::new();
        let mut linker = Linker::new(&engine);

        let get_assembly_count = Func::wrap(&mut store, exports::get_assembly_count);
        let log_str = Func::wrap(&mut store, exports::log_str);
        let get_assemblies = Func::wrap(&mut store, exports::get_assemblies);
        let get_assembly_name = Func::wrap(&mut store, exports::get_assembly_name);

        linker.define("env", "fx_log_str", log_str)?;
        linker.define("env", "fx_get_assembly_count", get_assembly_count)?;
        linker.define("env", "fx_get_assemblies", get_assemblies)?;
        linker.define("env", "fx_get_assembly_name", get_assembly_name)?;

        let directory = fs::read_dir(mods_dir)?;

        let wasm_files: Vec<DirEntry> = directory
            .filter_map(Result::ok)
            .filter_map(|d| {
                d.path()
                    .to_str()
                    .and_then(|f| if f.ends_with(".wasm") { Some(d) } else { None })
            })
            .collect();

        for file in wasm_files {
            log!("Loading mod from: {}", file.path().display())?;
            let fmod = FerrexMod::new(file.path(), &engine, &mut linker, &mut store);
            if fmod.is_err() {
                err!("Failed to load mod: {}", fmod.err().unwrap().to_string())?;
                continue;
            }

            mods.push(fmod?);
        }

        for fmod in mods.iter_mut() {
            fmod.init(&mut store)?;
        }

        Ok(ModManager {
            engine,
            store,
            mods,
        })
    }
}

impl FerrexMod {
    fn new<P: AsRef<Path>>(
        path: P,
        engine: &Engine,
        linker: &mut Linker<()>,
        mut store: impl AsContextMut<Data = ()>,
    ) -> Result<Self, Box<dyn Error>> {
        let module = Module::from_file(engine, path)?;

        let instance = linker.instantiate(&mut store, &module)?;

        let ferrex_mod = FerrexMod { module, instance };

        Ok(ferrex_mod)
    }

    fn init(&mut self, mut store: impl AsContextMut) -> Result<(), Box<dyn Error>> {
        let foo = self
            .instance
            .get_func(&mut store, "init")
            .expect("Mod doesn't export a init function!");

        // ... or we can make a static assertion about its signature and call it.
        // Our first call here can fail if the signatures don't match, and then the
        // second call can fail if the function traps (like the `match` above).
        let foo = foo.typed::<(), ()>(&mut store)?;
        foo.call(&mut store, ())?;

        Ok(())
    }
}
