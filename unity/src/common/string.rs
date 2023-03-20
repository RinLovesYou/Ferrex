//! TODO

use std::ffi::c_void;
use crate::{
    runtime::{
        RuntimeError, FerrexRuntime
    },
};

/// Represents a C# String
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityString {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityString {}
unsafe impl Sync for UnityString {}

impl Clone for UnityString {
    fn clone(&self) -> UnityString {
        UnityString { ..*self }
    }
}

impl UnityString {
    pub fn from_string(from: &str, runtime: &FerrexRuntime) -> Result<UnityString, RuntimeError> {
        runtime.new_string(from)
    }

    pub fn from_raw(from: *const i8, runtime: &FerrexRuntime) -> Result<UnityString, RuntimeError> {
        runtime.string_from_raw(from)
    }
}
