use core::fmt;
use core::marker::PhantomData;

use super::handler::{
    DivergingHandlerFunc, DivergingHandlerFuncWithErrCode, HandlerFunc, HandlerFuncWithErrCode, PageFaultHandlerFunc,
};
use bit_field::BitField;

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_high: u16,
    phantom: PhantomData<F>,
}

macro_rules! impl_set_handler_fn {
    ($h:ty) => {
        impl Entry<$h> {
            pub fn set_handler_fn(&mut self, handler: $h) -> &mut EntryOptions {
                self.set_handler_addr(handler as u32)
            }
        }
    };
}
impl_set_handler_fn!(HandlerFunc);
impl_set_handler_fn!(HandlerFuncWithErrCode);
impl_set_handler_fn!(PageFaultHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFunc);
impl_set_handler_fn!(DivergingHandlerFuncWithErrCode);

impl<F> fmt::Debug for Entry<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use core::mem::size_of_val;
        write!(f, "size_of: {} bytes\n", size_of_val(self))?;
        write!(
            f,
            "p_low: {:b}, size_of: {}\n",
            &self.pointer_low,
            size_of_val(&self.pointer_low)
        )?;
        write!(
            f,
            "gdt_selector: {:b}, size_of: {}\n",
            &self.gdt_selector,
            size_of_val(&self.gdt_selector)
        )?;
        write!(
            f,
            "options: {:?}, size_of: {}\n",
            &self.options,
            size_of_val(&self.options)
        )?;
        write!(
            f,
            "p_high: {:b}, size_of: {}\n",
            &self.pointer_high,
            size_of_val(&self.pointer_high)
        )
    }
}

impl<F> Entry<F> {
    pub const fn missing() -> Self {
        Entry {
            pointer_low: 0,
            gdt_selector: 0,
            options: EntryOptions::minimal(),
            pointer_high: 0,
            phantom: PhantomData::<F>,
        }
    }

    pub fn set_handler_addr(&mut self, addr: u32) -> &mut EntryOptions {
        self.pointer_low = addr as u16;
        self.pointer_high = (addr >> 16) as u16;

        unsafe { asm!("mov %cs, $0" : "=r" (self.gdt_selector) ) };

        self.options.set_present(true);

        &mut self.options
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct EntryOptions(u16);

impl EntryOptions {
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    const fn minimal() -> Self {
        EntryOptions(0b0000_1110_0000_0000)
    }

    pub unsafe fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index + 1);
        self
    }
}

impl fmt::Debug for EntryOptions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}
