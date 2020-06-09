#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rust_os::println;
use x86_64::structures::paging::PageTable;
use rust_os::memory::translate_addr;

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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let addresses = [
        // The identity-mapped vga buffer page.
        0xb8000,
        // Some code page
        0x201008,
        // Some stack page
        0x0100_0020_1a10,
        // Virtual address mapped to physical address 0
        boot_info.physical_memory_offset
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, physical_memory_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)] test_main();

    println!("It did not crash");
    rust_os::hlt_loop();
}