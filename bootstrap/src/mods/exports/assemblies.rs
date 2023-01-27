use std::error::Error;

use scotch_host::host_function;
use unity_rs::common::assembly::UnityAssembly;

use crate::{core, err};

/* #region Get Assemblies as Vector */

#[host_function]
pub fn fx_get_assemblies() -> Vec<UnityAssembly> {
    get_assemblies().unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_assemblies: {}", e.to_string());
        Vec::new()
    })
}

fn get_assemblies() -> Result<Vec<UnityAssembly>, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(runtime.get_assemblies()?)
}

/* #endregion */

/* #region Get Assembly Name */

#[host_function]
pub fn fx_get_assembly_name(assembly: &UnityAssembly) -> String {
    get_assembly_name(assembly).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_assembly_name: {}", e.to_string());
        "".to_string()
    })
}

fn get_assembly_name(assembly: &UnityAssembly) -> Result<String, Box<dyn Error>> {
    Ok(
        UnityAssembly::get_name(assembly, core::get_runtime()?)?
    )
}

/* #endregion */

/* #region Get Assembly by name */

#[host_function]
pub fn fx_get_assembly(name: &String) -> UnityAssembly {
    get_assembly(name).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_assembly: {}", e.to_string());
        UnityAssembly { 
            inner: std::ptr::null_mut() 
        }
    })
}

fn get_assembly(name: &String) -> Result<UnityAssembly, Box<dyn Error>> {
    Ok(
        core::get_runtime()?.open_assembly(name.to_string())?
    )
}

/* #endregion */
