#![no_std] // 標準ライブラリの使用を許さない
#![no_main] // 通常のエントリポイントは使用しない

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::{fmt::Write, panic::PanicInfo};
mod vga_buffer;
mod serial;

// 通常使うPanic Handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// テストモードで使う Panic Handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Macro{}", "!");

    vga_buffer::WRITER.lock().write_str("Hello Again").unwrap();
    write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 42, 1.337).unwrap();

    // panic!("Something Error");

    #[cfg(test)]
    test_main();

    loop {}
}

/// QEMU Exit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        use x86_64::instructions::port::Port;
        let mut port = Port::new(0xf4); // I/OのBase
        port.write(exit_code as u32);
    }
}


/// Test
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    // Exit
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    //print!("trivial assertion... ");
    serial_print!("trivial asserition...");
    assert_eq!(1, 1);
    // println!("[ok]");
    serial_println!("[ok]")
}