use std::{error::Error, ffi::c_char, mem::transmute};

use dobby_rs::Address;
use unity_rs::{
    il2cpp::types::Il2CppDomain,
    runtime::{Runtime, RuntimeType},
};

use crate::{core, errors::hookerr::HookError, internal_failure, log};

use super::hook;

type InitFn = fn(*mut c_char) -> *mut Il2CppDomain;

static mut INIT_ORIGINAL: Option<InitFn> = None;

pub fn hook_init() -> Result<(), HookError> {
    let runtime = unity_rs::runtime::get_runtime()?;

    log!("Running under {}", runtime.get_type())?;

    let init = match runtime.get_type() {
        RuntimeType::Mono(mono) => mono.get_export_ptr("mono_jit_init_version")?,
        RuntimeType::Il2Cpp(il2cpp) => il2cpp.get_export_ptr("il2cpp_init")?,
    };

    if init.is_null() {
        return Err(HookError::Nullpointer(
            "could not find runtime_invoke".to_string(),
        ));
    }

    log!("Attaching Hook to init")?;

    unsafe {
        INIT_ORIGINAL = Some(transmute(hook::attach(init, init_detour as Address)?));
    }

    Ok(())
}

fn init_detour(name: *mut c_char) -> *mut Il2CppDomain {
    init_detour_inner(name).unwrap_or_else(|e| {
        internal_failure!("Failed to run invoke_detour: {}", e.to_string());
    })
}

fn init_detour_inner(name: *mut c_char) -> Result<*mut Il2CppDomain, Box<dyn Error>> {
    let ret = unsafe {
        let original = INIT_ORIGINAL.ok_or(HookError::NoTrampoline("init".to_string()))?;

        original(name)
    };

    let runtime = core::get_runtime()?;

    log!("Init called!")?;

    unsafe {
        let init = match runtime.get_type() {
            RuntimeType::Mono(mono) => mono.get_export_ptr("mono_jit_init_version")?,
            RuntimeType::Il2Cpp(il2cpp) => il2cpp.get_export_ptr("il2cpp_init")?,
        };

        hook::detach(transmute(init))?;
    }

    Ok(ret)
}
