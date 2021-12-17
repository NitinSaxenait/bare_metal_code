// cutting our application to use standard library.
#![no_std]
// with no standard library we use no_main and define our own entry point.
#![no_main]

use core::panic::PanicInfo;
// save us from compiler to provide unique name to our function.
#[no_mangle]
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {

    loop {}
}

/// This function saves our application from getting into panic.
/// This will handle the panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

