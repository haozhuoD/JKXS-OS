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

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use buddy_system_allocator::LockedHeap;
use syscall::*;

const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

const AT_FDCWD: isize = -100;

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
        const _X2 = 1 << 2;
        const _X3 = 1 << 3;
        const _X4 = 1 << 4;
        const _X5 = 1 << 5;
        const CREATE = 1 << 6;
        const EXCL = 1 << 7;
        const _X8 = 1 << 8;
        const TRUNC = 1 << 9;
        const APPEND = 1 << 10;
        const _X11 = 1 << 11;
        const _X12 = 1 << 12;
        const _X13 = 1 << 13;
        const _X14 = 1 << 14;
        const LARGEFILE = 1 << 15;
        const DIRECTORY_ = 1 << 16;
        const _X17 = 1 << 17;
        const _X18 = 1 << 18;
        const CLOEXEC = 1 << 19;
        const _X20 = 1 << 20;
        const DIRECTORY = 1 << 21;
    }
}

#[derive(Copy, Clone)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

impl TimeVal {
    pub fn new() -> Self {
        Self { sec: 0, usec: 0 }
    }

    pub fn add_usec(&mut self, usec: usize) {
        self.usec += usec;
        self.sec += self.usec / 1000_000;
        self.usec %= 1000_000;
    }

    pub fn is_zero(&self) -> bool {
        self.sec == 0 && self.usec == 0
    }
}

#[allow(unused)]
#[repr(packed)]
pub struct FSDirent {
    d_ino: u64,    // 索引结点号
    d_off: i64,    // 到下一个dirent的偏移
    d_reclen: u16, // 当前dirent的长度
    d_type: u8,    // 文件类型
}

pub fn dup(fd: usize) -> isize {
    sys_dup(fd)
}

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
    (time.sec * 1000 + time.usec / 1000) as isize
}
pub fn getpid() -> isize {
    sys_getpid()
}
pub fn fork() -> isize {
    sys_clone()
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

pub fn chdir(path: &str) -> isize {
    sys_chdir(path)
}

pub fn readcwd() -> Vec<String> {
    let mut buf = [0u8; 3000];
    let len = sys_getdents64(AT_FDCWD, &mut buf);
    let dir_size = core::mem::size_of::<FSDirent>();
    let mut start_offset = dir_size;
    let mut end_offset = start_offset;
    let mut dirv: Vec<String> = Vec::new();
    if len > 0 {
        while end_offset <= len as usize {
            while buf[end_offset] != 0 {
                end_offset += 1;
            }
            dirv.push(
                core::str::from_utf8(&buf[start_offset..end_offset])
                    .unwrap()
                    .to_string(),
            );
            end_offset = end_offset + dir_size + 1;
            start_offset = end_offset;
        }
    }
    dirv
}

pub fn change_cwd(cwd: &str, path: &str) -> String {
    if path.starts_with("/") {
        let mut path = path.trim_end_matches("/\0").to_string();
        path.push('/');
        return path;
    }
    let mut cwdv: Vec<&str> = cwd.split("/").filter(|x| *x != "").collect();
    let pathv: Vec<&str> = path
        .split("/")
        .map(|x| x.trim_end_matches("\0"))
        .filter(|x| *x != "")
        .collect();
    for &path_element in pathv.iter() {
        if path_element == "." {
            continue;
        } else if path_element == ".." {
            cwdv.pop();
        } else {
            cwdv.push(path_element);
        }
    }
    let mut cwd = String::from("/");
    for &cwd_element in cwdv.iter() {
        cwd.push_str(cwd_element);
        cwd.push('/');
    }
    cwd
}

pub fn get_wordlist(abs_path: &str) -> Vec<String> {
    let mut buf = [0u8; 3000];
    let mut abs_path = abs_path.to_string();
    abs_path.push('\0');
    let len = sys_readdir(abs_path.as_str(), &mut buf);
    let dir_size = core::mem::size_of::<FSDirent>();
    let mut start_offset = dir_size;
    let mut end_offset = start_offset;
    let mut dirv: Vec<String> = Vec::new();
    if len > 0 {
        while end_offset <= len as usize {
            while buf[end_offset] != 0 {
                end_offset += 1;
            }
            dirv.push(
                core::str::from_utf8(&buf[start_offset..end_offset])
                    .unwrap()
                    .to_string(),
            );
            end_offset = end_offset + dir_size + 1;
            start_offset = end_offset;
        }
    }
    dirv
}

pub fn longest_common_prefix(str_vec: &Vec<String>) -> String {
    str_vec
        .iter()
        .max()
        .unwrap()
        .chars()
        .zip(str_vec.iter().min().unwrap().chars())
        .take_while(|x| x.0 == x.1)
        .map(|x| x.0)
        .collect()
}

pub fn preliminary_test() {
    let mut preliminary_apps = Vec::new();
    preliminary_apps.push("times\0");
    preliminary_apps.push("gettimeofday\0");
    preliminary_apps.push("sleep\0");
    preliminary_apps.push("brk\0");
    preliminary_apps.push("clone\0");
    // preliminary_apps.push("close\0");
    preliminary_apps.push("dup2\0");
    preliminary_apps.push("dup\0");
    preliminary_apps.push("execve\0");
    preliminary_apps.push("exit\0");
    preliminary_apps.push("fork\0");
    preliminary_apps.push("fstat\0");
    preliminary_apps.push("getcwd\0");
    preliminary_apps.push("getdents\0");
    preliminary_apps.push("getpid\0");
    preliminary_apps.push("getppid\0");
    preliminary_apps.push("mkdir_\0");
    preliminary_apps.push("mmap\0");
    preliminary_apps.push("munmap\0");
    preliminary_apps.push("mount\0");
    preliminary_apps.push("openat\0");
    preliminary_apps.push("open\0");
    preliminary_apps.push("pipe\0");
    preliminary_apps.push("read\0");
    preliminary_apps.push("umount\0");
    preliminary_apps.push("uname\0");
    preliminary_apps.push("wait\0");
    preliminary_apps.push("waitpid\0");
    preliminary_apps.push("write\0");
    preliminary_apps.push("yield\0");
    preliminary_apps.push("unlink\0");
    preliminary_apps.push("chdir\0");
    preliminary_apps.push("close\0");

    for app_name in preliminary_apps {
        let pid = fork();
        if pid == 0 {
            exec(app_name, &[core::ptr::null::<u8>()]);
        } else {
            let mut exit_code = 0;
            waitpid(pid as usize, &mut exit_code);
        }
    }
}

pub fn busybox_lua_test() {
    let mut apps = Vec::new();
    apps.push("./busybox_testcode.sh\0");
    apps.push("./lua_testcode.sh\0");
    for app_name in apps {
        let pid = fork();
        if pid == 0 {
            exec(app_name, &[app_name.as_ptr(), core::ptr::null::<u8>()]);
        } else {
            let mut exit_code = 0;
            waitpid(pid as usize, &mut exit_code);
        }
    }
}
