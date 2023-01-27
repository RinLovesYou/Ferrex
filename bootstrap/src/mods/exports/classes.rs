use std::error::Error;

use scotch_host::host_function;
use unity_rs::common::{assembly::UnityAssembly, class::UnityClass};

use crate::{err, core};

/* #region Get a class by name */

#[host_function]
pub fn fx_get_class(assembly: &UnityAssembly, namespace: &String, name: &String) -> UnityClass {
    get_class(assembly, namespace, name).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_class: {}", e.to_string());
        UnityClass { inner: std::ptr::null_mut() }
    })
}

fn get_class(assembly: &UnityAssembly, namespace: &String, name: &String) -> Result<UnityClass, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(runtime.get_class(assembly, namespace.to_string(), name.to_string())?)
}

/* #endregion */

/* #region Get the name of a class */

#[host_function]
pub fn fx_get_class_name(class: &UnityClass) -> String {
    get_class_name(class).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_class_name: {}", e.to_string());
        "".to_string()
    })
}

fn get_class_name(class: &UnityClass) -> Result<String, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(class.get_name(runtime)?)
}

/* #endregion */