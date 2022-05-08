use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

pub struct ConsoleInner;

impl ConsoleInner {
    fn puts(&self, args: fmt::Arguments) {
        Stdout.write_fmt(args).unwrap();
    }
}

static CONSOLE: ConsoleInner = ConsoleInner {};

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    CONSOLE.puts(args);
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
