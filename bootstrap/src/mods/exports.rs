use wasmtime::{Caller, Extern};

use crate::{log, err};


pub fn log_str(caller: Caller<'_, ()>, ptr: i32, len: i32) {
    log_str_inner(caller, ptr, len).unwrap_or_else(|e| {
        let _ = err!("Failed to invoke log: {}", e.to_string());
    })
}

fn log_str_inner(mut caller: Caller<'_, ()>, ptr: i32, len: i32) -> Result<(), anyhow::Error> {
    let mem = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => anyhow::bail!("failed to find host memory"),
    };
    let data = mem.data(&caller)
        .get(ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize));
    let string = match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => anyhow::bail!("invalid utf-8"),
        },
        None => anyhow::bail!("pointer/length out of bounds"),
    };
    let _ = log!("{}", string);
    Ok(())
}