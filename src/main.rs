#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // Don't mangle the name of this function.
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);
    panic!("Some panic message");
    loop {}
}