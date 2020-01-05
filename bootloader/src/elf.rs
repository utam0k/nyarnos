pub const ELF_MAGIC: u32 = 0x464C457F; /* "\x7FELF" in little endian */

#[repr(C)]
pub struct ProgramHeader {
    pub p_type: u32,    // segment type
    pub offset: usize,  // segment offset
    pub vaddr: *mut u8, // virtual address of segment
    pub paddr: *mut u8, // physical address - ignored ?
    pub filesz: usize,  // number of bytes in file for seg.
    pub memsz: usize,   // number of bytes in mem. for seg.
    pub flags: u32,     // flags
    pub align: usize,   // memory alignment
}

#[repr(C)]
pub struct ELFHeader {
    pub magic: u32,                   // ELF Identification
    pub elf: [u8; 12],                // ELF Identification
    pub e_type: u16,                  // object file type
    pub machine: u16,                 // machine
    pub version: u32,                 // object file version
    pub entry: extern "C" fn() -> (), // virtual entry point
    pub phoff: usize,                 // program header table offset
    pub shoff: usize,                 // section header table offset
    pub flags: u32,                   // processor-specific flags
    pub ehsize: u16,                  // ELF header size
    pub phentsize: u16,               // program header entry size
    pub phnum: u16,                   // number of program header entries
    pub shent_size: u16,              // section header entry size
    pub shnum: u16,                   // number of section header entries
    pub shstrndx: u16,                // section header tables's
}
