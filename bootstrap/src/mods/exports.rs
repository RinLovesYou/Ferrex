use std::error;

use wasmtime::Caller;

use crate::{err, log};

pub fn log_str(caller: Caller<'_, ()>, ptr: i32, len: i32) {
    log_str_inner(caller, ptr, len).unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
    })
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
