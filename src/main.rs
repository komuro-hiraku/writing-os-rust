#![no_std] // 標準ライブラリの使用を許さない
#![no_main] // 通常のエントリポイントは使用しない

use core::{fmt::Write, panic::PanicInfo};
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Macro{}", "!");

    vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
    write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 42, 1.337).unwrap();

    // panic!("Something Error");

    loop {}
}
