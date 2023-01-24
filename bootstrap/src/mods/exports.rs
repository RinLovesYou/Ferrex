use std::{error::{self, Error}, ffi::c_void};

use unity_rs::{runtime::{self}, common::assembly::UnityAssembly};
use wasmtime::Caller;

use crate::{err, log, core};

#[no_mangle]
pub fn log_str(caller: Caller<'_, ()>, ptr: i32, len: i32) {
    log_str_inner(caller, ptr, len).unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
    })
}

pub fn get_assemblies(caller: Caller<'_, ()>, ptr: i32, len: i32) {
    get_assemblies_inner(caller, ptr, len).unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
    });
}

#[no_mangle]
pub fn get_assembly_count() -> i32 {
    get_assembly_count_inner().unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
        0
    })
}

pub fn get_assembly_name(caller: Caller<'_, ()>, assembly: i32, name: i32) -> i32 {
    get_assembly_name_inner(caller, assembly, name).unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
        0
    })
}

fn get_assembly_name_inner(mut caller: Caller<'_, ()>, assembly: i32, name_ptr: i32) -> Result<i32, Box<dyn Error>> {
    let assembly = UnityAssembly {
        inner: assembly as usize as *mut c_void
    };

    let mut name = assembly.get_name(core::get_runtime()?)?;

    let mem = caller.get_export("memory");

    let memory = mem
        .ok_or("Failed to get Memory")?
        .into_memory()
        .ok_or("Failed to get Memory")?;
    unsafe {
        let _ = memory
            .data_mut(&mut caller)
            .get_mut(name_ptr as usize..(name_ptr + name.len() as i32) as usize)
            .ok_or("Failed to read Array")?
            .copy_from_slice(name.as_bytes_mut());
    }

    Ok(name.len() as i32)
}

fn get_assembly_count_inner() -> Result<i32, Box<dyn Error>> {
    Ok(runtime::get_runtime()?.get_assemblies()?.len() as i32)
}

fn get_assemblies_inner(mut caller: Caller<'_, ()>, ptr: i32, len: i32) -> Result<(), Box<dyn Error>> {
    if ptr == 0 {
        return Err("Failed to get array pointer".into());
    }

    let runtime = runtime::get_runtime()?;

    let mem = caller.get_export("memory");

    let memory = mem
        .ok_or("Failed to get Memory")?
        .into_memory()
        .ok_or("Failed to get Memory")?;

    let mut assemblies: Vec<i32> = runtime.get_assemblies()?.iter().map(|a| a.inner as usize as i32).collect();

    let array_data: &mut [u8] = bytemuck::cast_slice_mut(assemblies.as_mut_slice());

    let _ = memory
        .data_mut(&mut caller)
        .get_mut(ptr as usize..(ptr + (len*4)) as usize)
        .ok_or("Failed to read Array")?
        .copy_from_slice(array_data);

    Ok(())
}

fn log_str_inner(
    mut caller: Caller<'_, ()>,
    ptr: i32,
    len: i32,
) -> Result<(), Box<dyn error::Error>> {
    let mem = caller.get_export("memory");

    let memory = mem
        .ok_or("Failed to get Memory")?
        .into_memory()
        .ok_or("Failed to get Memory")?;

    let data = memory
        .data(&caller)
        .get(ptr as usize..)
        .and_then(|arr| arr.get(..len as usize))
        .ok_or("Failed to read String")?;

    let string = std::str::from_utf8(data)?;

    let _ = log!("{}", string);

    Ok(())
}
