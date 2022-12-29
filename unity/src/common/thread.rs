//! TODO

use std::ffi::c_void;

/// Represents a C# Thread
#[derive(Debug)]
pub struct UnityThread {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityThread {}
unsafe impl Sync for UnityThread {}

impl Clone for UnityThread {
    fn clone(&self) -> UnityThread {
        UnityThread { ..*self }
    }
}
