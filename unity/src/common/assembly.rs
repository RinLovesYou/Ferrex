//! TODO

use std::ffi::{c_void, OsStr};

use crate::runtime::{FerrexRuntime, RuntimeError};

use super::{class::UnityClass, image::UnityImage};

/// Represents a C# Assembly
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityAssembly {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityAssembly {}
unsafe impl Sync for UnityAssembly {}

impl Clone for UnityAssembly {
    fn clone(&self) -> UnityAssembly {
        UnityAssembly { ..*self }
    }
}

impl UnityAssembly {
    pub fn new(pointer: *mut c_void) -> Result<Self, RuntimeError> {
        if pointer.is_null() {
            return Err(RuntimeError::NullPointer("pointer"));
        }

        Ok(UnityAssembly { inner: pointer })
    }
    
    pub fn open<P: AsRef<OsStr>>(
        filename: P,
        runtime: &FerrexRuntime,
    ) -> Result<UnityAssembly, RuntimeError> {
        let name = filename
            .as_ref()
            .to_str()
            .ok_or_else(|| RuntimeError::Passthrough("Failed to get string from path".to_string()))?;
        runtime.open_assembly(name)
    }

    pub fn get_name(&self, runtime: &FerrexRuntime) -> Result<String, RuntimeError> {
        runtime.get_assembly_name(self)
    }

    pub fn get_image(&self, runtime: &FerrexRuntime) -> Result<UnityImage, RuntimeError> {
        runtime.assembly_get_image(self)
    }

    pub fn get_class(
        &self,
        namespace: &str,
        name: &str,
        runtime: &FerrexRuntime,
    ) -> Result<UnityClass, RuntimeError> {
        runtime.get_class(self, namespace.to_string(), name.to_string())
    }
}
