#![allow(unused)]

pub const MAX_SYSCALL_NUM: usize = 0x10000;
pub const SYSCALL_GETCWD: usize = 17;
pub const SYSCALL_DUP: usize = 23;
pub const SYSCALL_DUP3: usize = 24;
pub const SYSCALL_FCNTL: usize = 25;
pub const SYSCALL_IOCTL: usize = 29;
pub const SYSCALL_MKDIRAT: usize = 34;
pub const SYSCALL_UNLINKAT: usize = 35;
pub const SYSCALL_LINKAT: usize = 37;
pub const SYSCALL_UMOUNT2: usize = 39;
pub const SYSCALL_MOUNT: usize = 40;
pub const SYSCALL_STATFS: usize = 43;
pub const SYSCALL_FACCESSAT: usize = 48;
pub const SYSCALL_CHDIR: usize = 49;
pub const SYSCALL_OPENAT: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_PIPE2: usize = 59;
pub const SYSCALL_GETDENTS64: usize = 61;
pub const SYSCALL_LSEEK: usize = 62;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_READV: usize = 65;
pub const SYSCALL_WRITEV: usize = 66;
pub const SYSCALL_PREAD64: usize = 67;
pub const SYSCALL_SENDFILE: usize = 71;
pub const SYSCALL_PSELECT6: usize = 72;
pub const SYSCALL_PPOLL: usize = 73;
pub const SYSCALL_READLINKAT: usize = 78;
pub const SYSCALL_FSTATAT: usize = 79;
pub const SYSCALL_FSTAT: usize = 80;
pub const SYSCALL_FSYNC: usize = 82;
pub const SYSCALL_UTIMENSAT: usize = 88;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_EXIT_GRUOP: usize = 94;
pub const SYSCALL_SET_TID_ADDRESS: usize = 96;
pub const SYSCALL_FUTEX: usize = 98;
pub const SYSCALL_NANOSLEEP: usize = 101;
pub const SYSCALL_GETITIMER: usize = 102;
pub const SYSCALL_SETITIMER: usize = 103;
pub const SYSCALL_CLOCK_GETTIME: usize = 113;
pub const SYSCALL_SYSLOG: usize = 116;
pub const SYSCALL_SCHED_YIELD: usize = 124;
pub const SYSCALL_KILL: usize = 129;
pub const SYSCALL_TKILL: usize = 130;
pub const SYSCALL_SIGACTION: usize = 134;
pub const SYSCALL_SIGPROCMASK: usize = 135;
pub const SYSCALL_SIGRETURN: usize = 139;
pub const SYSCALL_TIMES: usize = 153;
pub const SYSCALL_SETPGID: usize = 154;
pub const SYSCALL_GETPGID: usize = 155;
pub const SYSCALL_UNAME: usize = 160;
pub const SYSCALL_GETRUSAGE: usize = 165;
pub const SYSCALL_GETTIMEOFDAY: usize = 169;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_GETPPID: usize = 173;
pub const SYSCALL_GETUID: usize = 174;
pub const SYSCALL_GETEUID: usize = 175;
pub const SYSCALL_GETGID: usize = 176;
pub const SYSCALL_GETEGID: usize = 177;
pub const SYSCALL_GETTID: usize = 178;
pub const SYSCALL_SYSINFO: usize = 179;
pub const SYSCALL_SENDTO: usize = 206;
pub const SYSCALL_RECVFROM: usize = 207;
pub const SYSCALL_SBRK: usize = 213;
pub const SYSCALL_BRK: usize = 214;
pub const SYSCALL_MUNMAP: usize = 215;
pub const SYSCALL_CLONE: usize = 220;
pub const SYSCALL_EXECVE: usize = 221;
pub const SYSCALL_MMAP: usize = 222;
pub const SYSCALL_MPROTECT: usize = 226;
pub const SYSCALL_WAIT4: usize = 260;
pub const SYSCALL_PRLIMIT: usize = 261;
pub const SYSCALL_RENAMEAT2: usize = 276;

pub const SYSCALL_TOGGLE_TRACE: usize = 0xf000;
pub const SYSCALL_READDIR: usize = 0xf001;
pub const SYSCALL_SHUTDOWN: usize = 0xffff;

mod errorno;
mod fs;
mod net;
mod process;
mod signal;
mod sync;

pub use fs::*;
pub use net::*;
pub use process::*;
pub use signal::*;
pub use sync::*;
pub use errorno::*;

use crate::{
    gdb_println,
    monitor::{QEMU, SYSCALL_ENABLE},
    task::current_trap_cx,
};

pub static mut SYSCALL_TABLE: [usize; MAX_SYSCALL_NUM] = [0; MAX_SYSCALL_NUM];

