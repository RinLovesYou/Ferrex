use dobby_rs::DobbyHookError;
use thiserror::Error;
use unity_rs::runtime::RuntimeError;

use crate::logging::logger::LogError;

use super::moderr::ModError;

#[derive(Debug, Error)]
pub enum HookError {
    #[error(transparent)]
    Dobby(#[from] DobbyHookError),
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
    #[error(transparent)]
    Mod(#[from] ModError),
    #[error(transparent)]
    Log(#[from] LogError),

    #[error("Hook returned a Nullpointer trampoline")]
    Null,
    #[error("Paramter {0} is a Nullpointer")]
    Nullpointer(String),
    #[error("Trampoline to {0} is none!")]
    NoTrampoline(String),
}