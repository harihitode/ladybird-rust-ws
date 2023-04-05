#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};
global_asm!(include_str!("crt0.s"));

#[no_mangle]
pub extern "C" fn main() -> u32 {
    let uart = 0x1000_0000 as *mut u8;
    for c in b"Hello, World!\n".iter() {
        unsafe {
            *uart = *c as u8;
        }
    }
    return 0;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
