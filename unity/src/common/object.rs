//! TODO

use std::ffi::c_void;

use bincode::{Encode, Decode};

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

impl Encode for UnityObject {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        (self.inner as i32).encode(encoder)
    }
}

impl Decode for UnityObject {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let ptr = i32::decode(decoder)?;

        Ok(UnityObject {
            inner: ptr as usize as *mut c_void
        })
    }
}