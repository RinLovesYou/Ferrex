use std::{path::PathBuf, error};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum PathError {
   #[error("Failed to find Mono")]
    MonoNotFound,
    #[error("Failed to find DataPath")]
    DataPathNotFound,
}

/// joins a path with a file name, and appends the platform specific extension.
/// 
/// # Arguments
/// 
/// * `path` - The path to join with the file name.
/// * `file_name` - The file name to join with the path.
/// 
/// # Example
/// 
/// ```
/// use std::path::PathBuf;
/// 
/// use crate::utils::path::join_path;
/// 
/// let path = PathBuf::from("C:\\Users\\User\\Desktop");
/// let file_name = "test";
/// 
/// let joined_path = join_dll_path(&path, file_name);
/// 
/// #[cfg(target_os = "windows")]
/// assert_eq!(joined_path, PathBuf::from("C:\\Users\\User\\Desktop\\test.dll"));
/// #[cfg(target_os = "linux")]
/// assert_eq!(joined_path, PathBuf::from("C:\\Users\\User\\Desktop\\test.so"));
/// #[cfg(target_os = "macos")]
/// assert_eq!(joined_path, PathBuf::from("C:\\Users\\User\\Desktop\\test.dylib"));
#[macro_export]
macro_rules! join_dll_path {
    //take two arguments, a PathBuf and a file name
    ($path:expr, $file:expr) => {
        //join the path and the file name
        $path.join($file)
        //add the correct extension
        .with_extension(std::env::consts::DLL_EXTENSION)
    };
}

pub fn find_mono(game_base_path: &PathBuf, game_data_path: &PathBuf) -> Result<PathBuf, Box<dyn error::Error>> {
    let folder_names = vec!["MonoBleedingEdge", "Mono", "MonoBleedingEdge.x64", "MonoBleedingEdge.x86"];
    // workaround for weird path behavior, extension will be replaced with platform specific extension.
    let lib_names = vec!["libmono.so", "mono.dll", "mono-2.0-bdwgc.dll", "mono-2.0-sgen.dll", "mono-2.0-boehm.dll", "libmonobdwgc-2.0.so"];

    for folder_name in folder_names.iter() {
        for lib_name in lib_names.iter() {
            let lib_path = join_dll_path!(game_base_path.join(folder_name), lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }

            let lib_path = join_dll_path!(game_base_path.join(folder_name).join("EmbedRuntime"), lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }

            let lib_path = join_dll_path!(game_data_path.join(folder_name), lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }

            let lib_path = join_dll_path!(game_data_path.join(folder_name).join("x86_64"), lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }

            let lib_path = join_dll_path!(game_data_path.join(folder_name).join("EmbedRuntime"), lib_name);
            if lib_path.exists() {
                return Ok(lib_path);
            }
        }
    }

    Err(Box::new(PathError::MonoNotFound))
}

pub fn get_data_path(file_path: &PathBuf) -> Result<PathBuf, Box<dyn error::Error>> {
    let file_name = file_path.file_stem()
        .ok_or_else(|| PathError::DataPathNotFound)?
        .to_str()
        .ok_or_else(|| PathError::DataPathNotFound)?;

    let base_folder = file_path.parent()
        .ok_or_else(|| PathError::DataPathNotFound)?;

    let data_path = base_folder.join(format!("{}_Data", file_name));

    match data_path.exists() {
        true => Ok(data_path),
        false => Err(Box::new(PathError::DataPathNotFound)),
    }
}