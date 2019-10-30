#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use crate::qemu_exit::QemuExitCode;

mod vga_buffer;
mod qemu_exit;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!(r#"Chapool Kernel 0.0.1"#);
    #[cfg(test)]
    test_main();
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn cpu_test() {
    print!("CPU");
    assert_eq!(1, 1);
    println!("[ok]");
}