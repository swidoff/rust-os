#![no_std] // Don't link the Rust standard library
#![no_main]

use core::panic::PanicInfo;

use rust_os::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle] // Don't mangle the name of this function.
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}

fn should_fail() {
    serial_print!("should fail... ");
    assert_eq!(0, 1);
}