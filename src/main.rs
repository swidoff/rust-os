#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

/// This function is called on panic.
#[panic_handler]
#[cfg(not(test))]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[panic_handler]
#[cfg(test)]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[no_mangle] // Don't mangle the name of this function.
/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rust_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _)  = Cr3::read();
    println!("Level 4 page table at {:?}", level_4_page_table.start_address());

    #[cfg(test)] test_main();

    println!("It did not crash");
    rust_os::hlt_loop();
}