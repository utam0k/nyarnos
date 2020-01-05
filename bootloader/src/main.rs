#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;

mod elf;
mod x86;

use x86::*;

global_asm!(include_str!("boot.S"));

const SECTSIZE: usize = 512; // same as u32 on i386

type Sector = [u8; SECTSIZE];

#[no_mangle]
pub unsafe extern "C" fn bootmain() {
    let elf_ptr = 0x10000 as *mut u8;
    let elf = (elf_ptr as *mut elf::ELFHeader).as_ref().unwrap();

    // Read 1st page off disk
    read_segment(elf_ptr, 4096, 0);

    // Is this an ELF executable?
    if elf.magic != elf::ELF_MAGIC {
        return;
    }

    // Program segment
    let mut ph_ptr = elf_ptr.offset(elf.phoff as isize) as *const elf::ProgramHeader;
    let eph_ptr = ph_ptr.offset(elf.phnum as isize) as *const elf::ProgramHeader;
    while ph_ptr < eph_ptr {
        let ph = ph_ptr.as_ref().unwrap();
        let pa = ph.paddr;
        read_segment(pa, ph.filesz, ph.offset);
        if ph.memsz > ph.filesz {
            stosb(pa.offset(ph.filesz as isize), 0, ph.memsz - ph.filesz);
        }
        ph_ptr = ph_ptr.offset(1);
    }

    print_vga(b"Kernel loaded.");
    (elf.entry)();
    unreachable!()
}

unsafe fn read_segment(pa: *mut u8, count: usize, offset: usize) {
    let end_pa = pa.offset(count as isize);
    let mut pa = pa.offset(-((offset % SECTSIZE) as isize)) as *mut Sector;
    let mut offset = (offset / SECTSIZE) + 1;

    while (pa as *mut u8) < end_pa {
        read_sector(pa, offset);
        pa = pa.offset(1);
        offset += 1;
    }
}

unsafe fn wait_disk() {
    while (inb(0x01F7) & 0xC0) != 0x40 {}
}

macro_rules! trunc8 {
    ($x:expr) => {
        ($x & 0xFF) as u8
    };
}

unsafe fn read_sector(dst: *mut Sector, offset: usize) {
    wait_disk();
    outb(0x01F2, 1); // count = 1
    outb(0x01F3, trunc8!(offset >> 0)); // Low byte
    outb(0x01F4, trunc8!(offset >> 8)); // Mid byte
    outb(0x01F5, trunc8!(offset >> 16)); // High byte
    outb(0x01F6, trunc8!(offset >> 24) | 0xE0); // Last byte
    outb(0x01F7, 0x20); // read sectors
    wait_disk();

    insl(0x01F0, dst as *mut u32, SECTSIZE / 4);
}

unsafe fn print_vga(s: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &b) in s.iter().enumerate() {
        *vga_buffer.offset((i * 2 + 0) as isize) = b;
        *vga_buffer.offset((i * 2 + 1) as isize) = 0b00001010;
    }
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    loop {}
}
