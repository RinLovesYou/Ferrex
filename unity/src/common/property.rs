//! TODO

use std::{ffi::c_void, error::Error};

use crate::runtime::{Runtime, RuntimeError};

use super::object::UnityObject;

/// Represents a C# Property
#[derive(Debug, Copy)]
#[repr(C)]
pub struct UnityProperty {
    /// The inner pointer to the Property
    pub inner: *mut c_void,
}

unsafe impl Send for UnityProperty {}
unsafe impl Sync for UnityProperty {}

impl Clone for UnityProperty {
    fn clone(&self) -> UnityProperty {
        UnityProperty { ..*self }
    }
}

impl UnityProperty {
    pub fn get_name(&self, runtime: &Box<dyn Runtime>) -> Result<String, RuntimeError> {
        runtime.get_property_name(self)
    }

    pub fn set(&self, runtime: &Box<dyn Runtime>, object: Option<&UnityObject>, value: *mut c_void) -> Result<(), Box<dyn Error>> {
        let method = runtime.get_property_set_method(self)?;

        let mut args = Vec::new();
        args.push(value);

        let _ = runtime.invoke_method(&method, object, Some(&mut args))?;

        Ok(())
    }

    pub fn get(&self, runtime: &Box<dyn Runtime>, object: Option<&UnityObject>) -> Result<Option<UnityObject>, Box<dyn Error>> {
        let method = runtime.get_property_get_method(self)?;

        Ok(runtime.invoke_method(&method, object, None)?)
    }
}