pub fn init() {
    unsafe {
        SYSCALL_TABLE.iter_mut().for_each(|x| *x = sys_unknown as usize);
        SYSCALL_TABLE[SYSCALL_GETCWD] = sys_getcwd as usize;
        SYSCALL_TABLE[SYSCALL_DUP] = sys_dup as usize;
        SYSCALL_TABLE[SYSCALL_DUP3] = sys_dup3 as usize;
        SYSCALL_TABLE[SYSCALL_FCNTL] = sys_fcntl as usize;
        SYSCALL_TABLE[SYSCALL_IOCTL] = sys_ioctl as usize;
        SYSCALL_TABLE[SYSCALL_MKDIRAT] = sys_mkdirat as usize;
        SYSCALL_TABLE[SYSCALL_UNLINKAT] = sys_unlinkat as usize;
        SYSCALL_TABLE[SYSCALL_UMOUNT2] = sys_umount as usize;
        SYSCALL_TABLE[SYSCALL_MOUNT] = sys_mount as usize;
        SYSCALL_TABLE[SYSCALL_STATFS] = sys_statfs as usize;
        SYSCALL_TABLE[SYSCALL_FACCESSAT] = sys_faccessat as usize;
        SYSCALL_TABLE[SYSCALL_CHDIR] = sys_chdir as usize;
        SYSCALL_TABLE[SYSCALL_OPENAT] = sys_open_at as usize;
        SYSCALL_TABLE[SYSCALL_CLOSE] = sys_close as usize;
        SYSCALL_TABLE[SYSCALL_PIPE2] = sys_pipe2 as usize;
        SYSCALL_TABLE[SYSCALL_GETDENTS64] = sys_getdents64 as usize;
        SYSCALL_TABLE[SYSCALL_LSEEK] = sys_lseek as usize;
        SYSCALL_TABLE[SYSCALL_READ] = sys_read as usize;
        SYSCALL_TABLE[SYSCALL_WRITE] = sys_write as usize;
        SYSCALL_TABLE[SYSCALL_READV] = sys_readv as usize;
        SYSCALL_TABLE[SYSCALL_WRITEV] = sys_writev as usize;
        SYSCALL_TABLE[SYSCALL_PREAD64] = sys_pread64 as usize;
        SYSCALL_TABLE[SYSCALL_SENDFILE] = sys_sendfile as usize;
        SYSCALL_TABLE[SYSCALL_PSELECT6] = sys_pselect as usize;
        SYSCALL_TABLE[SYSCALL_PPOLL] = sys_ppoll as usize;
        SYSCALL_TABLE[SYSCALL_READLINKAT] = sys_readlinkat as usize;
        SYSCALL_TABLE[SYSCALL_FSTATAT] = sys_fstatat as usize;
        SYSCALL_TABLE[SYSCALL_FSTAT] = sys_fstat as usize;
        SYSCALL_TABLE[SYSCALL_UTIMENSAT] = sys_utimensat as usize;
        SYSCALL_TABLE[SYSCALL_EXIT] = sys_exit as usize;
        SYSCALL_TABLE[SYSCALL_EXIT_GRUOP] = sys_exit_group as usize;
        SYSCALL_TABLE[SYSCALL_SET_TID_ADDRESS] = sys_set_tid_address as usize;
        SYSCALL_TABLE[SYSCALL_FUTEX] = sys_futex as usize;
        SYSCALL_TABLE[SYSCALL_NANOSLEEP] = sys_sleep as usize;
        SYSCALL_TABLE[SYSCALL_GETITIMER] = sys_getitimer as usize;
        SYSCALL_TABLE[SYSCALL_SETITIMER] = sys_setitimer as usize;
        SYSCALL_TABLE[SYSCALL_CLOCK_GETTIME] = sys_clock_get_time as usize;
        SYSCALL_TABLE[SYSCALL_SYSLOG] = sys_syslog as usize;
        SYSCALL_TABLE[SYSCALL_SCHED_YIELD] = sys_yield as usize;
        SYSCALL_TABLE[SYSCALL_KILL] = sys_kill as usize;
        SYSCALL_TABLE[SYSCALL_TKILL] = sys_tkill as usize;
        SYSCALL_TABLE[SYSCALL_SIGACTION] = sys_sigaction as usize;
        SYSCALL_TABLE[SYSCALL_SIGPROCMASK] = sys_sigprocmask as usize;
        SYSCALL_TABLE[SYSCALL_SIGRETURN] = sys_sigreturn as usize;
        SYSCALL_TABLE[SYSCALL_TIMES] = sys_times as usize;
        SYSCALL_TABLE[SYSCALL_SETPGID] = sys_setpgid as usize;
        SYSCALL_TABLE[SYSCALL_GETPGID] = sys_getpgid as usize;
        SYSCALL_TABLE[SYSCALL_UNAME] = sys_uname as usize;
        SYSCALL_TABLE[SYSCALL_GETTIMEOFDAY] = sys_get_time as usize;
        SYSCALL_TABLE[SYSCALL_GETPID] = sys_getpid as usize;
        SYSCALL_TABLE[SYSCALL_GETPPID] = sys_getppid as usize;
        SYSCALL_TABLE[SYSCALL_GETUID] = sys_getuid as usize;
        SYSCALL_TABLE[SYSCALL_GETTID] = sys_gettid as usize;
        SYSCALL_TABLE[SYSCALL_SYSINFO] = sys_sysinfo as usize;
        SYSCALL_TABLE[SYSCALL_SENDTO] = sys_sendto as usize;
        SYSCALL_TABLE[SYSCALL_RECVFROM] = sys_recvfrom as usize;
        SYSCALL_TABLE[SYSCALL_BRK] = sys_brk as usize;
        SYSCALL_TABLE[SYSCALL_MUNMAP] = sys_munmap as usize;
        SYSCALL_TABLE[SYSCALL_CLONE] = sys_clone as usize;
        SYSCALL_TABLE[SYSCALL_EXECVE] = sys_exec as usize;
        SYSCALL_TABLE[SYSCALL_MMAP] = sys_mmap as usize;
        SYSCALL_TABLE[SYSCALL_MPROTECT] = sys_mprotect as usize;
        SYSCALL_TABLE[SYSCALL_WAIT4] = sys_waitpid as usize;
        SYSCALL_TABLE[SYSCALL_PRLIMIT] = sys_prlimit as usize;
        SYSCALL_TABLE[SYSCALL_RENAMEAT2] = sys_renameat2 as usize;
        SYSCALL_TABLE[SYSCALL_TOGGLE_TRACE] = sys_toggle_trace as usize;
        SYSCALL_TABLE[SYSCALL_READDIR] = sys_readdir as usize;
        SYSCALL_TABLE[SYSCALL_SHUTDOWN] = sys_shutdown as usize;
    }
}
