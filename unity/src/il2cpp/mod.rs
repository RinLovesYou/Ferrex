//! TODO

use std::{
    ffi::{CStr, CString},
    path::PathBuf,
    ptr::addr_of_mut,
};

use libc::c_void;

use crate::{
    common::{
        assembly::UnityAssembly,
        domain::UnityDomain,
        method::{MethodPointer, UnityMethod},
        object::UnityObject,
        string::UnityString,
        thread::UnityThread, image::UnityImage, class::UnityClass, property::UnityProperty,
    },
    join_dll_path,
    libs::{self, NativeLibrary, NativeMethod},
    mono::AssemblyHookType,
    runtime::{Runtime, RuntimeError, RuntimeType},
};

use self::{exports::Il2CppExports, types::Il2CppObject};

pub mod exports;
pub mod types;

#[derive(Debug, Clone)]
pub struct Il2Cpp {
    pub game_assembly: NativeLibrary,
    pub exports: Il2CppExports,
}

unsafe impl Send for Il2Cpp {}
unsafe impl Sync for Il2Cpp {}

impl Il2Cpp {
    pub fn new(base_path: PathBuf) -> Result<Self, RuntimeError> {
        let game_assembly_path = join_dll_path!(base_path, "GameAssembly");

        if !game_assembly_path.exists() {
            return Err(RuntimeError::GameAssemblyNotFound);
        }

        let lib = libs::load_lib(&game_assembly_path)?;

        let exports = Il2CppExports::new(&lib)?;

        let il2cpp = Il2Cpp {
            game_assembly: lib,
            exports,
        };
        Ok(il2cpp)
    }
}

