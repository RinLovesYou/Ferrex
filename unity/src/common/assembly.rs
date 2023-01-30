//! TODO

use std::ffi::c_void;

use bincode::{Encode, Decode};

use crate::runtime::{Runtime, RuntimeError};

use super::{image::UnityImage, class::UnityClass};

/// Represents a C# Assembly
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityAssembly {
    /// The inner pointer to the Tread
    pub inner: *mut c_void,
}

unsafe impl Send for UnityAssembly {}
unsafe impl Sync for UnityAssembly {}

impl Clone for UnityAssembly {
    fn clone(&self) -> UnityAssembly {
        UnityAssembly { ..*self }
    }
}

impl Encode for UnityAssembly {
    fn encode<E: bincode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), bincode::error::EncodeError> {
        (self.inner as i32).encode(encoder)
    }
}

impl Decode for UnityAssembly {
    fn decode<D: bincode::de::Decoder>(decoder: &mut D) -> Result<Self, bincode::error::DecodeError> {
        let ptr = i32::decode(decoder)?;

        Ok(UnityAssembly {
            inner: ptr as usize as *mut c_void
        })
    }
}

impl UnityAssembly {
    pub fn get_name(&self, runtime: &Box<dyn Runtime>) -> Result<String, RuntimeError> {
        runtime.get_assembly_name(self)
    }

    pub fn get_image(&self, runtime: &Box<dyn Runtime>) -> Result<UnityImage, RuntimeError> {
        runtime.assembly_get_image(self)
    }

    pub fn get_class(&self, runtime: &Box<dyn Runtime>, namespace: &str, name: &str) -> Result<UnityClass, RuntimeError> {
        runtime.get_class(self, namespace.to_string(), name.to_string())
    }
}