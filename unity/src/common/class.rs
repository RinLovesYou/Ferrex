//! TODO

use std::ffi::c_void;

use bincode::{Encode, Decode};

use crate::runtime::{Runtime, RuntimeError};

use super::property::UnityProperty;

/// Represents a C# Class
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityClass {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityClass {}
unsafe impl Sync for UnityClass {}

impl Clone for UnityClass {
    fn clone(&self) -> UnityClass {
        UnityClass { ..*self }
    }
}

impl Encode for UnityClass {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        (self.inner as i32).encode(encoder)
    }
}

impl Decode for UnityClass {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let ptr = i32::decode(decoder)?;

        Ok(UnityClass {
            inner: ptr as usize as *mut c_void
        })
    }
}

impl UnityClass {
    pub fn get_name(&self, runtime: &Box<dyn Runtime>) -> Result<String, RuntimeError> {
        runtime.get_class_name(self)
    }

    pub fn get_property(&self, runtime: &Box<dyn Runtime>, name: &str) -> Result<UnityProperty, RuntimeError> {
        runtime.get_property(self, name)
    }
}
