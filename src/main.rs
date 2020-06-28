#![no_std] // Don't link the Rust standard library
#![no_main] // Disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

// use alloc::boxed::Box;
// use alloc::rc::Rc;
// use alloc::vec;
// use alloc::vec::Vec;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

use rust_os::println;
use rust_os::task::executor::Executor;
use rust_os::task::keyboard;
use rust_os::task::simple_executor::SimpleExecutor;
use rust_os::task::Task;

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
    use rust_os::{memory, allocator};
    use x86_64::{VirtAddr};

    println!("Hello World{}", "!");
    rust_os::init();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_memory_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    // // allocate a new number on the heap
    // let heap_value = Box::new(41);
    // println!("heap value at {:p}", heap_value);
    //
    // // create a dynamically-sized vector
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());
    //
    // // create a reference counted vector -> will be freed when count reaches 0.
    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    // core::mem::drop(cloned_reference);
    // println!("reference count is {} now", Rc::strong_count(&reference_counted));

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
