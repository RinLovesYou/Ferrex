//! TODO

use std::ffi::{c_void, OsStr};

use crate::runtime::{RuntimeError, FerrexRuntime};

/// Represents a C# Image
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityImage {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityImage {}
unsafe impl Sync for UnityImage {}

impl Clone for UnityImage {
    fn clone(&self) -> UnityImage {
        UnityImage { ..*self }
    }
}

impl UnityImage {
    pub fn open<P: AsRef<OsStr>>(filename: P, runtime: &FerrexRuntime) -> Result<UnityImage, RuntimeError> {
        let name = filename.as_ref().to_str().ok_or_else(|| RuntimeError::Passthrough("Failed to get string from path".to_string()))?;
        runtime.open_assembly(name)?.get_image(runtime)
    }
}
