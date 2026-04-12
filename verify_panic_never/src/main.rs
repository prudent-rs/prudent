// Thanks to
// - https://docs.rs/panic-never/latest/panic_never/ and
// - https://dev.to/ender_minyard/rust-nostd-template-23j0

#![no_std]
#![no_main]

#[cfg(debug_assertions)]
compile_error!("Build in release profile only. Required by `panic_never`.");

#[cfg(not(debug_assertions))]
use panic_never as _;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {


    loop {}
}
