#![no_std]  // 標準ライブラリの使用を許さない
#![no_main] // 通常のエントリポイントは使用しない

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::Writer::print_something();
    
    loop {}
}

