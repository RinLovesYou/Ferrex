use scotch_host::host_function;

use crate::log;

#[host_function]
pub fn fx_log_str(text: &String) {
    log!("{}", text);
}