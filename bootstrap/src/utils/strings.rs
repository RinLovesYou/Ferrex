#[macro_export]
macro_rules! win_str {
    ($s:expr) => {
        std::ffi::CStr::from_bytes_with_nul_unchecked($s).as_ptr()
    };
}
