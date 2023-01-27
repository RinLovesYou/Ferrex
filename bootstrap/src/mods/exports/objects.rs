use std::error::Error;

use scotch_host::host_function;
use unity_rs::common::object::UnityObject;

use crate::{err};

/* #region Unbox an Object as i32 */

#[host_function]
pub fn fx_unbox_i32(object: &UnityObject) -> i32 {
    unbox_i32(object).unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_unbox_i32: {}", e.to_string());
        0
    })
}

fn unbox_i32(object: &UnityObject) -> Result<i32, Box<dyn Error>> {
    if object.inner.is_null() {
        return Err("Object is null".into());
    }

    unsafe {
        Ok(*object.inner.cast())
    }
}

/* #endregion */