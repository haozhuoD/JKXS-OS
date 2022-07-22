use core::arch::asm;
use core::mem::size_of;
use core::slice::from_raw_parts;

use crate::config::{aligned_up, PAGE_SIZE, FDMAX};
use crate::console::{
    clear_log_buf, read_all_log_buf, read_clear_log_buf, read_log_buf, unread_size, LOG_BUF_LEN,
};
use crate::fs::{open_file, OpenFlags};
use crate::gdb_println;
use crate::loader::get_usershell_binary;
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PTEFlags,
    UserBuffer, VirtAddr, VirtPageNum,
};
use crate::monitor::{QEMU, SYSCALL_ENABLE};
use crate::sbi::shutdown;
use crate::task::{
    current_process, current_task, current_user_token, exit_current_and_run_next, is_signal_valid,
    pid2process, suspend_current_and_run_next, SigAction,SIG_DFL,
};
use crate::timer::{get_time_ns, get_time_us, NSEC_PER_SEC, USEC_PER_SEC};
use crate::trap::page_fault_handler;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use fat32_fs::sync_all;

use super::errorno::{EINVAL, EPERM};

pub fn sys_shutdown() -> ! {
    sync_all();
    shutdown();
}

pub fn sys_toggle_trace() -> isize {
    unsafe { *(SYSCALL_ENABLE as *mut u8) = 1 - *(SYSCALL_ENABLE as *mut u8) };
    0
}

pub fn sys_exit(exit_code: i32) -> ! {
    gdb_println!(SYSCALL_ENABLE, "sys_exit(exit_code: {} ) = ?", exit_code);
    exit_current_and_run_next(exit_code, false);
}

