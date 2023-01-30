//! TODO

use std::ffi::{c_char, c_int, c_void};

use crate::libs::{LibError, NativeLibrary, NativeMethod};

use super::types::{
    AssemblyName, MonoAssembly, MonoClass, MonoDomain, MonoImage, MonoMethod, MonoObject,
    MonoString, MonoThread, MonoProperty,
};

type GFunc = extern "C" fn(*mut MonoAssembly, *mut c_void);

#[derive(Debug, Clone)]
pub struct MonoExports {
    pub mono_jit_init_version:
        Option<NativeMethod<fn(*const c_char, *const c_char) -> *mut MonoDomain>>,
    pub mono_debug_domain_create: Option<NativeMethod<fn(*mut MonoDomain)>>,
    pub mono_thread_current: Option<NativeMethod<fn() -> *mut MonoThread>>,
    pub mono_thread_set_main: Option<NativeMethod<fn(*mut MonoThread)>>,
    pub mono_thread_attach: Option<NativeMethod<fn(*mut MonoDomain) -> *mut MonoThread>>,
    pub mono_domain_set_config:
        Option<NativeMethod<fn(*mut MonoDomain, *const c_char, *const c_char)>>,
    pub mono_add_internal_call: Option<NativeMethod<fn(*const c_char, *mut c_void)>>,
    pub mono_get_root_domain: Option<NativeMethod<fn() -> *mut MonoDomain>>,
    pub mono_string_new:
        Option<NativeMethod<fn(*mut MonoDomain, *const c_char) -> *mut MonoString>>,
    pub mono_domain_assembly_open:
        Option<NativeMethod<fn(*mut MonoDomain, *const c_char) -> *mut MonoAssembly>>,
    pub mono_assembly_get_image: Option<NativeMethod<fn(*mut MonoAssembly) -> *mut MonoImage>>,
    pub mono_class_from_name:
        Option<NativeMethod<fn(*mut MonoImage, *const c_char, *const c_char) -> *mut MonoClass>>,
    pub mono_class_get_method_from_name:
        Option<NativeMethod<fn(*mut MonoClass, *const c_char, c_int) -> *mut MonoMethod>>,
    pub mono_runtime_invoke: Option<
        NativeMethod<
            fn(
                *mut MonoMethod,
                *mut MonoObject,
                *mut *mut c_void,
                *mut *mut MonoObject,
            ) -> *mut MonoObject,
        >,
    >,
    pub mono_object_to_string:
        Option<NativeMethod<fn(*mut MonoObject, *mut *mut MonoObject) -> *mut MonoString>>,
    pub mono_string_to_utf8: Option<NativeMethod<fn(*mut MonoString) -> *const c_char>>,
    pub mono_method_get_name: Option<NativeMethod<fn(*mut MonoMethod) -> *const c_char>>,
    pub mono_install_assembly_preload_hook: Option<NativeMethod<fn(*mut c_void, *mut c_void)>>,
    pub mono_install_assembly_search_hook: Option<NativeMethod<fn(*mut c_void, *mut c_void)>>,
    pub mono_install_assembly_load_hook: Option<NativeMethod<fn(*mut c_void, *mut c_void)>>,
    pub mono_assembly_get_object:
        Option<NativeMethod<fn(*mut MonoDomain, *mut MonoAssembly) -> *mut MonoObject>>,
    pub mono_assembly_foreach: Option<NativeMethod<fn(GFunc, *mut c_void)>>,
    pub mono_assembly_get_name: Option<NativeMethod<fn(*mut MonoAssembly) -> *mut AssemblyName>>,
    pub mono_class_get_name: Option<NativeMethod<fn(*mut MonoClass) -> *const c_char>>,
    pub mono_class_get_property_from_name: Option<NativeMethod<fn(*mut MonoClass, *const c_char) -> *mut MonoProperty>>,
    pub mono_property_get_name: Option<NativeMethod<fn(*mut MonoProperty) -> *const c_char>>,
    pub mono_property_get_get_method: Option<NativeMethod<fn(*mut MonoProperty) -> *mut MonoMethod>>,
    pub mono_property_get_set_method: Option<NativeMethod<fn(*mut MonoProperty) -> *mut MonoMethod>>,
    pub mono_method_get_unmanaged_thunk: Option<NativeMethod<fn(*mut MonoMethod) -> *mut c_void>>,
}

impl MonoExports {
    pub fn new(lib: &NativeLibrary) -> Result<Self, LibError> {
        Ok(MonoExports {
            mono_jit_init_version: Some(lib.sym("mono_jit_init_version")?),
            mono_debug_domain_create: {
                // probably not present on old mono
                let res = lib.sym("mono_debug_domain_create");
                match res.is_err() {
                    true => None,
                    false => Some(res.unwrap()),
                }
            },
            mono_string_new: Some(lib.sym("mono_string_new")?),
            mono_runtime_invoke: Some(lib.sym("mono_runtime_invoke")?),
            mono_string_to_utf8: Some(lib.sym("mono_string_to_utf8")?),
            mono_thread_current: Some(lib.sym("mono_thread_current")?),
            mono_thread_attach: Some(lib.sym("mono_thread_attach")?),
            mono_get_root_domain: Some(lib.sym("mono_get_root_domain")?),
            mono_class_from_name: Some(lib.sym("mono_class_from_name")?),
            mono_method_get_name: Some(lib.sym("mono_method_get_name")?),
            mono_thread_set_main: Some(lib.sym("mono_thread_set_main")?),
            mono_object_to_string: {
                // probably not present on old mono
                let res = lib.sym("mono_object_to_string");

                match res.is_err() {
                    true => None,
                    false => Some(res.unwrap()),
                }
            },
            mono_add_internal_call: Some(lib.sym("mono_add_internal_call")?),
            mono_domain_set_config: {
                // probably not present on old mono
                let res = lib.sym("mono_domain_set_config");

                match res.is_err() {
                    true => None,
                    false => Some(res.unwrap()),
                }
            },
            mono_assembly_get_image: Some(lib.sym("mono_assembly_get_image")?),
            mono_assembly_get_object: Some(lib.sym("mono_assembly_get_object")?),
            mono_domain_assembly_open: Some(lib.sym("mono_domain_assembly_open")?),
            mono_install_assembly_load_hook: Some(lib.sym("mono_install_assembly_load_hook")?),
            mono_class_get_method_from_name: Some(lib.sym("mono_class_get_method_from_name")?),
            mono_install_assembly_search_hook: Some(lib.sym("mono_install_assembly_search_hook")?),
            mono_install_assembly_preload_hook: Some(
                lib.sym("mono_install_assembly_preload_hook")?,
            ),
            mono_assembly_foreach: Some(lib.sym("mono_assembly_foreach")?),
            mono_assembly_get_name: Some(lib.sym("mono_assembly_get_name")?),
            mono_class_get_name: Some(lib.sym("mono_class_get_name")?),
            mono_class_get_property_from_name: Some(lib.sym("mono_class_get_property_from_name")?),
            mono_property_get_name: Some(lib.sym("mono_property_get_name")?),
            mono_property_get_get_method: Some(lib.sym("mono_property_get_get_method")?),
            mono_property_get_set_method: Some(lib.sym("mono_property_get_set_method")?),
            mono_method_get_unmanaged_thunk: Some(lib.sym("mono_method_get_unmanaged_thunk")?),
        })
    }
}
