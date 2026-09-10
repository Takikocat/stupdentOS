//cargo build --target thumbv7em-none-eabihf

#![no_std] // Do not link against the Rust standard library.
#![no_main] // It does not use the execution entry point (the `main` function) used in the Rust language.

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // This function's name will not be mangled
pub extern "C" fn _start() -> ! {
    // The linker will use the function named '_start' as the execution entry point,
    // so this function becomes the execution entry point.
    loop {}
}

/// This function is called when a panic occurs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}