pub fn sys_exit_group(exit_code: i32) -> ! {
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_exit_group(exit_code: {} ) = ?",
        exit_code
    );
    exit_current_and_run_next(exit_code, true);
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *mut u64, _tz: usize) -> isize {
    let token = current_user_token();
    let curtime = get_time_us();
    *translated_refmut(token, ts) = (curtime / USEC_PER_SEC) as u64;
    *translated_refmut(token, unsafe { ts.add(1) }) = (curtime % USEC_PER_SEC) as u64;
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_get_time(ts: {:#x?}, tz = {:x?} ) = 0",
        ts,
        _tz
    );
    0
}

pub fn sys_getpid() -> isize {
    let ret = current_task().unwrap().process.upgrade().unwrap().getpid() as isize;
    gdb_println!(SYSCALL_ENABLE, "sys_getpid() = {}", ret);
    ret
}

bitflags! {
    pub struct CloneFlags: u32 {
        const SIGCHLD              = 17;
        const CLONE_VM             = 0x00000100;
        const CLONE_FS             = 0x00000200;
        const CLONE_FILES          = 0x00000400;
        const CLONE_SIGHAND        = 0x00000800;
        const CLONE_PIDFD          = 0x00001000;
        const CLONE_PTRACE         = 0x00002000;
        const CLONE_VFORK          = 0x00004000;
        const CLONE_PARENT         = 0x00008000;
        const CLONE_THREAD         = 0x00010000;
        const CLONE_NEWNS          = 0x00020000;
        const CLONE_SYSVSEM        = 0x00040000;
        const CLONE_SETTLS         = 0x00080000;
        const CLONE_PARENT_SETTID  = 0x00100000;
        const CLONE_CHILD_CLEARTID = 0x00200000;
        const CLONE_DETACHED       = 0x00400000;
        const CLONE_UNTRACED       = 0x00800000;
        const CLONE_CHILD_SETTID   = 0x01000000;
        const CLONE_NEWCGROUP      = 0x02000000;
        const CLONE_NEWUTS         = 0x04000000;
        const CLONE_NEWIPC         = 0x08000000;
        const CLONE_NEWUSER        = 0x10000000;
        const CLONE_NEWPID         = 0x20000000;
        const CLONE_NEWNET         = 0x40000000;
        const CLONE_IO             = 0x80000000;
    }
}

pub fn sys_clone(
    flags: u32,
    child_stack: *const u8,
    ptid: usize,
    ctid: *mut u32,
    newtls: usize,
) -> isize {
    let current_process = current_process();
    let flags = CloneFlags::from_bits(flags).unwrap();
    let new_process = current_process.fork(flags, child_stack as usize, newtls);
    let new_pid = new_process.getpid();

    if flags.contains(CloneFlags::CLONE_CHILD_SETTID) && ctid as usize != 0 {
        *translated_refmut(
            new_process.acquire_inner_lock().get_user_token(),
            ctid,
        ) = new_pid as u32;
    }
    // // modify trap context of new_task, because it returns immediately after switching
    // let new_process_inner = new_process.inner_exclusive_access();
    // let task = new_process_inner.tasks[0].as_ref().unwrap();
    // let trap_cx = task.inner_exclusive_access().get_trap_cx();
    // // we do not have to move to next instruction since we have done it before
    // // for child process, fork returns 0
    // trap_cx.x[10] = 0;
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_clone(flags: {:#x?}, child_stack: {:#x?}, ptid: {:#x?}, ctid: {:#x?}, newtls: {:#x?}) = {}",
        flags,
        child_stack,
        ptid,
        ctid,
        newtls,
        new_pid
    );
    unsafe {
        asm!("sfence.vma");
        asm!("fence.i");
    }
    new_pid as isize
}

pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = current_user_token();
    let mut path = translated_str(token, path);
    let mut args_vec: Vec<String> = Vec::new();

    loop {
        let arg_str_ptr = *translated_ref(token, args);
        if arg_str_ptr == 0 {
            break;
        }
        args_vec.push(translated_str(token, arg_str_ptr as *const u8));
        unsafe {
            args = args.add(1);
        }
    }

    let cwd = current_process().acquire_inner_lock().cwd.clone();

    // run usershell
    if cwd == "/" && path == "user_shell" {
        let process = current_process();
        let exec_ret = process.exec(get_usershell_binary(), args_vec);
        if exec_ret!=0 {
            return -1;
        }
        unsafe {
            asm!("sfence.vma");
            asm!("fence.i");
        }
        return 0;
    }

    // 执行./xxx.sh时，自动转化为 /busybox sh ./xxx.sh
    if path.ends_with(".sh") {
        args_vec.insert(0, String::from("sh"));
        args_vec.insert(0, String::from("/busybox"));
        path = String::from("/busybox");
    }

    // run other programs
    if let Some(app_vfile) = open_file(cwd.as_str(), path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_vfile.read_all();
        let process = current_process();
        let argc = args_vec.len();
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_exec(path: {:?}, args: {:x?} ) = {}",
            path,
            args_vec,
            argc
        );
        let exec_ret = process.exec(all_data.as_slice(), args_vec);
        if exec_ret!=0 {
            return -1;
        }
        unsafe {
            asm!("sfence.vma");
            asm!("fence.i");
        }
        // return argc because cx.x[10] will be covered with it later
        argc as isize
    } else {
        -EPERM
    }
}

const WNOHANG: isize = 1;

