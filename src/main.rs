#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crate::qemu_exit::{QemuExitCode, exit_qemu};

mod vga_buffer;
mod qemu_exit;
mod serial;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(r#"Chapool Kernel 0.0.1"#);
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))] // not in test mode
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn cpu_test() {
    serial_println!("CPU");
    assert_eq!(1, 1);
    serial_println!("[ok]");
}