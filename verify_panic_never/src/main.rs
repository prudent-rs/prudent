// Thanks to
// - https://docs.rs/panic-never/latest/panic_never/ and
// - https://dev.to/ender_minyard/rust-nostd-template-23j0

#![no_std]
#![no_main]

#[cfg(debug_assertions)]
compile_error!("Build in release profile only. Required by `panic_never`.");

#[cfg(not(debug_assertions))]
use panic_never as _;

// This is NOT being used - because in debug mode we get the compile error from above. However, we
// do include this in debug mode, otherwise (in debug mode) we would also get a confusing error:
//
// `#[panic_handler]` function required, but not found
#[cfg(debug_assertions)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
