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
    sys_pipe2(pipe_fd)
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

pub fn str2args(s: &str) -> (Vec<String>, Vec<*const u8>) {
    let args_copy: Vec<String> = s
        .split(' ')
        .map(|s1| {
            let mut string = String::new();
            string.push_str(&s1);
            string.push('\0');
            string
        })
        .collect();

    let mut args_addr: Vec<*const u8> = args_copy.iter().map(|arg| arg.as_ptr()).collect();
    args_addr.push(core::ptr::null::<u8>());

    (args_copy, args_addr)
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

pub fn load_libc_test_cmds() -> Vec<String> {
    let mut cmds = Vec::new();
    cmds.push(String::from("./runtest.exe -w entry-static.exe argv"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe basename"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe clocale_mbfuncs"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe clock_gettime"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe crypt"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe dirname"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe env"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fdopen"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fnmatch"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fscanf"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fwscanf"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe iconv_open"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe inet_pton"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe mbc"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe memstream"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_cancel_points"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_cancel"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_cond"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_tsd"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe qsort"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe random"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe search_hsearch"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe search_insque"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe search_lsearch"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe search_tsearch"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe setjmp"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe snprintf"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe socket"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe sscanf"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe sscanf_long"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe stat"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strftime"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_memcpy"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_memmem"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_memset"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_strchr"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_strcspn"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe string_strstr"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strptime"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strtod"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strtod_simple"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strtof"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strtol"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strtold"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe swprintf"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe tgmath"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe time"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe tls_align"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe udiv"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe ungetc"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe utime"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe wcsstr"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe wcstol"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pleval"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe daemon_failure"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe dn_expand_empty"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe dn_expand_ptr_0"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fflush_exit"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fgets_eof"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fgetwc_buffering"));
    // cmds.push(String::from("./runtest.exe -w entry-static.exe flockfile_list"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe fpclassify_invalid_ld80"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe ftello_unflushed_append"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe getpwnam_r_crash"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe getpwnam_r_errno"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe iconv_roundtrips"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe inet_ntop_v4mapped"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe inet_pton_empty_last_field"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe iswspace_null"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe lrand48_signextend"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe lseek_large"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe malloc_0"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe mbsrtowcs_overflow"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe memmem_oob_read"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe memmem_oob"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe mkdtemp_failure"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe mkstemp_failure"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe printf_1e9_oob"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe printf_fmt_g_round"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe printf_fmt_g_zeros"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe printf_fmt_n"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_robust_detach"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_cancel_sem_wait"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_cond_smasher"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_condattr_setclock"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_exit_cancel"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_once_deadlock"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe pthread_rwlock_ebusy"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe putenv_doublefree"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regex_backref_0"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regex_bracket_icase"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regex_ere_backref"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regex_escaped_high_byte"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regex_negated_range"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe regexec_nosub"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe rewind_clear_error"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe rlimit_open_files"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe scanf_bytes_consumed"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe scanf_match_literal_eof"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe scanf_nullbyte_char"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe setvbuf_unget"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe sigprocmask_internal"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe sscanf_eof"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe statvfs"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe strverscmp"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe syscall_sign_extend"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe uselocale_0"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe wcsncpy_read_overflow"));
    cmds.push(String::from("./runtest.exe -w entry-static.exe wcsstr_false_negative"));

    // ----------------- dynamic -----------------------
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe argv"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe basename"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe clocale_mbfuncs"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe clock_gettime"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe crypt"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe dirname"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe dlopen"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe env"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fdopen"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fnmatch"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fscanf"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fwscanf"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe iconv_open"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe inet_pton"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe mbc"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe memstream"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_cancel_points"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_cancel"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_cond"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_tsd"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe qsort"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe random"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe search_hsearch"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe search_insque"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe search_lsearch"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe search_tsearch"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe sem_init"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe setjmp"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe snprintf"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe socket"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe sscanf"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe sscanf_long"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe stat"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strftime"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_memcpy"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_memmem"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_memset"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_strchr"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_strcspn"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe string_strstr"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strptime"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strtod"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strtod_simple"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strtof"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strtol"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strtold"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe swprintf"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe tgmath"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe time"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe tls_init"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe tls_local_exec"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe udiv"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe ungetc"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe utime"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe wcsstr"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe wcstol"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe daemon_failure"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe dn_expand_empty"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe dn_expand_ptr_0"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fflush_exit"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fgets_eof"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fgetwc_buffering"));
    // cmds.push(String::from("./runtest.exe -w entry-dynamic.exe flockfile_list"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe fpclassify_invalid_ld80"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe ftello_unflushed_append"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe getpwnam_r_crash"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe getpwnam_r_errno"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe iconv_roundtrips"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe inet_ntop_v4mapped"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe inet_pton_empty_last_field"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe iswspace_null"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe lrand48_signextend"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe lseek_large"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe malloc_0"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe mbsrtowcs_overflow"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe memmem_oob_read"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe memmem_oob"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe mkdtemp_failure"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe mkstemp_failure"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe printf_1e9_oob"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe printf_fmt_g_round"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe printf_fmt_g_zeros"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe printf_fmt_n"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_robust_detach"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_cond_smasher"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_condattr_setclock"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_exit_cancel"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_once_deadlock"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe pthread_rwlock_ebusy"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe putenv_doublefree"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regex_backref_0"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regex_bracket_icase"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regex_ere_backref"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regex_escaped_high_byte"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regex_negated_range"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe regexec_nosub"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe rewind_clear_error"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe rlimit_open_files"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe scanf_bytes_consumed"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe scanf_match_literal_eof"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe scanf_nullbyte_char"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe setvbuf_unget"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe sigprocmask_internal"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe sscanf_eof"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe statvfs"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe strverscmp"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe syscall_sign_extend"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe tls_get_new_dtv"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe uselocale_0"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe wcsncpy_read_overflow"));
    cmds.push(String::from("./runtest.exe -w entry-dynamic.exe wcsstr_false_negative"));
    cmds
}

pub fn libc_test() {
    let mut buf = [0u8; 4096];
    let fd = open("run-static.sh", OpenFlags::RDONLY);
    read(fd as _, &mut buf);

    let cmds = load_libc_test_cmds();

    for cmd in cmds {
        let (args_copy, args_addr) = str2args(&cmd);
        let pid = fork();
        if pid == 0 {
            exec(args_copy[0].as_str(), args_addr.as_slice());
        } else {
            let mut exit_code = 0;
            waitpid(pid as usize, &mut exit_code);
        }
    }
}
