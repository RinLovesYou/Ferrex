use std::error::Error;

use scotch_host::host_function;
use unity_rs::common::domain::UnityDomain;

use crate::{err, core};

/* #region Get the current Domain */

#[host_function]
pub fn fx_get_domain() -> UnityDomain {
    get_domain().unwrap_or_else(|e| {
        let _ = err!("Failed to execute fx_get_domain: {}", e.to_string());
        UnityDomain {
            inner: std::ptr::null_mut()
        }
    })
}

fn get_domain() -> Result<UnityDomain, Box<dyn Error>> {
    let runtime = core::get_runtime()?;

    Ok(runtime.get_domain()?)
}

/* #endregion */