//! TODO

use std::ffi::c_void;

/// Represents a C# Appdomain
#[derive(Debug)]
pub struct UnityDomain {
    /// The inner pointer to the Appdomain
    pub inner: *mut c_void,
}

unsafe impl Send for UnityDomain {}
unsafe impl Sync for UnityDomain {}

impl Clone for UnityDomain {
    fn clone(&self) -> UnityDomain {
        UnityDomain { ..*self }
    }
}
