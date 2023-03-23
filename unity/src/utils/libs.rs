use crate::libs::{NativeLibrary, LibError, NativeMethod};

pub fn get_function_option<T>(lib: &NativeLibrary, name: &str) -> Result<Option<NativeMethod<T>>, LibError> {
    let res = lib.sym(name);
    match res.is_err() {
        true => Ok(None),
        false => Ok(Some(res.unwrap())),
    }
}