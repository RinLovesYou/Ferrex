use std::{mem::transmute, error::Error};

use dobby_rs::Address;
use unity_rs::{
    common::method::UnityMethod,
    runtime::{Runtime, RuntimeType},
};

use crate::{errors::hookerr::HookError, internal_failure, mods::{manager::ModManager}, log};

use super::hook;

type InvokeFn = fn(Address, Address, *mut Address, *mut Address) -> Address;

static mut INVOKE_ORIGINAL: Option<InvokeFn> = None;

pub fn hook_init() -> Result<(), HookError> {
    let runtime = unity_rs::runtime::get_runtime()?;

    log!("Running under {}", runtime.get_type())?;

    let runtime_invoke = match runtime.get_type() {
        RuntimeType::Mono(mono) => mono.get_export_ptr("mono_runtime_invoke")?,
        RuntimeType::Il2Cpp(il2cpp) => il2cpp.get_export_ptr("il2cpp_runtime_invoke")?,
    };

    if runtime_invoke.is_null() {
        return Err(HookError::Nullpointer("could not find runtime_invoke".to_string()));
    }

    log!("Attaching Hook to runtime_invoke")?;

    unsafe {
        INVOKE_ORIGINAL = Some(transmute(hook::attach(
            runtime_invoke,
            invoke_detour as Address,
        )?));
    }

    Ok(())
}

fn invoke_detour(
    method: Address,
    object: Address,
    params: *mut Address,
    exception: *mut Address,
) -> Address {
    invoke_detour_inner(method, object, params, exception).unwrap_or_else(|e| {
        internal_failure!("Failed to run invoke_detour: {}", e.to_string());
    })
}

fn invoke_detour_inner(
    method: Address,
    object: Address,
    params: *mut Address,
    exception: *mut Address,
) -> Result<Address, Box<dyn Error>> {
    let ret = unsafe {
        let original =
            INVOKE_ORIGINAL.ok_or(HookError::NoTrampoline("runtime_invoke".to_string()))?;

        original(method, object, params, exception)
    };

    let runtime = unity_rs::runtime::get_runtime()?;

    let unity_method = UnityMethod {
        inner: method.cast(),
    };

    let name = unity_method.get_name(&runtime)?;

    let is_old_mono = match runtime.get_type() {
        RuntimeType::Mono(mono) => mono.is_old,
        RuntimeType::Il2Cpp(_) => false,
    };

    let should_run = (name.contains("Internal_ActiveSceneChanged")
        || name.contains("UnityEngine.ISerializationCallbackReceiver.OnAfterSerialize"))
        || (is_old_mono && (name.contains("Awake") || name.contains("DoSendMouseEvents")));

    if !should_run {
        return Ok(ret);
    }

    unsafe {
        let runtime_invoke = match runtime.get_type() {
            RuntimeType::Mono(mono) => mono.get_export_ptr("mono_runtime_invoke")?,
            RuntimeType::Il2Cpp(il2cpp) => il2cpp.get_export_ptr("il2cpp_runtime_invoke")?,
        };

        hook::detach(transmute(runtime_invoke))?;

        let m = ModManager::new()?;
    }

    Ok(ret)
}
