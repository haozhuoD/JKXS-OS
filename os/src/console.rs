#![allow(unused)]

use crate::sbi::console_putchar;
use core::fmt::{self, Write};
use spin::{Lazy, RwLock};

struct Stdout;

pub struct ConsoleInner;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 无锁print
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

// 有锁print
pub fn lock_console_and_print(args: fmt::Arguments) {
    let lock = CONSOLE.write();
    print(args);
}

pub static CONSOLE: Lazy<RwLock<ConsoleInner>> = Lazy::new(|| RwLock::new(ConsoleInner {}));

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::lock_console_and_print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::lock_console_and_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}


// ======================== color constants ========================
const FG_BLACK      :u8 = 30;
const FG_RED        :u8 = 31;
const FG_GREEN      :u8 = 32;
const FG_YELLOW     :u8 = 33;
const FG_BLUE       :u8 = 34;
const FG_MAGENTA    :u8 = 35;
const FG_CYAN       :u8 = 36;
const FG_WHITE      :u8 = 37;

const FG_B_BLACK    :u8 = 90;
const FG_B_RED      :u8 = 91;
const FG_B_GREEN    :u8 = 92;
const FG_B_YELLOW   :u8 = 93;
const FG_B_BLUE     :u8 = 94;
const FG_B_MAGENTA  :u8 = 95;
const FG_B_CYAN     :u8 = 96;
const FG_B_WHITE    :u8 = 97;

const FG_DEFAULT    :u8 = 39;

const BG_BLACK      :u8 = 40;
const BG_RED        :u8 = 41;
const BG_GREEN      :u8 = 42;
const BG_YELLOW     :u8 = 43;
const BG_BLUE       :u8 = 44;
const BG_MAGENTA    :u8 = 45;
const BG_CYAN       :u8 = 46;
const BG_WHITE      :u8 = 47;

const BG_B_BLACK    :u8 = 100;
const BG_B_RED      :u8 = 101;
const BG_B_GREEN    :u8 = 102;
const BG_B_YELLOW   :u8 = 103;
const BG_B_BLUE     :u8 = 104;
const BG_B_MAGENTA  :u8 = 105;
const BG_B_CYAN     :u8 = 106;
const BG_B_WHITE    :u8 = 107;

const BG_DEFAULT    :u8 = 49;

// ======================== log ========================

/// kernel output log level
#[derive(PartialOrd)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum LogLevel {
    Verbose = 0,
    Debug   = 1,
    Info    = 2,
    Warning = 3,
    Error   = 4,
    Fatal   = 5,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::result::Result<(), core::fmt::Error> { 
        let s = match *self {
            LogLevel::Verbose   => "[VERBOSE]",
            LogLevel::Debug     => "[ DEBUG ]",
            LogLevel::Info      => "[ INFO ]",
            LogLevel::Warning   => "[WARNING]",
            LogLevel::Error     => "[ ERROR ]",
            LogLevel::Fatal     => "[ FATAL ]"
        };
        f.write_str(s)
    }
}

/// Set console color according to the log level
pub fn set_log_color(ll: LogLevel) {
    match ll {
        LogLevel::Verbose   => set_color(FG_B_BLACK,    BG_DEFAULT),
        LogLevel::Debug     => set_color(FG_DEFAULT,    BG_DEFAULT),
        LogLevel::Info      => set_color(FG_B_GREEN,    BG_DEFAULT),
        LogLevel::Warning   => set_color(FG_B_YELLOW,   BG_DEFAULT),
        LogLevel::Error     => set_color(FG_B_RED,      BG_DEFAULT),
        LogLevel::Fatal     => set_color(FG_BLACK,      BG_B_RED  )
    }
}

/// Set foreground color and background color.  
/// Foreground and background color codes are from [ANSI Escape Codes](https://en.wikipedia.org/wiki/ANSI_escape_code)
pub fn set_color(fg: u8, bg: u8) {
    print(format_args!("\x1b[{};{}m", fg, bg));
}

/// Reset console color to default.
pub fn reset_color() {
    set_color(FG_DEFAULT, BG_DEFAULT);
}

/// Return the minimal log level of this build.
fn min_log_level() -> LogLevel {
    if cfg!(feature = "min_log_level_fatal") {
        return LogLevel::Fatal;
    } else if cfg!(feature = "min_log_level_error") {
        return LogLevel::Error;
    } else if cfg!(feature = "min_log_level_warning") {
        return LogLevel::Warning;
    } else if cfg!(feature = "min_log_level_info") {
        return LogLevel::Info;
    } else if cfg!(feature = "min_log_level_debug") {
        return LogLevel::Debug;
    } else {
        return LogLevel::Verbose;
    }
}


/// Print log info, alongside with log level, source file and line number.  
/// *Don't call this function. Use marcos instead.*
pub fn log(log_level: LogLevel, args: fmt::Arguments, file: &'static str, line: u32) {
    if log_level >= min_log_level() {
        let lock = CONSOLE.write();
        set_log_color(log_level);
        print(format_args!("{} {:#?} @ {:#?} : {:#?} \n", log_level, file, line , args));
        reset_color();
    }
}

/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// verbose!("This is a verbose message!");
/// ```
#[macro_export]
macro_rules! verbose {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log(crate::sbi::LogLevel::Verbose, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// debug!("This is a debug message!");
/// ```
#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log(crate::sbi::LogLevel::Debug, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// info!("This is an info message!");
/// ```
#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log(crate::console::LogLevel::Info, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// warning!("This is an warning message!");
/// ```
#[macro_export]
macro_rules! warning {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log(crate::console::LogLevel::Warning, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// error!("This is an error message!");
/// ```
#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::log(crate::console::LogLevel::Error, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number. Will not print if the log level is lower then the min_log_level.
/// # Examples
/// ```
/// fatal!("This is a fatal message!");
/// ```
#[macro_export]
macro_rules! fatal {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::sbi::log(crate::console::LogLevel::Fatal, format_args!($fmt $(, $($arg)+)?), file!(), line!())
    };
}


/// Print log info, alongside with log level, source file and line number.  
/// *Don't call this function. Use marcos instead.*
pub fn monitor_log(args: fmt::Arguments) {
    //可以在这里设置monitor打印的颜色
    let lock = CONSOLE.write();
    set_color(FG_B_MAGENTA,    BG_DEFAULT);
    print(format_args!("[syscall] : {:#?}",args));
    // print(args);
    reset_color();
}

#[macro_export]
macro_rules! monitor_print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::monitor_log( format_args!($fmt $(, $($arg)+)?))
    };
}
