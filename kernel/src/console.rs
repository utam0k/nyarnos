use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::uart::Uart;

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console::new());
}

pub struct Console {
    uart: Uart,
}

impl Console {
    fn new() -> Console {
        Console {
            uart: Uart::new().unwrap(),
        }
    }
    fn write_byte(&self, value: u8) {
        self.uart.write_byte(value)
    }
}
impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
          $crate::console::print(format_args!($($arg)*));
    });
}
