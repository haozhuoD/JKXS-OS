#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

extern crate alloc;
#[macro_use]
extern crate bitflags;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
use syscall::*;

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

#[global_allocator]
static HEAP: LockedHeap<32> = LockedHeap::empty();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start(argc: usize, argv: usize) -> ! {
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    }
    let mut v: Vec<&'static str> = Vec::new();
    for i in 0..argc {
        let str_start =
            unsafe { ((argv + i * core::mem::size_of::<usize>()) as *const usize).read_volatile() };
        let len = (0usize..)
            .find(|i| unsafe { ((str_start + *i) as *const u8).read_volatile() == 0 })
            .unwrap();
        v.push(
            core::str::from_utf8(unsafe {
                core::slice::from_raw_parts(str_start as *const u8, len)
            })
            .unwrap(),
        );
    }
    exit(main(argc, v.as_slice()));
}

#[linkage = "weak"]
#[no_mangle]
fn main(_argc: usize, _argv: &[&str]) -> i32 {
    panic!("Cannot find main!");
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const CREATE = 1 << 9;
        const TRUNC = 1 << 10;
    }
}

#[derive(Copy, Clone)]
pub struct TimeVal{
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal{
    pub fn new() -> Self{
        Self{
            sec:0,
            usec:0
        }
    }

    pub fn add_usec(&mut self, usec:usize){
        self.usec += usec;
        self.sec += self.usec/1000_000;
        self.usec %= 1000_000;
    }

    pub fn is_zero(&self) -> bool{
        self.sec == 0 && self.usec == 0
    }

}

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}

const AT_FDCWD: isize = -100;

pub fn open(path: &str, flags: OpenFlags) -> isize {
    sys_open_at(AT_FDCWD, path, flags.bits, 2)
}
pub fn close(fd: usize) -> isize {
    sys_close(fd)
}
pub fn pipe(pipe_fd: &mut [usize]) -> isize {
    sys_pipe(pipe_fd)
}
pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    let mut time = TimeVal::new();
    sys_get_time(&mut time);
    (time.sec*1000 + time.usec/1000) as isize
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn exec(path: &str, args: &[*const u8]) -> isize {
    sys_exec(path, args)
}

const WNOHANG: isize = 1;

pub fn wait(wstatus: &mut i32) -> isize {
    sys_waitpid(-1, wstatus as *mut _, 0)
}

pub fn waitpid(pid: usize, wstatus: &mut i32) -> isize {
    sys_waitpid(pid as isize, wstatus as *mut _, 0)
}

pub fn waitpid_nb(pid: usize, exit_code: &mut i32) -> isize {
    sys_waitpid(pid as isize, exit_code as *mut _, WNOHANG)
}

bitflags! {
    pub struct SignalFlags: i32 {
        const SIGINT    = 1 << 2;
        const SIGILL    = 1 << 4;
        const SIGABRT   = 1 << 6;
        const SIGFPE    = 1 << 8;
        const SIGSEGV   = 1 << 11;
    }
}

pub fn kill(pid: usize, signal: i32) -> isize {
    sys_kill(pid, signal)
}

pub fn sleep(_sleep_ms: usize) {
    panic!("outdated function");
    // sys_sleep(sleep_ms);
}

pub fn gettid() -> isize {
    sys_gettid()
}
// pub fn waittid(tid: usize) -> isize {
//     loop {
//         match sys_waittid(tid) {
//             -2 => {
//                 yield_();
//             }
//             exit_code => return exit_code,
//         }
//     }
// }
pub fn brk(addr: usize) -> isize {
    sys_brk(addr)
}

pub fn shutdown() -> ! {
    sys_shutdown()
}

pub fn toggle_trace() -> isize {
    sys_toggle_trace()
}