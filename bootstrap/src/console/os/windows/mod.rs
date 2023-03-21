//! Windows Console Module.

use std::sync::Mutex;

use lazy_static::lazy_static;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{self, HANDLE, HWND},
        System::Console::*,
    },
};

use crate::{errors::{DynErr, conerr::ConsoleError}, win_str};

lazy_static! {
    static ref WINDOW: Mutex<HWND> = Mutex::new(HWND(0));
    static ref OUTPUT_HANDLE: Mutex<HANDLE> = Mutex::new(HANDLE(0));
}

pub unsafe fn init() -> Result<(), DynErr> {
    // creates a console window, if one already exists it'll just return true.
    if !AllocConsole().as_bool() {
        return Err(ConsoleError::FailedToAllocateConsole.into());
    }

    // store the console window handle
    let mut window = WINDOW.lock()?;
    *window = GetConsoleWindow();

    // a null check
    if window.0 == 0 {
        return Err(ConsoleError::FailedToGetConsoleWindow.into());
    }

    // this lets us hook into console close events, and run some cleanup logic.
    if SetConsoleCtrlHandler(Some(ctrl_handler_hook), Foundation::TRUE) == Foundation::FALSE {
        return Err(ConsoleError::FailedToSetConsoleCtrlHandler.into());
    }

    set_title("Ferrex v0.0.1\0");

    let _ = libc::freopen(
        win_str!(b"CONIN$\0"),
        win_str!(b"r\0"),
        libc_stdhandle::stdin(),
    );
    let _ = libc::freopen(
        win_str!(b"CONOUT$\0"),
        win_str!(b"w\0"),
        libc_stdhandle::stdout(),
    );
    let _ = libc::freopen(
        win_str!(b"CONOUT$\0"),
        win_str!(b"w\0"),
        libc_stdhandle::stderr(),
    );

    // needs to be in its own scope to drop the lock
    {
        let mut output_handle = OUTPUT_HANDLE.lock()?;
        *output_handle = GetStdHandle(STD_OUTPUT_HANDLE)?;
    }

    set_handles()?;

    let output_handle = OUTPUT_HANDLE.lock()?;

    let mut mode = CONSOLE_MODE(0);
    let _ = GetConsoleMode(*output_handle, &mut mode);

    mode |= ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT;

    if SetConsoleMode(*output_handle, mode) != Foundation::TRUE {
        mode &= !(ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT);
    } else {
        mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;

        if SetConsoleMode(*output_handle, mode) != Foundation::TRUE {
            mode &= !ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        }
    }

    mode |= ENABLE_EXTENDED_FLAGS;
    mode &= !(ENABLE_MOUSE_INPUT | ENABLE_WINDOW_INPUT | ENABLE_INSERT_MODE);

    let _ = SetConsoleMode(*output_handle, mode);
    Ok(())
}

pub fn set_title(title: &str) {
    unsafe {
        let t = PCSTR(title.as_ptr());
        let _ = SetConsoleTitleA(t);
    }
}

pub fn set_handles() -> Result<(), DynErr> {
    unsafe {
        let handle = OUTPUT_HANDLE.lock()?;

        let _ = SetStdHandle(STD_OUTPUT_HANDLE, *handle);
        let _ = SetStdHandle(STD_ERROR_HANDLE, *handle);

        Ok(())
    }
}

pub fn null_handles() -> Result<(), DynErr> {
    unsafe {
        let _ = SetStdHandle(STD_OUTPUT_HANDLE, HANDLE(0));
        let _ = SetStdHandle(STD_ERROR_HANDLE, HANDLE(0));

        Ok(())
    }
}

unsafe extern "system" fn ctrl_handler_hook(ctrltype: u32) -> Foundation::BOOL {
    match ctrltype {
        CTRL_C_EVENT | CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => {
            std::process::exit(0);
        }

        _ => Foundation::FALSE,
    }
}
