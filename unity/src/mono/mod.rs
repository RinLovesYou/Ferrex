//! TODO

use std::{error, path::PathBuf, ptr::{addr_of, addr_of_mut}, fmt::{Display, self}, ffi::{CString, c_void, CStr}};

use thiserror::Error;

use crate::{
    common::{thread::UnityThread, domain::UnityDomain, string::UnityString, method::{MethodPointer, UnityMethod}, object::UnityObject, assembly::UnityAssembly},
    libs::{self, NativeLibrary, NativeMethod}, runtime::{Runtime, RuntimeError, RuntimeType},
};

use self::{exports::MonoExports, types::{MonoThread, MonoDomain, MonoObject, MonoAssembly}};

pub mod exports;
pub mod types;

/// assembly hook types
#[derive(Debug, Clone, Copy)]
pub enum AssemblyHookType {
    /// called when an assembly is loaded
    Preload,
    /// called when an assembly is unloaded
    Load,
    /// called when an assembly is searched
    Search,
}

impl Display for AssemblyHookType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssemblyHookType::Preload => write!(f, "preload"),
            AssemblyHookType::Load => write!(f, "load"),
            AssemblyHookType::Search => write!(f, "search"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mono {
    pub is_old: bool,
    pub mono_lib: NativeLibrary,
    pub exports: MonoExports,
}

impl Mono {
    pub fn new(mono_path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        if !mono_path.exists() {
            return Err(Box::new(RuntimeError::MonoLibPath));
        }

        let lib_name = mono_path
            .file_stem()
            .ok_or(RuntimeError::MonoLibName)?
            .to_str()
            .ok_or(RuntimeError::MonoLibName)?;

        let is_old = lib_name == "mono" || lib_name == "libmono";

        let mono_lib = libs::load_lib(&mono_path)?;

        let exports = MonoExports::new(&mono_lib)?;

        let mono = Mono {
            is_old,
            mono_lib,
            exports,
        };

        Ok(mono)
    }
}

impl Runtime for Mono {
    fn get_type(&self) -> RuntimeType {
        RuntimeType::Mono(self)
    }

    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError> {
        let function: NativeMethod<fn()> = self.mono_lib.sym(name)?;

        if function.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("get_export_ptr"));
        }

        Ok(function.inner)
    }

    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().mono_thread_current.ok_or(RuntimeError::MissingFunction("mono_thread_current"))?;
        let thread = function();

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_current").into());
        }

        Ok(UnityThread {
            inner: thread.cast(),
        })
    }

    fn set_main_thread(&self, thread: UnityThread) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_thread_set_main.ok_or(RuntimeError::MissingFunction("mono_thread_set_main"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_set_main"));
        }

        function(thread.inner.cast());
        Ok(())
    }

    fn attach_to_thread(&self, thread: UnityDomain) -> Result<UnityThread, RuntimeError> {
        let function = &self.exports.clone().mono_thread_attach.ok_or(RuntimeError::MissingFunction("mono_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_attach"));
        }

        let result = function(thread.inner.cast());

        if result.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_thread_attach"));
        }

        Ok(UnityThread {
            inner: result.cast(),
        })
    }

    fn add_internal_call(&self, name: String, func: MethodPointer) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_add_internal_call.ok_or(RuntimeError::MissingFunction("mono_add_internal_call"))?;

        if name.is_empty() {
            return Err(RuntimeError::EmptyString);
        }

        if func.is_null() {
            return Err(RuntimeError::NullPointer("func"));
        }

        let name = CString::new(name.as_str())?;

        function(name.as_ptr(), func);

        Ok(())
    }

    fn install_assembly_hook(&self, hook_type: AssemblyHookType, func: MethodPointer) -> Result<(), RuntimeError> {
        if func.is_null() {
            return Err(RuntimeError::NullPointer("func"));
        }

        let hook_func = match hook_type {
            AssemblyHookType::Preload => self.exports.clone().mono_install_assembly_preload_hook,
            AssemblyHookType::Load => self.exports.clone().mono_install_assembly_load_hook,
            AssemblyHookType::Search => self.exports.clone().mono_install_assembly_search_hook,
        }.ok_or(RuntimeError::MissingFunction("mono_install_assembly_hook"))?;

        hook_func(func, std::ptr::null_mut());

        Ok(())
    }

    fn get_domain(&self) -> Result<UnityDomain, RuntimeError> {
        let function = &self.exports.clone().mono_get_root_domain.ok_or(RuntimeError::MissingFunction("mono_get_root_domain"))?;

        let domain = function();

        if domain.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_get_root_domain"));
        }

        Ok(UnityDomain {
            inner: domain.cast(),
        })
    }

    fn create_debug_domain(&self, domain: UnityDomain) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_debug_domain_create.ok_or(RuntimeError::MissingFunction("mono_debug_domain_create"))?;

        if domain.inner.is_null() {
            return Err(RuntimeError::NullPointer("domain"));
        }

        function(domain.inner.cast());

        Ok(())
    }

    fn set_domain_config(&self, domain: UnityDomain, dir: String, name: String) -> Result<(), RuntimeError> {
        let function = &self.exports.clone().mono_domain_set_config.ok_or(RuntimeError::MissingFunction("mono_domain_set_config"))?;

        if domain.inner.is_null() {
            return Err(RuntimeError::NullPointer("domain"));
        }

        let dir = CString::new(dir.as_str())?;
        let name = CString::new(name.as_str())?;

        function(domain.inner.cast(), dir.as_ptr(), name.as_ptr());

        Ok(())
    }

    fn new_string(&self, name: String) -> Result<UnityString, RuntimeError> {
        if name.is_empty() {
            return Err(RuntimeError::EmptyString);
        }

        let native_str = CString::new(name)?;

        self.string_from_raw(native_str.as_ptr())
    }

    fn string_from_raw(&self, name: *const i8) -> Result<UnityString, RuntimeError> {
        let function = &self.exports.clone().mono_string_new.ok_or(RuntimeError::MissingFunction("mono_string_new"))?;

        if name.is_null() {
            return Err(RuntimeError::NullPointer("name"));
        }

        let res = function(self.get_domain()?.inner.cast(), name);

        if res.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_string_new"))
        }

        Ok(UnityString {
            inner: res.cast()
        })
    }

    fn invoke_method(&self, method: UnityMethod, obj: Option<UnityObject>, params: Option<&mut Vec<*mut c_void>>) -> Result<Option<UnityObject>, RuntimeError> {
        let function = &self.exports.clone().mono_runtime_invoke.ok_or(RuntimeError::MissingFunction("mono_runtime_invoke"))?;

        if method.inner.is_null() {
            return Err(RuntimeError::NullPointer("method"))
        }

        let exc: *mut MonoObject = std::ptr::null_mut();
        let object = match obj {
            Some(obj) => obj.inner,
            None => std::ptr::null_mut(),
        };

        let params = match params {
            Some(params) => addr_of_mut!(params[0]),
            None => std::ptr::null_mut(),
        };

        let result = function(method.inner.cast(), object.cast(), params, exc as *mut *mut MonoObject);

        match result.is_null() {
            true => Ok(Some(UnityObject {
                inner: result.cast(),
            })),
            false => Ok(None)
        }
    }

    fn get_method_name(&self, method: UnityMethod) -> Result<String, RuntimeError> {
        let function = &self.exports.clone().mono_method_get_name.ok_or(RuntimeError::MissingFunction("mono_method_get_name"))?;

        if method.inner.is_null() {
            return Err(RuntimeError::NullPointer("method"))
        }

        let name_c = function(method.inner.cast());

        if name_c.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_method_get_name"))
        }

        let name = unsafe { 
            CStr::from_ptr(name_c)
        };

        Ok(name.to_str()?.to_string())
    }

    fn get_assemblies(&self) -> Result<Vec<UnityAssembly>, RuntimeError> {
        let function = &self.exports.clone().mono_assembly_foreach.ok_or(RuntimeError::MissingFunction("mono_assembly_foreach"))?;

        let mut assemblies: Vec<UnityAssembly> = Vec::new();

        function(enumerate_assemblies, &mut assemblies as *mut _ as *mut c_void);

        Ok(assemblies)
    }

    fn get_assembly_name(&self, assembly: UnityAssembly) -> Result<String, RuntimeError> {
        let function = &self.exports.clone().mono_assembly_get_name.ok_or(RuntimeError::MissingFunction("mono_assembly_get_name"))?;

        if assembly.inner.is_null() {
            return Err(RuntimeError::NullPointer("assembly"));
        }

        let name = function(assembly.inner.cast());

        if name.is_null() {
            return Err(RuntimeError::ReturnedNull("mono_assembly_get_name"));
        }

        let name = unsafe {
            CStr::from_ptr((*name).name.cast())
        }.to_str()?;

        Ok(name.to_string())
    }
}

extern "C" fn enumerate_assemblies(assembly: *mut MonoAssembly, data: *mut c_void) {
    unsafe {
        if assembly.is_null() || data.is_null() {
            return;
        }

        (*data.cast::<Vec<UnityAssembly>>()).push(UnityAssembly { inner: assembly.cast() });
    }
}