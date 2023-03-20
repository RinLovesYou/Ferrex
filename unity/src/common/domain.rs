//! TODO

use std::ffi::c_void;

/// Represents a C# Domain
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityDomain {
    /// The inner pointer to the Domain
    pub inner: *mut c_void,
}

impl UnityDomain {
    pub fn new(inner: *mut c_void) -> UnityDomain {
        UnityDomain { inner }
    }
}

unsafe impl Send for UnityDomain {}
unsafe impl Sync for UnityDomain {}

impl Clone for UnityDomain {
    fn clone(&self) -> UnityDomain {
        UnityDomain { ..*self }
    }
}