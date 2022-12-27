use std::io;

use thiserror::Error;
use unity_rs::runtime::RuntimeError;

use crate::logging::logger::LogError;

#[derive(Debug, Error)]
pub enum ModError {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
    #[error(transparent)]
    Log(#[from] LogError),
    #[error(transparent)]
    Io(#[from] io::Error),
}