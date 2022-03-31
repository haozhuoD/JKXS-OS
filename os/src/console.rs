use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use lazy_static::*;
use spin::Mutex;

struct Stdout;

pub struct ConsoleInner;

lazy_static! {
    pub static ref CONSOLE: Mutex<ConsoleInner> = Mutex::new(ConsoleInner {});
}

impl ConsoleInner {
    pub fn putstr(&self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        CONSOLE.lock().putstr(s)
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}