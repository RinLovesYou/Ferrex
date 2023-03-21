use std::{error::Error};


use unity_rs::runtime::{Runtime, self, FerrexRuntime};

use crate::{hooking, log, logging::logger, errors::DynErr, console};

pub fn init() -> Result<(), Box<dyn Error>> {
    console::init()?;
    logger::init()?;

    if !check_unity()? {
        return Ok(());
    }


    log!("Initializing Ferrex")?;

    //hooking::init::hook_init()?;
    hooking::invoke::hook_invoke()?;
    console::null_handles()?;

    Ok(())
}

#[allow(dead_code)]
static mut RUNTIME: Option<FerrexRuntime> = None;

pub fn get_runtime() -> Result<&'static FerrexRuntime, DynErr> {
    unsafe {
        if RUNTIME.is_none() {
            RUNTIME = Some(runtime::get_runtime()?)
        }

        Ok(RUNTIME.as_ref().ok_or("Failed to get runtime")?)
    }
}

fn check_unity() -> Result<bool, Box<dyn Error>> {
    let file_path = std::env::current_exe()?;

    let file_name = file_path
        .file_stem()
        .ok_or("failed to get file name")?
        .to_str()
        .ok_or("failed to get file name")?;

    let base_folder = file_path.parent().ok_or("failed to get base folder")?;

    let data_path = base_folder.join(format!("{}_Data", file_name));

    if !data_path.exists() {
        return Ok(false);
    }

    let global_game_managers = data_path.join("globalgamemanagers");
    let data_unity3d = data_path.join("data.unity3d");
    let main_data = data_path.join("mainData");

    if global_game_managers.exists() || data_unity3d.exists() || main_data.exists() {
        Ok(true)
    } else {
        Ok(false)
    }
}
