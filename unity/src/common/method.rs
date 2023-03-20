//! TODO

use std::ffi::c_void;

use crate::runtime::{RuntimeError, FerrexRuntime};

use super::object::UnityObject;

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
    pub fn new(pointer: MethodPointer) -> Result<Self, RuntimeError> {
        if pointer.is_null() {
            return Err(RuntimeError::NullPointer("pointer"));
        }

        Ok(UnityMethod { inner: pointer })
    }
    pub fn get_name(&self, runtime: &FerrexRuntime) -> Result<String, RuntimeError> {
        runtime.get_method_name(self)
    }

    pub fn invoke(&self, object: Option<&UnityObject>, params: Option<&mut Vec<*mut c_void>>, runtime: &FerrexRuntime) -> Result<Option<UnityObject>, RuntimeError> {
        runtime.invoke_method(self, object, params)
    }
}
