//! TODO

use std::ffi::*;

/// the appdomain
#[derive(Debug)]
#[repr(C)]
pub struct MonoDomain {}

/// a thread
#[derive(Debug)]
#[repr(C)]
pub struct MonoThread {}

/// a method
#[derive(Debug)]
#[repr(C)]
pub struct MonoMethod {}

/// a class
#[derive(Debug)]
#[repr(C)]
pub struct MonoClass {}

/// an assembly
#[derive(Debug)]
#[repr(C)]
pub struct MonoAssembly {}

/// a mono image
#[derive(Debug)]
#[repr(C)]
pub struct MonoImage {}
/// a mono string
#[derive(Debug)]
#[repr(C)]
pub struct MonoString {}

/// a mono object
#[derive(Debug)]
#[repr(C)]
pub struct MonoObject {
    /// the vtable
    pub vtable: *mut c_void,
    /// the sync
    pub syncchronisation: *mut c_void,
}

/// a reflection assembly
#[derive(Debug)]
#[repr(C)]
pub struct MonoReflectionAssembly {
    /// the object
    pub object: MonoObject,
    /// the assembly
    pub assembly: *mut MonoAssembly,
    /// evidence
    pub evidence: *mut MonoObject,
}

/// an assembly name
#[derive(Debug)]
#[repr(C)]
pub struct AssemblyName {
    pub name: *mut c_char,
    pub culture: *mut c_char,
    pub hash_value: *mut c_char,
    pub public_key: *mut c_char,

    pub public_key_token: [c_char; 17],

    pub hash_alg: u32,
    pub hash_len: u32,

    pub flags: u32,
    pub major: c_ushort,
    pub minor: c_ushort,
    pub build: c_ushort,
    pub revision: c_ushort,
    pub arch: u32,
}
