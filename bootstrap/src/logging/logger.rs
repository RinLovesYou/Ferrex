//! Handles logging across the entire package

use std::{error::{self, Error}, io::Write, path::PathBuf};

use colored::{Color, Colorize};
use thiserror::Error;

/// all possible errors that can occur when working with the logger
///
/// # Errors
/// ```
/// LogError::FailedToDeleteOldLog // Failed to delete the old log file
/// LogError::FailedToWriteToLog //Failed to write to the log file
/// LogError::FailedToGetBasePath //Failed to get the base path
/// ```
#[derive(Error, Debug)]
pub enum LogError {
    /// the log file could not be deleted
    #[error("Failed to delete the old log file")]
    FailedToDeleteOldLog,

    /// the log file could not be written to
    #[error("Failed to write to log file")]
    FailedToWriteToLog,

    /// the base path could not be fetched
    #[error("Failed to fetch base path")]
    FailedToGetBasePath,
}

/// gets the path to the log file
fn log_path() -> Result<PathBuf, Box<dyn error::Error>> {
    let base_path = std::env::current_dir()?;
    let log_path = base_path.join("Ferrex").join("Latest.log");

    Ok(log_path)
}

/// Initializes MelonLogger, which takes care of both logging to console & file
pub fn init() -> Result<(), Box<dyn error::Error>> {
    let log_path = log_path().map_err(|_| LogError::FailedToGetBasePath)?;

    if log_path.exists() {
        std::fs::remove_file(&log_path);
    }

    Ok(())
}

/// the different log levels
///
/// # Levels
/// ```
/// LogLevel::Info
/// LogLevel::Warning
/// LogLevel::Error
/// LogLevel::Debug
#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
pub enum LogLevel {
    /// Informational, always printed to console
    Info,
    /// Warning, always printed to console
    Warning,
    /// Error, always printed to console
    Error,
}

impl std::convert::TryFrom<u8> for LogLevel {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, <LogLevel as std::convert::TryFrom<u8>>::Error> {
        match value {
            0 => Ok(LogLevel::Info),
            1 => Ok(LogLevel::Warning),
            2 => Ok(LogLevel::Error),
            _ => Err("Invalid value for enum `LogLevel` possible: [1, 2, 3]".into())
        }
    }
}

static RED: Color = Color::TrueColor {
    r: (255),
    g: (0),
    b: (0),
};
static PINK: Color = Color::TrueColor {
    r: (253),
    g: (117),
    b: (255),
};

fn write(msg: &str) -> Result<(), Box<dyn error::Error>> {
    let log_path = log_path().map_err(|_| LogError::FailedToGetBasePath)?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(&log_path)
        .map_err(|_| LogError::FailedToWriteToLog)?;

    let message = format!("{}\r\n", msg);

    file.write_all(message.as_bytes())
        .map_err(|_| LogError::FailedToWriteToLog)?;

    Ok(())
}

/// logs to console and file, should not be used, use the log! macro instead
pub fn log_console_file(level: LogLevel, message: &str) -> Result<(), LogError> {
    match level {
        LogLevel::Info => {
            let console_string = format!(
                "{}{}{} {}",
                "[".bright_black(),
                timestamp().color(PINK),
                "]".bright_black(),
                message
            );

            let file_string = format!("[{}] {}", timestamp(), message);

            println!("{}", console_string);
            write(&file_string).map_err(|_| LogError::FailedToWriteToLog)?;
        }
        LogLevel::Warning => {
            //same as log, but all colors are yellow
            let console_string = format!("[{}] [WARNING] {}", timestamp(), message);

            let file_string = format!("[{}] [WARNING] {}", timestamp(), message);

            println!("{}", console_string.yellow());

            write(&file_string).map_err(|_| LogError::FailedToWriteToLog)?;
        }
        LogLevel::Error => {
            //same as log, but all colors are red

            let log_string = format!("[{}] [ERROR] {}", timestamp(), message);

            println!("{}", log_string.color(RED));
            write(&log_string).map_err(|_| LogError::FailedToWriteToLog)?;
        }
    }

    Ok(())
}

/// Fetches the current time, and formats it.
///
/// returns a String, formatted as 15:23:24:123
fn timestamp() -> String {
    let now = chrono::Local::now();
    let time = now.time();

    time.format("%H:%M:%S.%3f").to_string()
}

/// Logs a message to the console and log file
///
/// # Example
/// ```
/// log!("Hello World!")?;
/// ```
/// log! returns a Result<(), Box<LogError>>, so please handle this.
#[macro_export]
macro_rules! log {
    //case 1: empty
    () => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Info, "")
    };

    //case 2: single argument
    ($msg:expr) => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Info, $msg)
    };

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let msg = &format_args!($($arg)*).to_string();
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Info, msg)
    }};
}

/// Logs a warning to the console and log file
///
/// # Example
/// ```
/// warn!("Hello World!")?;
/// ```
/// warn! returns a Result<(), Box<LogError>>, so please handle this.
#[macro_export]
macro_rules! warn {
    //case 1: empty
    () => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Warning, "")
    };

    //case 2: single argument
    ($msg:expr) => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Warning, $msg)
    };

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let msg = &format_args!($($arg)*).to_string();
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Warning, msg)
    }};
}

/// Logs an error to the console and log file
///
/// # Example
/// ```
/// error!("Hello World!")?;
/// ```
/// error! returns a Result<(), Box<LogError>>, so please handle this.
#[macro_export]
macro_rules! err {
    //case 1: empty
    () => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Error, "")
    };

    //case 2: single argument
    ($msg:expr) => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Error, $msg)
    };

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let msg = &format_args!($($arg)*).to_string();
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Error, msg)
    }};
}

/// Logs a debug message to the console and log file
///
/// # Example
/// ```
/// debug!("Hello World!")?;
/// ```
/// debug! returns a Result<(), Box<LogError>>, so please handle this.
#[macro_export]
macro_rules! debug {
    //case 1: empty
    () => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Debug, "")
    };

    //case 2: single argument
    ($msg:expr) => {
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Debug, $msg)
    };

    //case 3: multiple arguments
    ($($arg:tt)*) => {{
        let msg = &format_args!($($arg)*).to_string();
        $crate::logging::logger::log_console_file($crate::logging::logger::LogLevel::Debug, msg)
    }};
}

#[macro_export]
macro_rules! cstr {
    ($s:expr) => {
        std::ffi::CString::new($s)?.as_ptr()
    };

    //format str
    ($($arg:tt)*) => {
       std::ffi::CString::new(format!($($arg)*))?.as_ptr()
    };
}
