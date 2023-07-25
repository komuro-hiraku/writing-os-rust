#![no_std]  // 標準ライブラリの使用を許さない
#![no_main] // 通常のエントリポイントは使用しない

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // リンカーはデフォルトで `_start` という名前で関数を探すのでここがエントリポイント
    loop {}
}

