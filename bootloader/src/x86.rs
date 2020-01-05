// read a byte from the port
#[inline]
pub fn inb(port: u16) -> u8 {
    let data: u8;
    unsafe {
        asm!("inb $1, $0"
            : "={al}"(data)
            : "{dx}"(port)
            : 
            : "volatile");
    }
    data
}

// read cnt double-words from the port
#[inline]
pub fn insl(port: u16, addr: *mut u32, cnt: usize) {
    let mut _addr = addr;
    let mut _cnt = cnt;
    unsafe {
        asm!("cld; rep insl"
            : "+{edi}"(_addr), "+{ecx}"(_cnt)
            : "{dx}"(port)
            : "memory", "cc"
            : "volatile");
    }
}

// write the byte (data) to the port
#[inline]
pub fn outb(port: u16, data:u8) {
    unsafe {
        asm!("outb $0, $1"
            :
            : "{al}"(data), "{dx}"(port)
            :
            : "volatile");
    }
}

// write the word (data) to the port
#[inline]
pub fn outw(port: u16, data: u16) {
    unsafe {
        asm!("outw $0, $1"
            :
            : "{ax}"(data), "{dx}"(port)
            :
            : "volatile");
    }
}

// write cnt double-words from the addr to the port
#[inline]
pub fn outsl(port: u16, addr: *const u32, cnt: usize) {
    let mut _addr = addr;
    let mut _cnt = cnt;
    unsafe {
        asm!("cld; rep outsl"
            : "+{esi}"(_addr), "+{ecx}"(_cnt)
            : "{dx}"(port)
            : "cc"
            : "volatile");
    }
}

// write the byte (data) to the address (cnt times repeatedly)
#[inline]
pub fn stosb(addr: *const u8, data: u8, cnt: usize) {
    let mut _addr = addr;
    let mut _cnt = cnt;
    unsafe {
        asm!("cld; rep stosb"
            : "+{edi}"(_addr), "+{ecx}"(_cnt)
            : "{al}"(data)
            : "memory", "cc"
            : "volatile");
    }
}

// write the double word (data) to the address (cnt times repeatedly)
#[inline]
pub fn stosl(addr: *const u8, data: u32, cnt: usize) {
    let mut _addr = addr;
    let mut _cnt = cnt;
    unsafe {
        asm!("cld; rep stosl"
            : "+{edi}"(_addr), "+{ecx}"(_cnt)
            : "{eax}"(data)
            : "memory", "cc"
            : "volatile");
    }
}

// do nothing
#[inline]
pub fn nop() {
    unsafe {
        asm!("nop");
    }
}
