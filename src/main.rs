#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Page;

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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{VirtAddr};

    println!("Hello World{}", "!");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)] test_main();

    println!("It did not crash");
    rust_os::hlt_loop();
}