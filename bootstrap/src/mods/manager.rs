use std::{error::Error, fs::{self}};

use unity_rs::runtime::Runtime;
use wasmer::{Store, Module, imports, Function, Instance};

use crate::log;

pub fn init(_runtime: Box<dyn Runtime>) -> Result<(), Box<dyn Error>> {
    log!("Initializing Wasi Mods")?;

    let mods_dir = std::env::current_dir()?.join("Ferrex").join("Mods");

    if !mods_dir.exists() {
        fs::create_dir(&mods_dir)?;
    }

    let path = fs::read("Ferrex/Mods/ferrex_plugin.wasm")?;

    let mut store = Store::default();
    let module = Module::new(&store, path)?;

    let function = Function::new_typed(
        &mut store,
        exposed_log
        
    );

    let imports = imports! { 
        "env" => {
            "log_ferrex" => function,
        },
    };

    #[allow(unused_variables)]
    let instance = Instance::new(&mut store, &module, &imports)?;

    let _ = instance.exports.get_function("entry")?.call(&mut store, &[]);

    Ok(())
}

fn exposed_log() {
    let _ = exposed_log_inner();
}

fn exposed_log_inner() -> Result<(), Box<dyn Error>> {
    log!("TEST")?;
    Ok(())
}