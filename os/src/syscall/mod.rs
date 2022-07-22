// #![allow(unused)]

const SYSCALL_GETCWD: usize = 17;
const SYSCALL_DUP: usize = 23;
const SYSCALL_DUP3: usize = 24;
const SYSCALL_FCNTL: usize = 25;
const SYSCALL_IOCTL: usize = 29;
const SYSCALL_MKDIRAT: usize = 34;
const SYSCALL_UNLINKAT: usize = 35;
const SYSCALL_LINKAT: usize = 37;
const SYSCALL_UMOUNT2: usize = 39;
const SYSCALL_MOUNT: usize = 40;
const SYSCALL_STATFS: usize = 43;
const SYSCALL_FACCESSAT: usize = 48;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE2: usize = 59;
const SYSCALL_GETDENTS64: usize = 61;
const SYSCALL_LSEEK: usize = 62;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_READV: usize = 65;
const SYSCALL_WRITEV: usize = 66;
const SYSCALL_PREAD64: usize = 67;
const SYSCALL_SENDFILE: usize = 71;
const SYSCALL_PSELECT6: usize = 72;
const SYSCALL_PPOLL: usize = 73;
const SYSCALL_READLINKAT: usize = 78;
const SYSCALL_FSTATAT: usize = 79;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_FSYNC: usize = 82;
const SYSCALL_UTIMENSAT: usize = 88;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GRUOP: usize = 94;
const SYSCALL_SET_TID_ADDRESS: usize = 96;
const SYSCALL_FUTEX: usize = 98;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_GETITIMER: usize = 102;
const SYSCALL_SETITIMER: usize = 103;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SYSLOG: usize = 116;
const SYSCALL_SCHED_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGPROCMASK: usize = 135;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_SETPGID: usize = 154;
const SYSCALL_GETPGID: usize = 155;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GETRUSAGE: usize = 165;
const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_GETUID: usize = 174;
const SYSCALL_GETEUID: usize = 175;
const SYSCALL_GETGID: usize = 176;
const SYSCALL_GETEGID: usize = 177;
const SYSCALL_GETTID: usize = 178;
const SYSCALL_SYSINFO: usize = 179;
const SYS_SENDTO: usize = 206;
const SYS_RECVFROM: usize = 207;
const SYSCALL_SBRK: usize = 213;
const SYSCALL_BRK: usize = 214;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_CLONE: usize = 220;
const SYSCALL_EXECVE: usize = 221;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MPROTECT: usize = 226;
const SYSCALL_WAIT4: usize = 260;
const SYSCALL_PRLIMIT: usize = 261;
const SYSCALL_RENAMEAT2: usize = 276;

pub const SYSCALL_TOGGLE_TRACE: usize = 0xf000;
pub const SYSCALL_READDIR: usize = 0xf001;
pub const SYSCALL_SHUTDOWN: usize = 0xffff;

mod errorno;
mod fs;
mod process;
mod sync;
mod net;

pub use fs::*;
pub use process::*;
pub use sync::*;
pub use net::*;

use crate::{
    gdb_println,
    monitor::{QEMU, SYSCALL_ENABLE},
};

pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    if syscall_id != SYSCALL_READ && syscall_id != SYSCALL_WRITE && syscall_id != SYSCALL_READDIR {
        gdb_println!(
            SYSCALL_ENABLE,
            "\x1b[034msyscall({}), args = {:x?}\x1b[0m",
            syscall_id,
            args
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
        SYSCALL_SIGPROCMASK => 0,
        SYSCALL_SIGRETURN => sys_sigreturn(),
        SYSCALL_TIMES => sys_times(args[0] as _),
        SYSCALL_SETPGID => sys_setpgid(),
        SYSCALL_GETPGID => sys_getpgid(),
        SYSCALL_UNAME => sys_uname(args[0] as _),
        SYSCALL_GETTIMEOFDAY => sys_get_time(args[0] as _, args[1]),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_GETTID => sys_getpid(),
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
        SYS_SENDTO => sys_sendto(args[0] as _, args[1] as _, args[2] as _, args[3] as _, args[4] as _, args[5] as _),
        SYS_RECVFROM => sys_recvfrom(args[0] as _, args[1] as _, args[2] as _, args[3] as _, args[4] as _, args[5] as _),
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
