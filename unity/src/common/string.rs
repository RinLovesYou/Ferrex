//! TODO

use std::ffi::c_void;
use crate::{
    runtime::{
        RuntimeError, Runtime
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
    pub fn from_string(runtime: Box<dyn Runtime>, from: impl Into<String>) -> Result<UnityString, RuntimeError> {
        runtime.new_string(from.into())
    }

    pub fn from_raw(runtime: Box<dyn Runtime>, from: *const i8) -> Result<UnityString, RuntimeError> {
        runtime.string_from_raw(from)
    }
}
