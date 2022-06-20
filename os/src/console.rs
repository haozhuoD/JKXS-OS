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

// ======================== kernel log buffer ========================

pub const LOG_BUF_LEN: usize = 4096;
pub struct LogBufInner {
    pub empty:  bool,  // The same values of head and tail mean that the log_buf is either empty or full
    pub clear_empty: bool,
    pub clear_head: usize,  // the head after the last clear command 
    pub head:   usize,
    pub tail:   usize,
    pub inner:  [u8; LOG_BUF_LEN]
}

impl LogBufInner {
    // READ
    pub fn read(&mut self, buf: &mut [u8], len: usize) -> usize {
        let tail = self.tail;
        let head = self.head;
        let r_len = self.unread_size().min(len);
        let r1_len = LOG_BUF_LEN - head;
        if r1_len < r_len {
            buf[0..r1_len].copy_from_slice(&self.inner[head..LOG_BUF_LEN]);
            buf[r1_len..r_len].copy_from_slice(&self.inner[0..r_len-r1_len]);
        } else {
            buf[0..r_len].copy_from_slice(&self.inner[head..head+r_len]);
        }
        // update
        self.head = (head + r_len) % LOG_BUF_LEN;
        let unread_sz = self.unread_size();
        if unread_sz < self.unclear_size() {
            self.clear_head = self.head;
        }
        if unread_sz == 0 {
            self.empty = true;
            self.clear_empty = true;
        }
        return r_len;
    }
    // READ_ALL & READ_CLEAR: depend on the value of "clear"
    // read the last len bytes
    pub fn read_all(&mut self, buf: &mut [u8], len: usize, clear: bool) -> usize {
        let tail = self.tail;
        let clear_head = self.clear_head;
        let r_len = self.unclear_size().min(len);
        if tail > r_len {
            buf[0..r_len].copy_from_slice(&self.inner[tail-r_len..tail]);
        } else {
            let r1_len = r_len - tail;
            buf[0..r1_len].copy_from_slice(&self.inner[LOG_BUF_LEN-r1_len..LOG_BUF_LEN]);
            buf[r1_len..r_len].copy_from_slice(&self.inner[0..tail]);
        }
        // clear
        if (clear) {
            self.clear_buf();
        }
        return r_len;
    }
    // from head to tail
    pub fn unread_size(&self) -> usize {
        let head = self.head;
        let tail = self.tail;
        if head < tail || self.empty {
            tail - head
        } else {
            LOG_BUF_LEN + tail - head
        }
    }
    // from clear_head to tail
    fn unclear_size(&self) -> usize {
        let clear_head = self.clear_head;
        let tail = self.tail;
        if clear_head < tail || self.clear_empty {
            tail - clear_head
        } else {
            LOG_BUF_LEN + tail - clear_head
        }
    }
    // clear the log buffer
    pub fn clear_buf(&mut self) {
        self.clear_head = self.tail;
        self.clear_empty = true;
    }
}

pub fn read_log_buf(buf: &mut [u8], len: usize) -> usize {
    LOG_BUF.write().read(buf, len)
}

pub fn read_all_log_buf(buf: &mut [u8], len: usize) -> usize {
    LOG_BUF.write().read_all(buf, len, false)
}

pub fn read_clear_log_buf(buf: &mut [u8], len: usize) -> usize {
    LOG_BUF.write().read_all(buf, len, true)
}

pub fn clear_log_buf() {
    LOG_BUF.write().clear_buf();
}

pub fn unread_size() -> usize {
    LOG_BUF.read().unread_size()
}

impl Write for LogBufInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let buf = s.as_bytes();
        let w_len;
        let tail = self.tail;
        let head = self.head;
        if head < tail || self.empty {
            w_len = (LOG_BUF_LEN + head - tail).min(buf.len());
        } else {
            w_len = (head - tail).min(buf.len());
        }
        if w_len > 0 {
            self.empty = false;
            self.clear_empty = false;
            let w1_len = LOG_BUF_LEN - tail;
            if w1_len < w_len {
                &self.inner[tail..LOG_BUF_LEN].copy_from_slice(&buf[0..w1_len]);
                &self.inner[0..w_len-w1_len].copy_from_slice(&buf[w1_len..w_len]);
            } else {
                &self.inner[tail..tail+w_len].copy_from_slice(&buf[0..w_len]);
            }   
            // update tail
            self.tail = (tail + w_len) % LOG_BUF_LEN;
        }
        Ok(())
    }
}

pub static LOG_BUF: Lazy<RwLock<LogBufInner>> = Lazy::new(|| RwLock::new(
    LogBufInner {
        empty: true, 
        clear_empty: true,
        clear_head: 0,
        head: 0,
        tail: 0,
        inner: [0; LOG_BUF_LEN] 
    }
));

// ======================== console ========================

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
        print(format_args!("{} {:#?} @ {:#?} : {:#?} \n", log_level, file, line, args));
        reset_color();
        LOG_BUF.write().write_fmt(format_args!("{} {:#?} @ {:#?} : {:#?} \n", log_level, file, line, args));
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
        $crate::console::log(crate::console::LogLevel::Verbose, format_args!($fmt $(, $($arg)+)?), file!(), line!())
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
        $crate::console::log(crate::console::LogLevel::Debug, format_args!($fmt $(, $($arg)+)?), file!(), line!())
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
