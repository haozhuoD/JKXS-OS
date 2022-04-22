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
const SYSCALL_FACCESSAT: usize = 48;
const SYSCALL_CHDIR: usize = 49;
const SYSCALL_OPENAT: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE2: usize = 59;
const SYSCALL_GETDENTS64: usize = 61;
const SYSCALL_LSEEK: usize = 62;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_WRITEV: usize = 66;
const SYSCALL_SENDFILE: usize = 71;
const SYSCALL_PSELECT6: usize = 72;
const SYSCALL_READLINKAT: usize = 78;
const SYSCALL_FSTATAT: usize = 79;
const SYSCALL_FSTAT: usize = 80;
const SYSCALL_FSYNC: usize = 82;
const SYSCALL_UTIMENSAT: usize = 88;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_EXIT_GRUOP: usize = 94;
const SYSCALL_SET_TID_ADDRESS: usize = 96;
const SYSCALL_NANOSLEEP: usize = 101;
const SYSCALL_GETITIMER: usize = 102;
const SYSCALL_SETITIMER: usize = 103;
const SYSCALL_CLOCK_GETTIME: usize = 113;
const SYSCALL_SCHED_YIELD: usize = 124;
const SYSCALL_KILL: usize = 129;
const SYSCALL_SIGACTION: usize = 134;
const SYSCALL_SIGRETURN: usize = 139;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_UNAME: usize = 160;
const SYSCALL_GETRUSAGE: usize = 165;
const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_GETPPID: usize = 173;
const SYSCALL_GETUID: usize = 174;
const SYSCALL_GETEUID: usize = 175;
const SYSCALL_GETGID: usize = 176;
const SYSCALL_GETEGID: usize = 177;
const SYSCALL_GETTID: usize = 177;
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

// Not standard POSIX sys_call
const SYSCALL_THREAD_CREATE: usize = 1000;
const SYSCALL_WAITTID: usize = 1002;
// const SYSCALL_MUTEX_CREATE: usize = 1010;
// const SYSCALL_MUTEX_LOCK: usize = 1011;
// const SYSCALL_MUTEX_UNLOCK: usize = 1012;
// const SYSCALL_SEMAPHORE_CREATE: usize = 1020;
// const SYSCALL_SEMAPHORE_UP: usize = 1021;
// const SYSCALL_SEMAPHORE_DOWN: usize = 1022;
// const SYSCALL_CONDVAR_CREATE: usize = 1033;
// const SYSCALL_CONDVAR_SIGNAL: usize = 1031;
// const SYSCALL_CONDVAR_WAIT: usize = 1032;

mod fs;
mod process;
mod sync;
mod thread;

use fs::*;
use process::*;
use sync::*;
use thread::*;

pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    match syscall_id {
        SYSCALL_GETCWD => sys_getcwd(args[0] as *mut u8, args[1]),
        SYSCALL_DUP => sys_dup(args[0]),
        SYSCALL_DUP3 => sys_dup3(args[0], args[1]),
        SYSCALL_IOCTL => sys_ioctl(),
        SYSCALL_FCNTL => sys_fcntl(),
        SYSCALL_MKDIRAT => sys_mkdirat(args[0] as isize, args[1] as *const u8, args[2] as u32),
        SYSCALL_UNLINKAT=> sys_unlinkat(args[0] as i32, args[1] as *const u8, args[2] as u32),
        SYSCALL_UMOUNT2=> sys_umount(args[0] as *const u8, args[1] as usize),
        SYSCALL_MOUNT=> sys_mount(args[0] as *const u8, args[1] as *const u8, args[2] as *const u8, args[3] as usize, args[4] as *const u8),
        SYSCALL_CHDIR => sys_chdir(args[0] as *const u8),
        SYSCALL_OPENAT => sys_open_at(
            args[0] as isize,
            args[1] as *const u8,
            args[2] as u32,
            args[3] as u32,
        ),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_PIPE2 => sys_pipe(args[0] as *mut u32),
        SYSCALL_GETDENTS64 => sys_getdents64(args[0] as isize, args[1] as *mut u8, args[2]),
        SYSCALL_READ => sys_read(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_WRITEV => sys_writev(args[0], args[1] as *mut crate::fs::IOVec, args[2]),
        SYSCALL_FSTATAT => sys_fstatat(args[0] as isize, args[1] as *mut u8, args[2] as *mut u8),
        SYSCALL_FSTAT => sys_fstat(args[0] as isize, args[1] as *mut u8),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_EXIT_GRUOP => sys_exit_group(args[0] as i32),
        SYSCALL_SET_TID_ADDRESS => sys_set_tid_address(args[0] as *mut usize),
        SYSCALL_NANOSLEEP => sys_sleep(args[0] as *mut u64),
        SYSCALL_CLOCK_GETTIME => sys_clock_get_time(args[0], args[1] as *mut u64),
        SYSCALL_SCHED_YIELD => sys_yield(),
        SYSCALL_KILL => sys_kill(args[0], args[1] as u32),
        SYSCALL_TIMES => sys_times(args[0] as *mut usize),
        SYSCALL_UNAME => sys_uname(args[0] as *mut u8),
        SYSCALL_GETTIMEOFDAY => sys_get_time(args[0] as *mut u64, args[1]),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_BRK => sys_brk(args[0]),
        SYSCALL_CLONE => sys_fork(args[0] as u32, args[1]),
        SYSCALL_EXECVE => sys_exec(args[0] as *const u8, args[1] as *const usize),
        SYSCALL_MMAP => sys_mmap(
            args[0] as usize,
            args[1] as usize,
            args[2] as usize,
            args[3] as usize,
            args[4] as isize,
            args[5] as usize,
        ),
        SYSCALL_MUNMAP => sys_munmap(args[0] as usize, args[1] as usize),
        SYSCALL_WAIT4 => sys_waitpid(args[0] as isize, args[1] as *mut i32, args[2] as isize),
        SYSCALL_THREAD_CREATE => sys_thread_create(args[0], args[1]),
        SYSCALL_GETPPID => sys_getppid(),
        SYSCALL_GETUID => sys_getuid(),
        SYSCALL_GETTID => sys_gettid(),
        SYSCALL_WAITTID => sys_waittid(args[0]) as isize,
        // SYSCALL_MUTEX_CREATE => sys_mutex_create(args[0] == 1),
        // SYSCALL_MUTEX_LOCK => sys_mutex_lock(args[0]),
        // SYSCALL_MUTEX_UNLOCK => sys_mutex_unlock(args[0]),
        // SYSCALL_SEMAPHORE_CREATE => sys_semaphore_create(args[0]),
        // SYSCALL_SEMAPHORE_UP => sys_semaphore_up(args[0]),
        // SYSCALL_SEMAPHORE_DOWN => sys_semaphore_down(args[0]),
        // SYSCALL_CONDVAR_CREATE => sys_condvar_create(args[0]),
        // SYSCALL_CONDVAR_SIGNAL => sys_condvar_signal(args[0]),
        // SYSCALL_CONDVAR_WAIT => sys_condvar_wait(args[0], args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
