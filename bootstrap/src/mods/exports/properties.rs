use std::{error::Error, ptr::addr_of_mut, ffi::c_void};

use scotch_host::host_function;
use unity_rs::common::{class::UnityClass, property::UnityProperty, object::UnityObject};

use crate::{core, err, mods::common::ArbitraryData};

/* #region Get a Property by name */

#[host_function]
pub fn fx_get_property(class: &UnityClass, name: &String) -> UnityProperty {
    get_property(class, name).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_property: {}", e.to_string());
        UnityProperty { inner: std::ptr::null_mut() }
    })
}

fn get_property(class: &UnityClass, name: &str) -> Result<UnityProperty, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(class.get_property(runtime, name)?)
}

/* #endregion */

/* #region Get a Property by name */

#[host_function]
pub fn fx_get_property_name(prop: &UnityProperty) -> String {
    get_property_name(prop).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_property_name: {}", e.to_string());
        "".to_string()
    })
}

fn get_property_name(prop: &UnityProperty) -> Result<String, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(prop.get_name(runtime)?)
}

/* #endregion */

/* #region Get a Property's value */

#[host_function]
pub fn fx_get_property_value(prop: &UnityProperty, object: &UnityObject) -> UnityObject {
    get_property_value(prop, object).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_property_value: {}", e.to_string());
        UnityObject { inner: std::ptr::null_mut() }
    })
}

fn get_property_value(prop: &UnityProperty, object: &UnityObject) -> Result<UnityObject, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    let obj = match object.inner as usize {
        0 => None,
        _ => Some(object)
    };

    let res = prop.get(runtime, obj)?;

    match res.is_none() {
        true => Err("Prop returned null".into()),
        false => Ok(res.unwrap())
    }
}


/* #endregion */

/* #region Set a Property's value */

#[host_function]
pub fn fx_set_property_value(prop: &UnityProperty, object: &UnityObject, value: &ArbitraryData) {
    set_property_value(prop, object, &mut value.to_owned()).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_property_value: {}", e.to_string());
    })
}

fn set_property_value(prop: &UnityProperty, object: &UnityObject, value: &mut ArbitraryData) -> Result<(), Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    let obj = match object.inner as usize {
        0 => None,
        _ => Some(object)
    };

    let args = match value.data.len() {
        0 => std::ptr::null_mut(),
        _ => {
            value.data[0].as_mut_ptr().cast()
        }
    };

    prop.set(runtime, obj, args)?;

    Ok(())
}

/* #endregion */