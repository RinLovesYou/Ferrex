//! TODO

use std::ffi::c_void;

use crate::runtime::{Runtime, RuntimeError};

pub type MethodPointer = *mut c_void;

/// Represents a C# Method
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityMethod {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityMethod {}
unsafe impl Sync for UnityMethod {}

impl Clone for UnityMethod {
    fn clone(&self) -> UnityMethod {
        UnityMethod { ..*self }
    }
}

impl UnityMethod {
    pub fn get_name(&self, runtime: &Box<dyn Runtime>) -> Result<String, RuntimeError> {
        runtime.get_method_name(self)
    }
}
