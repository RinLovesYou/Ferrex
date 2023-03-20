//! TODO

use std::ffi::c_void;

use crate::runtime::{RuntimeError, FerrexRuntime};

use super::{property::UnityProperty, method::UnityMethod};

/// Represents a C# Class
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityClass {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityClass {}
unsafe impl Sync for UnityClass {}

impl Clone for UnityClass {
    fn clone(&self) -> UnityClass {
        UnityClass { ..*self }
    }
}

impl UnityClass {
    pub fn get_name(&self, runtime: &FerrexRuntime) -> Result<String, RuntimeError> {
        runtime.get_class_name(self)
    }

    pub fn get_property(&self, name: &str, runtime: &FerrexRuntime) -> Result<UnityProperty, RuntimeError> {
        runtime.get_property(self, name)
    }

    pub fn get_method(&self, name: &str, args: i32, runtime: &FerrexRuntime) -> Result<UnityMethod, RuntimeError> {
        runtime.get_method(name, args, self)
    }
}
