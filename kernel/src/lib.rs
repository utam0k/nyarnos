#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(global_asm)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(asm)]

use core::panic::PanicInfo;

pub mod console;
pub mod uart;

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    println!("[failed]\n");
    println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub const NPDENTRIES: usize = 1024;
type PageDirEntry = u32;

macro_rules! assigned_array {
    ($def:expr; $len:expr; $([$idx:expr] = $val:expr),*) => {{
        let mut tmp = [$def; $len];
        $(tmp[$idx] = $val;)*
        tmp
    }};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[inline(always)]
unsafe fn write_to_port(port: u16, value: u32) {
    asm!("outl $1, $0" :: "N{dx}"(port), "{eax}"(value) :: "volatile");
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    println!("EXIT {:?}", exit_code);
    unsafe {
        write_to_port(0xf4, exit_code as u32)
        // let mut port = Port::new(0xf4);
        // port.write(exit_code as u32);
    }
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

global_asm!(include_str!("entry.S"));

/// Entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn main() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
