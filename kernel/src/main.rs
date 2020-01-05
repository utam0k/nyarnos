#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(lang_items)]

use core::panic::PanicInfo;

use kernel::print;
use kernel::println;

global_asm!(include_str!("entry.S"));

pub const NPDENTRIES: usize = 1024;
type PageDirEntry = u32;

macro_rules! assigned_array {
    ($def:expr; $len:expr; $([$idx:expr] = $val:expr),*) => {{
        let mut tmp = [$def; $len];
        $(tmp[$idx] = $val;)*
        tmp
    }};
}

#[used]
#[no_mangle]
#[link_section = ".rodata.entrypgdir"]
pub static entrypgdir: [PageDirEntry; NPDENTRIES] = assigned_array![
    0; NPDENTRIES;
    // Map VA's [0, 4MB) to PA's [0, 4MB)
    [0] = 0x000 | 0x001 | 0x002 | 0x080,
    // Map VA's [KERNBASE, KERNBASE+4MB) to PA's [0, 4MB)
    [0x80000000 >> 22] = 0x000 | 0x001 | 0x002 | 0x080
    // 0x80 means the size of the page is 4MiB
];

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
    println!("Yattane");

    unimplemented!();
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}
