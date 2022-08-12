use alloc::vec::Vec;

use super::File;
use crate::mm::UserBuffer;
use crate::sbi::console_getchar;
use crate::task::suspend_current_and_run_next;

pub struct Stdin;

pub struct Stdout;

const LF: usize = 0x0a;
const CR: usize = 0x0d;
// const DL: usize = 0x7f;
// const BS: usize = 0x08;

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
        let mut c: usize;
        let mut count: usize = 0;
        let mut buf = Vec::new();
        while count < user_buf.len() {
            c = console_getchar();
            match c {
                // `c > 255`是为了兼容OPENSBI，OPENSBI未获取字符时会返回-1
                0 | 256.. => {
                    suspend_current_and_run_next();
                    continue;
                }
                LF | CR => {
                    buf.push(CR as u8);
                    count += 1;
                    break;
                }
                _ => {
                    buf.push(c as u8);
                    count += 1;
                }
            }
        }
        user_buf.write(buf.as_slice());
        count
    }
    fn write(&self, _user_buf: UserBuffer) -> usize {
        panic!("Cannot write to stdin!");
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
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
        for buffer in user_buf.bufvec.bufs[0..user_buf.bufvec.sz].iter() {
            print!("{}", core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(buffer.0 as *const u8, buffer.1 - buffer.0)
            }).unwrap());
        }
        user_buf.len()
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
}
