//! TODO

use std::ffi::c_void;

use crate::runtime::{Runtime, RuntimeError};

/// Represents a C# Assembly
#[derive(Debug, Copy)]
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
    pub fn get_name(&self, runtime: &Box<dyn Runtime>) -> Result<String, RuntimeError> {
        runtime.get_assembly_name(self.to_owned())
    }
}
