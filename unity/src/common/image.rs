//! TODO

use std::ffi::c_void;

/// Represents a C# Image
#[derive(Debug, Copy)]
pub struct UnityImage {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityImage {}
unsafe impl Sync for UnityImage {}

impl Clone for UnityImage {
    fn clone(&self) -> UnityImage {
        UnityImage { ..*self }
    }
}

impl UnityImage {
    
}
