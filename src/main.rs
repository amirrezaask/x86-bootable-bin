#![no_std]
// we want rust compiler to ignore default rust start chain
#![no_main]

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop{}
}

static TEXT: &[u8] = b"Hello World!";
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

