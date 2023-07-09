#![no_std]  // 標準ライブラリの使用を許さない
#![no_main] // 通常のエントリポイントは使用しない

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}