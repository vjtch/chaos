#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello_world = b"Hello world!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hello_world.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xc;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}
