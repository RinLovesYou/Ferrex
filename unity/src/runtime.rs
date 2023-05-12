//! TODO

use std::{
    error,
    fmt::{self, Display},
    io,
    path::PathBuf,
    str::Utf8Error,
};

use libc::c_void;
use thiserror::Error;

use crate::{
    common::{
        assembly::UnityAssembly,
        domain::UnityDomain,
        method::{MethodPointer, UnityMethod},
        object::UnityObject,
        string::UnityString,
        thread::UnityThread, class::UnityClass, image::UnityImage, property::UnityProperty,
    },
    il2cpp::Il2Cpp,
    libs::{self},
    mono::{AssemblyHookType, Mono},
    utils,
};

pub type FerrexRuntime = Box<dyn Runtime + Send + Sync>;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Fmt(#[from] std::fmt::Error),
    #[error(transparent)]
    Std(#[from] Box<dyn error::Error>),
    #[error(transparent)]
    Lib(#[from] libs::LibError),
    #[error(transparent)]
    Nul(#[from] std::ffi::NulError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error("Not a unity process")]
    NotUnity,
    #[error("Failed to find Base Path")]
    BasePathNotFound,
    #[error("Data Path not found!")]
    DataPathNotFound,
    #[error("Failed to find mono library path")]
    MonoLibPath,
    #[error("Failed to get mono lib name")]
    MonoLibName,
    #[error("Missing version argument in mono_jit_init_version")]
    JitInitVersionArgMissing,
    #[error("Function '{0}' not found")]
    MissingFunction(&'static str),
    #[error("Function Returned Null at {0}")]
    ReturnedNull(&'static str),
    #[error("Failed to get Game Assembly")]
    GameAssemblyNotFound,
    #[error("Failed to initialize Runtime")]
    FailedToInitRuntime,
    #[error("Failed to create C-String")]
    FailedToCreateCString,
    #[error("{0}")]
    Passthrough(String),
    #[error("String may not be empty!")]
    EmptyString,
    #[error("Argument {0} is a null pointer!")]
    NullPointer(&'static str),
    #[error("Not Implemented: {0}")]
    NotImplemented(&'static str),
}

#[derive(Debug)]
pub enum RuntimeType<'a> {
    Mono(&'a Mono),
    Il2Cpp(&'a Il2Cpp),
}

impl Display for RuntimeType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RuntimeType::Il2Cpp(_) => write!(f, "Il2cpp"),
            RuntimeType::Mono(mono) => match mono.is_old {
                true => write!(f, "Mono"),
                false => write!(f, "MonoBleedingEdge"),
            },
        }
    }
}

pub trait Runtime {
    fn get_type(&self) -> RuntimeType<'_>;
    fn get_domain(&self) -> Result<UnityDomain, RuntimeError>;
    fn get_current_thread(&self) -> Result<UnityThread, RuntimeError>;
    fn set_main_thread(&self, thread: &UnityThread) -> Result<(), RuntimeError>;
    fn attach_to_thread(&self, thread: &UnityDomain) -> Result<UnityThread, RuntimeError>;
    fn add_internal_call(&self, name: &str, func: MethodPointer) -> Result<(), RuntimeError>;
    fn install_assembly_hook(
        &self,
        hook_type: AssemblyHookType,
        func: MethodPointer,
    ) -> Result<(), RuntimeError>;
    fn create_debug_domain(&self, domain: &UnityDomain) -> Result<(), RuntimeError>;
    fn get_export_ptr(&self, name: &str) -> Result<MethodPointer, RuntimeError>;
    fn set_domain_config(
        &self,
        domain: &UnityDomain,
        dir: &str,
        name: &str,
    ) -> Result<(), RuntimeError>;
    fn new_string(&self, name: &str) -> Result<UnityString, RuntimeError>;
    fn string_from_raw(&self, name: *const i8) -> Result<UnityString, RuntimeError>;
    fn invoke_method(
        &self,
        method: &UnityMethod,
        obj: Option<&UnityObject>,
        params: Option<&mut Vec<*mut c_void>>,
    ) -> Result<Option<UnityObject>, RuntimeError>;
    fn get_method_name(&self, method: &UnityMethod) -> Result<String, RuntimeError>;
    fn get_assemblies(&self) -> Result<Vec<UnityAssembly>, RuntimeError>;
    fn get_assembly_name(&self, assembly: &UnityAssembly) -> Result<String, RuntimeError>;
    fn open_assembly(&self, name: &str) -> Result<UnityAssembly, RuntimeError>;
    fn assembly_get_image(&self, assembly: &UnityAssembly) -> Result<UnityImage, RuntimeError>;
    fn get_class(&self, assembly: &UnityAssembly, namespace: String, name: String) -> Result<UnityClass, RuntimeError>;
    fn get_class_name(&self, class: &UnityClass) -> Result<String, RuntimeError>;
    fn get_property(&self, class: &UnityClass, name: &str) -> Result<UnityProperty, RuntimeError>;
    fn get_property_name(&self, prop: &UnityProperty) -> Result<String, RuntimeError>;
    fn get_property_get_method(&self, prop: &UnityProperty) -> Result<UnityMethod, RuntimeError>;
    fn get_property_set_method(&self, prop: &UnityProperty) -> Result<UnityMethod, RuntimeError>;
    fn get_unmanaged_thunk(&self, method: &UnityMethod) -> Result<MethodPointer, RuntimeError>;
    fn get_method(&self, name: &str, args: i32, class: &UnityClass) -> Result<UnityMethod, RuntimeError>;
    fn get_assembly_object(&self, assembly: &UnityAssembly) -> Result<UnityObject, RuntimeError>;
    fn unbox_object(&self, object: &UnityObject) -> Result<UnityObject, RuntimeError>;
}

/// looks up the runtime
pub fn get_runtime() -> Result<FerrexRuntime, RuntimeError> {
    let exe_path = std::env::current_exe()?;
    if !is_unity(&exe_path)? {
        return Err(RuntimeError::NotUnity);
    }

    let base_path = exe_path
        .parent()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_path_buf();
    let data_path = utils::path::get_data_path(&exe_path)?;

    let mono = utils::path::find_mono(&base_path, &data_path);

    if let Ok(mono_path) = mono {
        let mono = Mono::new(mono_path)?;
        Ok(Box::new(mono) as Box<dyn Runtime + Send + Sync>)
    } else {
        let il2cpp = Il2Cpp::new(base_path)?;
        Ok(Box::new(il2cpp) as Box<dyn Runtime + Send + Sync>)
    }
}

fn is_unity(file_path: &PathBuf) -> Result<bool, RuntimeError> {
    let file_name = file_path
        .file_stem()
        .ok_or(RuntimeError::BasePathNotFound)?
        .to_str()
        .ok_or(RuntimeError::BasePathNotFound)?;

    let base_folder = file_path.parent().ok_or(RuntimeError::BasePathNotFound)?;

    let data_path = base_folder.join(format!("{}_Data", file_name));

    if !data_path.exists() {
        return Ok(false);
    }

    let global_game_managers = data_path.join("globalgamemanagers");
    let data_unity3d = data_path.join("data.unity3d");
    let main_data = data_path.join("mainData");

    if global_game_managers.exists() || data_unity3d.exists() || main_data.exists() {
        Ok(true)
    } else {
        Ok(false)
    }
}
