use crate::config::aligned_up;
use crate::fs::{open_file, OpenFlags};
use crate::mm::{translated_ref, translated_refmut, translated_str};
use crate::task::{
    current_process, current_task, current_user_token, exit_current_and_run_next, pid2process,
    suspend_current_and_run_next, SignalFlags,
};
use crate::timer::{get_time_us, USEC_PER_SEC};
use crate::{gdb_println, monitor::*};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;

pub fn sys_exit(exit_code: i32) -> ! {
    exit_current_and_run_next(exit_code);
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
    0
}

pub fn sys_getpid() -> isize {
    current_task().unwrap().process.upgrade().unwrap().getpid() as isize
}

pub fn sys_fork() -> isize {
    let current_process = current_process();
    let new_process = current_process.fork();
    let new_pid = new_process.getpid();
    // modify trap context of new_task, because it returns immediately after switching
    let new_process_inner = new_process.inner_exclusive_access();
    let task = new_process_inner.tasks[0].as_ref().unwrap();
    let trap_cx = task.inner_exclusive_access().get_trap_cx();
    // we do not have to move to next instruction since we have done it before
    // for child process, fork returns 0
    trap_cx.x[10] = 0;
    new_pid as isize
}

pub fn sys_exec(path: *const u8, mut args: *const usize) -> isize {
    let token = current_user_token();
    let path = translated_str(token, path);
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
    if let Some(app_inode) = open_file(path.as_str(), OpenFlags::RDONLY) {
        let all_data = app_inode.read_all();
        let process = current_process();
        let argc = args_vec.len();
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_exec(path: {:?}, args: {:x?} ) = {}",
            path,
            args,
            argc
        );
        process.exec(all_data.as_slice(), args_vec);
        // return argc because cx.x[10] will be covered with it later
        argc as isize
    } else {
        -1
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
            let mut inner = process.inner_exclusive_access();

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
                    p.inner_exclusive_access().is_zombie
                        && (pid == -1 || pid as usize == p.getpid())
                    // ++++ release child PCB
                });
                if let Some((idx, _)) = pair {
                    let child = inner.children.remove(idx);
                    // confirm that child will be deallocated after being removed from children list
                    assert_eq!(Arc::strong_count(&child), 1);
                    let found_pid = child.getpid();
                    // ++++ temporarily access child PCB exclusively
                    // let exit_code = child.inner_exclusive_access().exit_code;
                    // ++++ release child PCB
                    if wstatus as usize != 0 {
                        *translated_refmut(inner.memory_set.token(), wstatus) = 3 << 8;
                    }
                    return found_pid as isize;
                } else {
                    exited = false;
                }
            }
            // ---- release current PCB automatically
        }
        // not found yet
        assert!(!found || !exited);
        if !found && options == WNOHANG {
            return -1;
        }
        suspend_current_and_run_next();
    }
}

pub fn sys_kill(pid: usize, signal: u32) -> isize {
    if let Some(process) = pid2process(pid) {
        if let Some(flag) = SignalFlags::from_bits(signal) {
            process.inner_exclusive_access().signals |= flag;
            0
        } else {
            -1
        }
    } else {
        -1
    }
}

pub fn sys_brk(addr: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    // println!("syscall brk addr = {:x?}, base = {:x?}, top = {:x?}", addr, inner.user_heap_base, inner.user_heap_top);
    if addr == 0 {
        inner.user_heap_top as isize
    } else if addr >= inner.user_heap_base {
        if addr < inner.user_heap_top {
            let prev_top = inner.user_heap_top;
            inner.memory_set.remove_heap_dataframes(prev_top, addr);
        }
        inner.user_heap_top = addr as usize;
        addr as isize
    } else {
        -1
    }
}

pub fn sys_mmap(
    start: usize,
    len: usize,
    prot: usize,
    flags: usize,
    fd: isize,
    offset: usize,
) -> isize {
    if start != 0 {
        unimplemented!();
    }
    let start = aligned_up(current_process().inner_exclusive_access().mmap_area_top);
    let aligned_len = aligned_up(len);

    let ret = current_process().mmap(start, aligned_len, prot, flags, fd, offset);
    gdb_println!(SYSCALL_ENABLE, "sys_mmap(aligned_start: {:#x?}, aligned_len: {}, prot: {:x?}, flags: {:x?}, fd: {}, offset: {} ) = {}", start, aligned_len, prot, flags, fd, offset, ret);
    ret
}

pub fn sys_munmap(start: usize, _len: usize) -> isize {
    let start = aligned_up(start);

    let ret = current_process().munmap(start, _len);
    gdb_println!(SYSCALL_ENABLE, "sys_munmap(aligend_start: {:#x?}, len: {}) = {}", start, _len, ret);
    ret
}
