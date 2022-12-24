//! various filesystem utils

use std::path::PathBuf;

/// get the path to bootstra
pub fn get_bootstrap_path(base_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let win_default = ["Ferrex", "Bootstrap"];
    let unix_default = ["Ferrex", "libBootstrap"];

    let path = base_path.clone();

    let mut windows_path = path.clone();
    let mut unix_path = path.clone();

    windows_path.extend(win_default.iter());
    unix_path.extend(unix_default.iter());

    let windows_path = windows_path.with_extension(std::env::consts::DLL_EXTENSION);
    let unix_path = unix_path.with_extension(std::env::consts::DLL_EXTENSION);

    if windows_path.exists() {
        Ok(windows_path)
    } else if unix_path.exists() {
        Ok(unix_path)
    } else {
        Err("Failed to find bootstrap".into())
    }
}