impl Runtime for Il2Cpp {
    fn get_type(&self) -> RuntimeType<'_> {
        RuntimeType::Il2Cpp(self)
    }

    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError> {
        let function: NativeMethod<fn()> = self.game_assembly.sym(name)?;

        if function.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("get_export_ptr"));
        }

        Ok(function.inner)
    }

    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_thread_current
            .ok_or(RuntimeError::MissingFunction("il2cpp_thread_current"))?;
        let thread = function();

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_current"));
        }

        Ok(UnityThread {
            inner: thread.cast(),
        })
    }

    /// this function doesn't exist in il2cpp, it just forwards to il2cpp_thread_attach
    fn set_main_thread(&self, thread: &UnityThread) -> Result<(), RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_thread_attach
            .ok_or(RuntimeError::MissingFunction("il2cpp_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        let _ = function(thread.inner.cast());

        Ok(())
    }

    fn attach_to_thread(&self, thread: &UnityDomain) -> Result<UnityThread, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_thread_attach
            .ok_or(RuntimeError::MissingFunction("il2cpp_thread_attach"))?;

        if thread.inner.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        let thread = function(thread.inner.cast());

        if thread.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_thread_attach").into());
        }

        Ok(UnityThread {
            inner: thread.cast(),
        })
    }

    fn add_internal_call(&self, name: String, func: MethodPointer) -> Result<(), RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_add_internal_call
            .ok_or(RuntimeError::MissingFunction("il2cpp_add_internal_call"))?;

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

    fn install_assembly_hook(
        &self,
        _hook_type: AssemblyHookType,
        _func: MethodPointer,
    ) -> Result<(), RuntimeError> {
        return Err(RuntimeError::NotImplemented(
            "install_assembly_hook are mono only functions",
        ));
    }

    fn create_debug_domain(&self, _domain: &UnityDomain) -> Result<(), RuntimeError> {
        return Err(RuntimeError::NotImplemented(
            "create_debug_domain is a mono only function",
        ));
    }

    fn get_domain(&self) -> Result<UnityDomain, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_domain_get
            .ok_or(RuntimeError::MissingFunction("il2cpp_domain_get"))?;

        let domain = function();

        if domain.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_domain_get"));
        }

        Ok(UnityDomain {
            inner: domain.cast(),
        })
    }

    fn set_domain_config(
        &self,
        _domain: &UnityDomain,
        _dir: String,
        _name: String,
    ) -> Result<(), RuntimeError> {
        return Err(RuntimeError::NotImplemented(
            "set_domain_config is a mono only function",
        ));
    }

    fn new_string(&self, name: String) -> Result<UnityString, RuntimeError> {
        if name.is_empty() {
            return Err(RuntimeError::EmptyString);
        }

        let native_str = CString::new(name.as_str())?;

        self.string_from_raw(native_str.as_ptr())
    }

    fn string_from_raw(&self, name: *const i8) -> Result<UnityString, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_string_new
            .ok_or(RuntimeError::MissingFunction("il2cpp_string_new"))?;

        if name.is_null() {
            return Err(RuntimeError::NullPointer("name"));
        }

        let res = function(name);

        if res.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_string_new"));
        }

        Ok(UnityString { inner: res.cast() })
    }

    fn invoke_method(
        &self,
        method: &UnityMethod,
        obj: Option<&UnityObject>,
        params: Option<&mut Vec<*mut c_void>>,
    ) -> Result<Option<UnityObject>, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_runtime_invoke
            .ok_or(RuntimeError::MissingFunction("il2cpp_runtime_invoke"))?;

        if method.inner.is_null() {
            return Err(RuntimeError::NullPointer("method"));
        }

        let exc: *mut Il2CppObject = std::ptr::null_mut();
        let object = match obj {
            Some(obj) => obj.inner,
            None => std::ptr::null_mut(),
        };

        let params = match params {
            Some(params) => addr_of_mut!(params[0]),
            None => std::ptr::null_mut(),
        };

        let result = function(
            method.inner.cast(),
            object.cast(),
            params,
            exc as *mut *mut Il2CppObject,
        );

        match result.is_null() {
            true => Ok(Some(UnityObject {
                inner: result.cast(),
            })),
            false => Ok(None),
        }
    }

    fn get_method_name(&self, method: &UnityMethod) -> Result<String, RuntimeError> {
        let function = &self
            .exports
            .clone()
            .il2cpp_method_get_name
            .ok_or(RuntimeError::MissingFunction("il2cpp_method_get_name"))?;

        if method.inner.is_null() {
            return Err(RuntimeError::NullPointer("method"));
        }

        let name_c = function(method.inner.cast());

        if name_c.is_null() {
            return Err(RuntimeError::ReturnedNull("il2cpp_method_get_name"));
        }

        let name = unsafe { CStr::from_ptr(name_c) };

        Ok(name.to_str()?.to_string())
    }

    fn get_assemblies(&self) -> Result<Vec<UnityAssembly>, RuntimeError> {
        Ok(Vec::new())
    }

    fn get_assembly_name(&self, _assembly: &UnityAssembly) -> Result<String, RuntimeError> {
        Ok("stub".to_string())
    }

    fn open_assembly(&self, _name: String) -> Result<UnityAssembly, RuntimeError> {
        Err(RuntimeError::NotImplemented("open_assembly"))
    }

    fn assembly_get_image(&self, _assembly: &UnityAssembly) -> Result<UnityImage, RuntimeError> {
        Err(RuntimeError::NotImplemented("assembly_get_image"))
    }

    fn get_class(&self, _assembly: &UnityAssembly, _namespace: String, _name: String) -> Result<UnityClass, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_class"))
    }

    fn get_class_name(&self, _class: &UnityClass) -> Result<String, RuntimeError> {
        Ok("TODO".to_string())
    }

    fn get_property(&self, _class: &UnityClass, _name: &str) -> Result<UnityProperty, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_property"))
    }

    fn get_property_name(&self, _prop: &UnityProperty) -> Result<String, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_property_name"))
    }

    fn get_property_get_method(&self, _prop: &UnityProperty) -> Result<UnityMethod, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_property_get_get_method"))
    }

    fn get_property_set_method(&self, _prop: &UnityProperty) -> Result<UnityMethod, RuntimeError> {
        Err(RuntimeError::NotImplemented("get_property_get_set_method"))
    }

    fn get_unmanaged_thunk(&self, method: &UnityMethod) -> Result<MethodPointer, RuntimeError> {
        if method.inner.is_null() {
            return Err(RuntimeError::NullPointer("method"));
        }

        Ok(method.inner)
    }
}
