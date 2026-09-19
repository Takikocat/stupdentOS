#![no_std]
#![no_main]

use core::panic::PanicInfo;

// The normal Rust runtime is unavailable, so we provide the entry point.
// Keep the `_start` symbol name visible to the linker and use a known ABI.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    loop {}
}