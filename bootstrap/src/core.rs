use std::error::Error;

use crate::{hooking, logging::logger, log};

pub fn init() -> Result<(), Box<dyn Error>> {
    if !check_unity()? {
        return Ok(());
    }

    logger::init()?;
    
    log!("Initializing Ferrex")?;
    
    hooking::invoke::hook_init()?;

    Ok(())
}

fn check_unity() -> Result<bool, Box<dyn Error>> {
    let file_path = std::env::current_exe()?;

    let file_name = file_path.file_stem()
        .ok_or_else(|| "failed to get file name")?
        .to_str()
        .ok_or_else(|| "failed to get file name")?;


    let base_folder = file_path.parent()
        .ok_or_else(|| "failed to get base folder")?;


    let data_path = base_folder.join(format!("{}_Data", file_name));


    if !data_path.exists() {
        return Ok(false)
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