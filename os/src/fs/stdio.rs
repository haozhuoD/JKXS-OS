use alloc::collections::VecDeque;
use alloc::vec::Vec;
use spin::Mutex;
use spin::lazy::Lazy;

use super::{File, Kstat, S_IFCHR};
use crate::mm::UserBuffer;
use crate::sbi::console_getchar;
use crate::task::suspend_current_and_run_next;

pub struct Stdin;

pub struct Stdout;

pub static STDIN_BUF: Lazy<Mutex<VecDeque<u8>>> = Lazy::new(|| Mutex::new(VecDeque::new()));

// const DL: usize = 0x7f;
// const BS: usize = 0x08;

impl Stdin {
    pub fn new() -> Self {
        Self
    }
}

impl File for Stdin {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        false
    }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        // assert_eq!(user_buf.len(), 1);
        // busy loop
        let mut count: usize = 0;
        let mut buf = Vec::new();
        while count < user_buf.len() {
            if !self.read_blocking() {
                let c = STDIN_BUF.lock().pop_front().unwrap();
                buf.push(c);
                count += 1;
            } else {
                suspend_current_and_run_next();
            }
        }
        user_buf.write(buf.as_slice());
        count
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
    fn read_blocking(&self) -> bool {
        let mut stdin_buf_locked = STDIN_BUF.lock();
        if !stdin_buf_locked.is_empty() {
            return false;
        }
        let c = console_getchar();
        match c {
            // `c > 255`是为了兼容OPENSBI，OPENSBI未获取字符时会返回-1
            0 | 256.. => true,
            _ => {
                stdin_buf_locked.push_back(c as u8);
                false
            }
        }
    }
    fn write_blocking(&self) -> bool {
        false
    }
    fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = S_IFCHR;
        kstat
    }
}

impl Stdout {
    pub fn new() -> Self {
        Self
    }
}

impl File for Stdout {
    fn readable(&self) -> bool {
        false
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot read from stdout!");
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        for buffer in user_buf.buffers.iter() {
            print!("{}", core::str::from_utf8(*buffer).unwrap());
        }
        user_buf.len()
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
    fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = S_IFCHR;
        kstat
    }
}
