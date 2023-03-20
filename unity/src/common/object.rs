//! TODO

use std::ffi::c_void;

/// Represents a C# Object
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityObject {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityObject {}
unsafe impl Sync for UnityObject {}

impl Clone for UnityObject {
    fn clone(&self) -> UnityObject {
        UnityObject { ..*self }
    }
}