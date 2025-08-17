// build with `cargo build --target x86_64-unknown-none`

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::global_asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}

}

global_asm!(".fill 510, 1, 0x00",
           ".word 0xAA55");