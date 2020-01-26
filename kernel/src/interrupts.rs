use crate::idt::handler::InterruptStackFrame;
use crate::{print, println};

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n");
}

pub extern "x86-interrupt" fn double_fault_handler(_stack_frame: &mut InterruptStackFrame, _error_code: u32) {
    println!("EXCEPTION: DOUBLE FAULT\n");
}

#[test_case]
fn test_breakpoint_exception() {
    #[inline]
    pub unsafe fn int3() {
        asm!("int3" :::: "volatile");
    }

    println!("test_breakpoint_exception...");
    println!("invoke a breakpoint exception!(int3)");
    unsafe {
        int3();
    }
    println!("[ok]");
}
