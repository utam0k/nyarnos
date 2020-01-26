#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use kernel::idt;
use kernel::print;
use kernel::println;

#[no_mangle]
pub extern "C" fn main() {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in b"Entry Kernel!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    println!("Hello, world!");
    idt::init();

    println!("Yattane");

    #[cfg(test)]
    test_main();

    unimplemented!();
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}
