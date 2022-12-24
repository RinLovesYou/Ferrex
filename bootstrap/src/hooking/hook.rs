use dobby_rs::Address;

use crate::errors::hookerr::HookError;

pub fn attach(target: Address, detour: Address) -> Result<Address, HookError> {
    if target.is_null() {
        return Err(HookError::Nullpointer("target".to_string()));
    }

    if detour.is_null() {
        return Err(HookError::Nullpointer("detour".to_string()));
    }

    unsafe {
        let detour = dobby_rs::hook(target, detour)?;

        if detour.is_null() {
            return Err(HookError::Null);
        }

        Ok(detour)
    }
}

pub fn detach(target: Address) -> Result<(), HookError> {
    if target.is_null() {
        return Err(HookError::Nullpointer("target".to_string()));
    }

    unsafe {
        dobby_rs::unhook(target)?;
    }

    Ok(())
}