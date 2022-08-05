#![allow(unused)]

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

use crate::{
    gdb_println,
    monitor::{QEMU, SYSCALL_ENABLE},
};

pub fn syscall(syscall_id: usize, args: [usize; 6], sepc: usize) -> isize {
    if !((syscall_id == SYSCALL_READ || syscall_id == SYSCALL_WRITE) && (args[0] <= 2))
        && syscall_id != SYSCALL_READDIR
    {
        gdb_println!(
            SYSCALL_ENABLE,
            "\x1b[034msyscall({}), args = {:x?}, sepc = {:#x?}\x1b[0m",
            syscall_id,
            args,
            sepc - 4
        );
    }
    match syscall_id {
        SYSCALL_GETCWD => sys_getcwd(args[0] as _, args[1]),
        SYSCALL_DUP => sys_dup(args[0]),
        SYSCALL_DUP3 => sys_dup3(args[0], args[1]),
        SYSCALL_IOCTL => sys_ioctl(),
        SYSCALL_FCNTL => sys_fcntl(args[0], args[1] as _, args[2]),
        SYSCALL_MKDIRAT => sys_mkdirat(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_UNLINKAT => sys_unlinkat(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_UMOUNT2 => sys_umount(args[0] as _, args[1] as _),
        SYSCALL_MOUNT => sys_mount(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
        ),
        SYSCALL_STATFS => sys_statfs(args[0] as _, args[1] as _),
        SYSCALL_FACCESSAT => sys_faccessat(args[0] as _, args[1] as _, args[2], args[3] as _), // fake
        SYSCALL_CHDIR => sys_chdir(args[0] as _),
        SYSCALL_OPENAT => sys_open_at(args[0] as _, args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_PIPE2 => sys_pipe2(args[0] as _, args[1] as u32),
        SYSCALL_GETDENTS64 => sys_getdents64(args[0] as _, args[1] as _, args[2]),
        SYSCALL_LSEEK => sys_lseek(args[0], args[1], args[2]),
        SYSCALL_READ => sys_read(args[0], args[1] as _, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as _, args[2]),
        SYSCALL_READV => sys_readv(args[0], args[1] as _, args[2]),
        SYSCALL_WRITEV => sys_writev(args[0], args[1] as _, args[2]),
        SYSCALL_PREAD64 => sys_pread64(args[0], args[1] as _, args[2], args[3]),
        SYSCALL_SENDFILE => sys_sendfile(args[0], args[1], args[2] as _, args[3]),
        SYSCALL_PPOLL => sys_ppoll(args[0] as _, args[1], args[2] as _),
        SYSCALL_READLINKAT => sys_readlinkat(args[0] as _, args[1] as _, args[2] as _, args[3]),
        SYSCALL_PSELECT6 => sys_pselect(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
        ),
        SYSCALL_FSTATAT => sys_fstatat(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_FSTAT => sys_fstat(args[0] as _, args[1] as _),
        SYSCALL_UTIMENSAT => sys_utimensat(args[0] as _, args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_EXIT => sys_exit(args[0] as _),
        SYSCALL_EXIT_GRUOP => sys_exit_group(args[0] as _),
        SYSCALL_SET_TID_ADDRESS => sys_set_tid_address(args[0] as _),
        SYSCALL_FUTEX => sys_futex(
            args[0] as _,
            args[1],
            args[2] as _,
            args[3] as _,
            args[4] as _,
            args[5] as _,
        ),
        SYSCALL_NANOSLEEP => sys_sleep(args[0] as _),
        SYSCALL_CLOCK_GETTIME => sys_clock_get_time(args[0], args[1] as _),
        SYSCALL_SYSLOG => sys_syslog(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_SCHED_YIELD => sys_yield(),
        SYSCALL_KILL => sys_kill(args[0], args[1] as _),
        SYSCALL_TKILL => sys_tkill(args[0], args[1] as _),
        SYSCALL_SIGACTION => sys_sigaction(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_SIGPROCMASK => sys_sigprocmask(args[0], args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_SIGRETURN => sys_sigreturn(),
        SYSCALL_TIMES => sys_times(args[0] as _),
        SYSCALL_SETPGID => sys_setpgid(),
        SYSCALL_GETPGID => sys_getpgid(),
        SYSCALL_UNAME => sys_uname(args[0] as _),
        SYSCALL_GETTIMEOFDAY => sys_get_time(args[0] as _, args[1]),
        // SYSCALL_GETTIMEOFDAY => sys_get_time_of_day(args[0] as _),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_GETTID => sys_gettid(),
        SYSCALL_BRK => sys_brk(args[0]),
        SYSCALL_CLONE => sys_clone(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
        ),
        SYSCALL_EXECVE => sys_exec(args[0] as _, args[1] as _),
        SYSCALL_MMAP => sys_mmap(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
            args[5] as _,
        ),
        SYSCALL_MPROTECT => sys_mprotect(args[0], args[1], args[2]),
        SYSCALL_MUNMAP => sys_munmap(args[0] as _, args[1] as _),
        SYSCALL_WAIT4 => sys_waitpid(args[0] as _, args[1] as _, args[2] as _),
        SYSCALL_GETPPID => sys_getppid(),
        SYSCALL_GETUID => sys_getuid(),
        SYSCALL_SYSINFO => sys_sysinfo(args[0] as _),
        SYSCALL_RENAMEAT2 => sys_renameat2(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4],
        ),
        SYSCALL_SHUTDOWN => sys_shutdown(),
        SYSCALL_TOGGLE_TRACE => sys_toggle_trace(),
        SYSCALL_READDIR => sys_readdir(args[0] as _, args[1] as _, args[2]),
        SYSCALL_PRLIMIT => sys_prlimit(args[0] as _, args[1] as _, args[2] as _, args[3] as _),
        SYSCALL_SENDTO => sys_sendto(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
            args[5] as _,
        ),
        SYSCALL_RECVFROM => sys_recvfrom(
            args[0] as _,
            args[1] as _,
            args[2] as _,
            args[3] as _,
            args[4] as _,
            args[5] as _,
        ),
        // SYSCALL_GETITIMER=> sys_getitimer(args[0] as _, args[1] as _,),
        // SYSCALL_SETITIMER=>sys_setitimer(args[0] as _, args[1] as _,args[2] as _,),
        _ => {
            gdb_println!(
                SYSCALL_ENABLE,
                "Unsupported syscall_id: {}, args = {:#x?}",
                syscall_id,
                args
            );
            0
        }
    }
}
