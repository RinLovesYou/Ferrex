//! TODO

use std::ffi::{c_char, c_int, c_void};

use crate::{libs::{LibError, NativeLibrary, NativeMethod}, utils::libs::get_function_option};

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
    pub mono_object_unbox: Option<NativeMethod<fn(*mut MonoObject) -> *mut c_void>>,
}

impl MonoExports {
    pub fn new(lib: &NativeLibrary) -> Result<Self, LibError> {
        Ok(MonoExports {
            mono_jit_init_version: get_function_option(&lib,  "mono_jit_init_version")?,
            mono_debug_domain_create: {
                // probably not present on old mono
                let res = lib.sym("mono_debug_domain_create");
                match res.is_err() {
                    true => None,
                    false => Some(res.unwrap()),
                }
            },
            mono_string_new: get_function_option(&lib,  "mono_string_new")?,
            mono_runtime_invoke: get_function_option(&lib,  "mono_runtime_invoke")?,
            mono_string_to_utf8: get_function_option(&lib,  "mono_string_to_utf8")?,
            mono_thread_current: get_function_option(&lib,  "mono_thread_current")?,
            mono_thread_attach: get_function_option(&lib,  "mono_thread_attach")?,
            mono_get_root_domain: get_function_option(&lib,  "mono_get_root_domain")?,
            mono_class_from_name: get_function_option(&lib,  "mono_class_from_name")?,
            mono_method_get_name: get_function_option(&lib,  "mono_method_get_name")?,
            mono_thread_set_main: get_function_option(&lib,  "mono_thread_set_main")?,
            mono_object_to_string: get_function_option(&lib,  "mono_object_to_string")?,
            mono_add_internal_call: get_function_option(&lib,  "mono_add_internal_call")?,
            mono_domain_set_config: get_function_option(&lib,  "mono_domain_set_config")?,
            mono_assembly_get_image: get_function_option(&lib,  "mono_assembly_get_image")?,
            mono_assembly_get_object: get_function_option(&lib,  "mono_assembly_get_object")?,
            mono_domain_assembly_open: get_function_option(&lib,  "mono_domain_assembly_open")?,
            mono_install_assembly_load_hook: get_function_option(&lib,  "mono_install_assembly_load_hook")?,
            mono_class_get_method_from_name: get_function_option(&lib,  "mono_class_get_method_from_name")?,
            mono_install_assembly_search_hook: get_function_option(&lib,  "mono_install_assembly_search_hook")?,
            mono_install_assembly_preload_hook: get_function_option(&lib,  "mono_install_assembly_preload_hook")?,
            mono_assembly_foreach: get_function_option(&lib,  "mono_assembly_foreach")?,
            mono_assembly_get_name: get_function_option(&lib,  "mono_assembly_get_name")?,
            mono_class_get_name: get_function_option(&lib,  "mono_class_get_name")?,
            mono_class_get_property_from_name: get_function_option(&lib,  "mono_class_get_property_from_name")?,
            mono_property_get_name: get_function_option(&lib,  "mono_property_get_name")?,
            mono_property_get_get_method: get_function_option(&lib,  "mono_property_get_get_method")?,
            mono_property_get_set_method: get_function_option(&lib,  "mono_property_get_set_method")?,
            mono_method_get_unmanaged_thunk: get_function_option(&lib,  "mono_method_get_unmanaged_thunk")?,
            mono_object_unbox: get_function_option(&lib,  "mono_object_unbox")?,
        })
    }
}