/// If there is not a child process whose pid is same as given, return -1.
/// Else if there is a child process but it is still running, return -2.
pub fn sys_waitpid(pid: isize, wstatus: *mut i32, options: isize) -> isize {
    loop {
        let mut found: bool = true; // when WNOHANG is set
        let mut exited: bool = true;
        {
            let process = current_process();
            let mut inner = process.acquire_inner_lock();

            // find a child process
            if !inner
                .children
                .iter()
                .any(|p| pid == -1 || pid as usize == p.getpid())
            {
                found = false;
            }

            // child process exists
            if found {
                let pair = inner.children.iter().enumerate().find(|(_, p)| {
                    // ++++ temporarily access child PCB exclusively
                    p.acquire_inner_lock().is_zombie && (pid == -1 || pid as usize == p.getpid())
                    // ++++ release child PCB
                });
                if let Some((idx, _)) = pair {
                    let child = inner.children.remove(idx);
                    // confirm that child will be deallocated after being removed from children list
                    assert_eq!(Arc::strong_count(&child), 1);
                    let found_pid = child.getpid();
                    // ++++ temporarily access child PCB exclusively
                    let exit_code = child.acquire_inner_lock().exit_code;
                    // ++++ release child PCB
                    if wstatus as usize != 0 {
                        *translated_refmut(inner.memory_set.token(), wstatus) =
                            (exit_code & 0xff) << 8;
                    }
                    gdb_println!(
                        SYSCALL_ENABLE,
                        "sys_waitpid(pid: {}, wstatus: {:#x?}, options: {}) = {}",
                        pid,
                        wstatus,
                        options,
                        found_pid as isize
                    );
                    return found_pid as isize;
                } else {
                    exited = false;
                }
            }
            // ---- release current PCB automatically
        }
        // not found yet
        assert!(!found || !exited);
        if !found || options == WNOHANG {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_waitpid(pid: {}, wstatus: {:#x?}, options: {}) = {}",
                pid,
                wstatus,
                options,
                -EPERM
            );
            return -EPERM;
        }
        suspend_current_and_run_next();
    }
}

pub fn sys_kill(pid: usize, signum: u32) -> isize {
    let signum = signum as usize;
    let ret = if let Some(process) = pid2process(pid) {
        if is_signal_valid(signum) {
            process
                .acquire_inner_lock()
                .signal_info
                .pending_signals
                .push_back(signum);
            0
        } else {
            -EPERM
        }
    } else {
        -EPERM
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_kill(pid: {}, signal: {:#x?}) = {}",
        pid,
        signum,
        ret
    );
    ret
}

pub fn sys_brk(addr: usize) -> isize {
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    // println!("syscall brk addr = {:x?}, base = {:x?}, top = {:x?}", addr, inner.user_heap_base, inner.user_heap_top);
    let ret = if addr == 0 {
        inner.user_heap_top as isize
    } else if addr >= inner.user_heap_base {
        if addr < inner.user_heap_top {
            let prev_top = inner.user_heap_top;
            inner.memory_set.remove_heap_dataframes(prev_top, addr);
        }
        inner.user_heap_top = addr as usize;
        addr as isize
    } else {
        -EPERM
    };
    gdb_println!(SYSCALL_ENABLE, "sys_brk(addr: {:#x?}) = {:#x?}", addr, ret);
    ret
}

pub fn sys_mmap(
    _start: usize,
    len: usize,
    prot: usize,
    flags: usize,
    fd: isize,
    offset: usize,
) -> isize {
    // let start = aligned_up(current_process().acquire_inner_lock().mmap_area_top);
    let start:usize;
    //若起始地址不为0，选择相信传入的起始地址。不做检查
    if _start != 0 {
        start = aligned_up(_start);
    }else {
        start = aligned_up(current_process().acquire_inner_lock().mmap_area_top);
    }
    let aligned_len = aligned_up(len);
    let ret = current_process().mmap(start, aligned_len, prot, flags, fd, offset);

    gdb_println!(SYSCALL_ENABLE, 
        "sys_mmap(aligned_start: {:#x?}, aligned_len: 0x{:x?}, prot: 0x{:x?}, flags: 0x{:x?}, fd: {}, offset: {} ) = {:#x?}",
        start , // start,
        aligned_len,// aligned_len, 
        prot, 
        flags, 
        fd, 
        offset, 
        ret
    );
    ret
}

pub fn sys_munmap(start: usize, _len: usize) -> isize {
    let start = aligned_up(start);

    let ret = current_process().munmap(start, _len);
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_munmap(aligend_start: {:#x?}, len: {}) = {}",
        start,
        _len,
        ret
    );
    ret
}

pub fn sys_getppid() -> isize {
    let parent = current_process()
        .acquire_inner_lock()
        .parent
        .clone()
        .unwrap()
        .upgrade();
    let ret = parent.unwrap().getpid() as isize;
    gdb_println!(SYSCALL_ENABLE, "sys_getppid() = {}", ret);
    ret
}

pub fn sys_getuid() -> isize {
    // only support root user
    gdb_println!(SYSCALL_ENABLE, "sys_getuid() = {}", 0);
    0
}

pub fn sys_times(time: *mut usize) -> isize {
    let token = current_user_token();
    let sec = get_time_us();
    *translated_refmut(token, time) = sec;
    *translated_refmut(token, unsafe { time.add(1) }) = sec;
    *translated_refmut(token, unsafe { time.add(2) }) = sec;
    *translated_refmut(token, unsafe { time.add(3) }) = sec;
    gdb_println!(SYSCALL_ENABLE, "sys_times(time: {:#x?}) = {}", time, 0);
    0
}

pub fn sys_set_tid_address(ptr: *mut usize) -> isize {
    let token = current_user_token();
    *translated_refmut(token, ptr) = current_process().pid.0;
    let ret = current_process().pid.0 as isize;
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_set_tid_address(ptr: {:#x?}) = {}",
        ptr,
        ret
    );
    ret
}

#[repr(packed)]
#[allow(unused)]
pub struct Uname {
    sysname: [u8; 65],
    nodename: [u8; 65],
    release: [u8; 65],
    version: [u8; 65],
    machine: [u8; 65],
    domainname: [u8; 65],
}

impl Uname {
    pub fn new() -> Self {
        Self {
            sysname: Uname::fill_field("Linux"),
            nodename: Uname::fill_field("debian"),
            release: Uname::fill_field("5.10.0-7-riscv64"),
            version: Uname::fill_field("#1 SMP Debian 5.10.40-1 (2021-05-28)"),
            machine: Uname::fill_field("riscv-64"),
            domainname: Uname::fill_field(""),
        }
    }

    pub fn fill_field(s: &str) -> [u8; 65] {
        let mut ret = [0u8; 65];
        for (i, ch) in String::from(s).chars().enumerate() {
            ret[i] = ch as u8;
        }
        ret
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self as *const _ as usize as *const u8, 65 * 6) }
    }
}

pub fn sys_uname(buf: *mut u8) -> isize {
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Uname>());
    let uname = Uname::new();
    let mut userbuf = UserBuffer::new(buf_vec);
    userbuf.write(uname.as_bytes());
    gdb_println!(SYSCALL_ENABLE, "sys_uname(buf: {:#x?}) = {}", buf, 0);
    0
}

pub fn sys_clock_get_time(_clk_id: usize, tp: *mut u64) -> isize {
    // struct timespec {
    //     time_t   tv_sec;        /* seconds */
    //     long     tv_nsec;       /* nanoseconds */
    // };
    let token = current_user_token();
    let curtime = get_time_ns();
    *translated_refmut(token, tp) = (curtime / NSEC_PER_SEC) as u64;
    *translated_refmut(token, unsafe { tp.add(1) }) = (curtime % NSEC_PER_SEC) as u64;
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_clock_get_time(clk_id: {}, tp = {:x?} ) = 0",
        _clk_id,
        tp
    );
    0
}

pub fn sys_sigaction(
    signum: usize,
    sigaction: *const SigAction,
    old_sigaction: *mut SigAction,
) -> isize {
    // todo: 暂不支持sa_flags
    // todo: 支持SIGIGNORE
    let token = current_user_token();
    let process = current_process();
    let mut inner = process.acquire_inner_lock();

    // signum超出范围，返回错误
    if !is_signal_valid(signum) {
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_sigaction(signum: {}, sigaction = {:#x?}, old_sigaction = {:#x?} ) = {}",
            signum,
            sigaction,
            old_sigaction,
            -EINVAL
        );
        return -EINVAL;
    }

    // let sigact = translated_refmut(token, sigaction as *mut SigAction);

    // 当sigaction存在时， 在pcb中注册给定的signaction 
    if sigaction as usize != 0 {
        // 如果旧的sigaction存在，则将它保存到指定位置.否则置为 SIG_DFL
        if let Some(old) = inner.signal_info.sigactions.get(&signum) {
            if old_sigaction as usize != 0 {
                // println!("arg old_sigaction !=0  ");
                *translated_refmut(token, old_sigaction) = (*old).clone();
            }
        }else{
            if old_sigaction as usize != 0 {
                let sigact_old = translated_refmut(token, old_sigaction);
                sigact_old.handler = SIG_DFL;
                sigact_old.sigaction = 0;
                sigact_old.mask = 0;
            }
        }

        //在pcb中注册给定的signaction
        inner
            .signal_info
            .sigactions
            .insert(signum, (*translated_ref(token, sigaction)).clone());
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_sigaction(signum: {}, sigaction = {:#x?}, old_sigaction = {:#x?} ) = {}",
        signum,
        sigaction,// sigact,
        old_sigaction,
        0
    );
    return 0;
}

pub fn sys_sigreturn() -> isize {
    // 恢复之前保存的trap_cx
    current_task()
        .unwrap()
        .acquire_inner_lock()
        .pop_trap_cx();
    gdb_println!(SYSCALL_ENABLE, "sys_sigreturn() = 0");
    return 0;
}

pub fn sys_setpgid() -> isize {
    gdb_println!(SYSCALL_ENABLE, "sys_setpgid(...) = 0");
    0
}

pub fn sys_getpgid() -> isize {
    gdb_println!(SYSCALL_ENABLE, "sys_getpgid(...) = 0");
    0
}

#[repr(packed)]
#[allow(unused)]
pub struct Sysinfo {
    uptime: isize,
    loads: [usize; 3],
    totalram: usize,
    freeram: usize,
    sharedram: usize,
    bufferram: usize,
    totalswap: usize,
    freeswap: usize,
    procs: u16,
    totalhigh: usize,
    freehigh: usize,
    mem_unit: u32,
    _f: [u8; 20 - 2 * size_of::<usize>() - size_of::<u32>()],
}

impl Sysinfo {
    pub fn new() -> Self {
        Self {
            uptime: 0,
            loads: [0; 3],
            totalram: 0,
            freeram: 0,
            sharedram: 0,
            bufferram: 0,
            totalswap: 0,
            freeswap: 0,
            procs: 0,
            totalhigh: 0,
            freehigh: 0,
            mem_unit: 0,
            _f: [0; 20 - 2 * size_of::<usize>() - size_of::<u32>()],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts(self as *const _ as usize as *const u8, size_of::<Sysinfo>()) }
    }
}

pub fn sys_sysinfo(buf: *mut u8) -> isize {
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Sysinfo>());
    let sysinfo = Sysinfo::new();

    let mut userbuf = UserBuffer::new(buf_vec);
    userbuf.write(sysinfo.as_bytes());
    gdb_println!(SYSCALL_ENABLE, "sys_sysinfo(buf: {:#x?}) = {}", buf, 0);
    return 0;
}

// const SYSLOG_ACTION_CLOSE: isize = 0;
// const SYSLOG_ACTION_OPEN: isize = 1;
const SYSLOG_ACTION_READ: isize = 2;
const SYSLOG_ACTION_READ_ALL: isize = 3;
const SYSLOG_ACTION_READ_CLEAR: isize = 4;
const SYSLOG_ACTION_CLEAR: isize = 5;
// const SYSLOG_ACTION_CONSOLE_OFF: isize = 6;
// const SYSLOG_ACTION_CONSOLE_ON: isize = 7;
// const SYSLOG_ACTION_CONSOLE_LEVER: isize = 8;
const SYSLOG_ACTION_SIZE_UNREAD: isize = 9;
const SYSLOG_ACTION_SIZE_BUFFER: isize = 10;

pub fn sys_syslog(_type: isize, bufp: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, bufp, len);
    let mut userbuf = UserBuffer::new(buf_vec);
    let ret = match _type {
        SYSLOG_ACTION_READ => {
            let mut tmp_buf: [u8; LOG_BUF_LEN] = [0; LOG_BUF_LEN];
            let r_sz = read_log_buf(tmp_buf.as_mut_slice(), len);
            userbuf.write(&tmp_buf[0..r_sz]);
            r_sz as isize
        }
        SYSLOG_ACTION_READ_ALL => {
            let mut tmp_buf: [u8; LOG_BUF_LEN] = [0; LOG_BUF_LEN];
            let r_sz = read_all_log_buf(tmp_buf.as_mut_slice(), len);
            userbuf.write(&tmp_buf[0..r_sz]);
            r_sz as isize
        }
        SYSLOG_ACTION_READ_CLEAR => {
            let mut tmp_buf: [u8; LOG_BUF_LEN] = [0; LOG_BUF_LEN];
            let r_sz = read_clear_log_buf(tmp_buf.as_mut_slice(), len);
            userbuf.write(&tmp_buf[0..r_sz]);
            r_sz as isize
        }
        SYSLOG_ACTION_CLEAR => {
            clear_log_buf();
            0
        }
        SYSLOG_ACTION_SIZE_UNREAD => unread_size() as isize,
        SYSLOG_ACTION_SIZE_BUFFER => LOG_BUF_LEN as isize,
        _ => 0,
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_syslog(type: {}, bufp = {:#x?}, len = {} ) = {}",
        _type,
        bufp,
        len,
        ret
    );
    ret
}

pub fn sys_mprotect(addr: usize, len: usize, prot: usize) -> isize {
    if addr % PAGE_SIZE != 0 || len % PAGE_SIZE != 0 {
        warning!("sys_mprotect: not aligned!");
        return -EINVAL;
    }
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    let start_vpn = addr / PAGE_SIZE;
    let flags = PTEFlags::from_bits((prot as u8) << 1).unwrap();

    for i in 0..len / PAGE_SIZE {
        let vpn = VirtPageNum::from(start_vpn + i);
        // 尝试直接改变pte_flags
        if (&mut inner.memory_set).set_pte_flags(vpn, flags) == 0 {
            continue;
        }
        // failed
        if page_fault_handler(&mut inner, VirtAddr::from(vpn).into()) == 0 {
            if (&mut inner.memory_set).set_pte_flags(vpn, flags) == 0 {
                continue;
            }
        }
        panic!("sys_mprotect: No such pte");
    }
    unsafe {
        asm!("sfence.vma");
        asm!("fence.i");
    }
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_mprotect(addr: {:#x?}, len: {}, prot: {:#x?}) = {}",
        addr,
        len,
        prot,
        0
    );
    0
}

// SYSCALL_PRLIMIT
#[derive(Clone,Copy,Debug)]
pub struct RLimit64 {
	pub rlim_cur: usize ,
	pub rlim_max: usize ,
}
// The resource argument :
// const RLIMIT_CPU : usize = 0;
// const RLIMIT_FSIZE : usize = 1;
// const RLIMIT_DATA : usize = 2;
// const RLIMIT_STACK : usize = 3;
// const RLIMIT_CORE : usize = 4;
// const RLIMIT_RSS : usize = 5;
// const RLIMIT_NPROC : usize = 6;
const RLIMIT_NOFILE : usize = 7;
// const RLIMIT_MEMLOCK : usize = 8;
// const RLIMIT_AS : usize = 9;
// const RLIMIT_LOCKS : usize = 10;
// const RLIMIT_SIGPENDING : usize = 11;
// const RLIMIT_MSGQUEUE : usize = 12;
// const RLIMIT_NICE : usize = 13;
// const RLIMIT_RTPRIO : usize = 14;
// const RLIMIT_RTTIME : usize = 15;
// const RLIM_NLIMITS : usize = 16;
/// 仅实现不完整的RLIMIT_NOFILE
pub fn sys_prlimit(pid:usize, resource:usize, rlimit:*const RLimit64, old_rlimit: *mut RLimit64) -> isize {
    let token = current_user_token();
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    let ret = match resource{
        RLIMIT_NOFILE => {
            // 仅仅记录值到inner.fd_max
            if rlimit as usize != 0 {
                let _rlimit = translated_ref(token, rlimit);
                inner.fd_max =  _rlimit.rlim_max - 1;
            }
            if old_rlimit as usize != 0 && inner.fd_max != FDMAX {
                let _old_rlimit = translated_refmut(token, old_rlimit);
                _old_rlimit.rlim_cur = inner.fd_max + 1;
                _old_rlimit.rlim_max = inner.fd_max + 1;
            }
            0
        }
        _ => {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_prlimit() unsuport resource:{}",
                resource,
            );
            0
        }
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_prlimit(pid: {:x?}, resource: {:x?}, rlimit: {:#?}, old_rlimit: {:#?} ) = {}",
        pid,
        resource,
        rlimit,
        old_rlimit,
        ret
    );
    ret
}