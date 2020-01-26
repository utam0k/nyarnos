use bitflags::bitflags;

#[derive(Clone)]
#[repr(C)]
pub struct InterruptStackFrameValue {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub struct InterruptStackFrame {
    pub value: InterruptStackFrameValue,
}

bitflags! {
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        const PROTECTION_VIOLATION = 1 << 0;
        const CAUSED_BY_WRITE = 1 << 1;
        const USER_MODE = 1 << 2;
        const MALFORMED_TABLE = 1 << 3;
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame);
pub type HandlerFuncWithErrCode = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u32);
pub type PageFaultHandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: PageFaultErrorCode);
pub type DivergingHandlerFunc = extern "x86-interrupt" fn(&mut InterruptStackFrame) -> !;
pub type DivergingHandlerFuncWithErrCode = extern "x86-interrupt" fn(&mut InterruptStackFrame, error_code: u32) -> !;
