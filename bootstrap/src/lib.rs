#![feature(type_ascription)]

mod core;
mod logging;
mod hooking;
mod errors;
mod mods;

use ctor::ctor;

#[no_mangle]
#[ctor]
fn init(){
    core::init().unwrap_or_else(|e| {
        internal_failure!("Failed to initialize: {}", e);
    });
}