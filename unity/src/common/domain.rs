//! TODO

use std::ffi::c_void;

use bincode::{Encode, Decode};

/// Represents a C# Domain
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityDomain {
    /// The inner pointer to the Domain
    pub inner: *mut c_void,
}

unsafe impl Send for UnityDomain {}
unsafe impl Sync for UnityDomain {}

impl Clone for UnityDomain {
    fn clone(&self) -> UnityDomain {
        UnityDomain { ..*self }
    }
}

impl Encode for UnityDomain {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        (self.inner as i32).encode(encoder)
    }
}

impl Decode for UnityDomain {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let ptr = i32::decode(decoder)?;

        Ok(UnityDomain {
            inner: ptr as usize as *mut c_void
        })
    }
}
