use crate::{
    gdb_println,
    mm::{translated_ref, translated_refmut},
    monitor::{QEMU, SYSCALL_ENABLE},
    syscall::sys_sleep,
    task::{
        current_process, current_task, current_user_token, is_signal_valid,
        suspend_current_and_run_next, tid2task, SAFlags, SigAction, UContext, SIGKILL, SIG_DFL,
    },
};

use super::errorno::{EINVAL, ESRCH};

fn do_tkill(tid: usize, signum: u32) -> isize {
    if let Some(task) = tid2task(tid) {
        if is_signal_valid(signum) {
            let mut inner = task.acquire_inner_lock();
            inner.add_signal(signum);
            if signum == SIGKILL {
                inner.killed = true;
            }
            0
        } else {
            -EINVAL
        }
    } else {
        -ESRCH
    }
}

pub fn sys_kill(pid: usize, signum: u32) -> isize {
    let ret = do_tkill(pid, signum);
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_kill(pid: {}, signum: {}) = {}",
        pid,
        signum,
        ret
    );
    ret
}

pub fn sys_tkill(tid: usize, signum: u32) -> isize {
    let ret = do_tkill(tid, signum);
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_tkill(tid: {}, signum: {}) = {}",
        tid,
        signum,
        ret
    );
    ret
}

pub fn sys_sigaction(signum: u32, sa_ptr: *const SigAction, oldsa_ptr: *mut SigAction) -> isize {
    let token = current_user_token();
    let process = current_process();
    let mut inner = process.acquire_inner_lock();

    // signum超出范围，返回错误
    if !is_signal_valid(signum) {
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_sigaction(signum: {}, sigaction = {:#x?}, old_sigaction = {:#x?} ) = {}",
            signum,
            sa_ptr,
            oldsa_ptr,
            -EINVAL
        );
        return -EINVAL;
    }

    // 当sigaction存在时， 在pcb中注册给定的signaction
    if sa_ptr as usize != 0 {
        // 如果旧的sigaction存在，则将它保存到指定位置.否则置为 SIG_DFL
        let old = inner.sigactions[signum as usize];
        if old.is_valid() {
            if oldsa_ptr as usize != 0 {
                // println!("arg old_sigaction !=0  ");
                *translated_refmut(token, oldsa_ptr) = old;
            }
        } else {
            if oldsa_ptr as usize != 0 {
                let sigact_old = translated_refmut(token, oldsa_ptr);
                sigact_old.sa_handler = SIG_DFL;
                sigact_old.sa_sigaction = 0;
                sigact_old.sa_mask = 0;
            }
        }

        //在pcb中注册给定的signaction
        inner.sigactions[signum as usize] = *translated_ref(token, sa_ptr);
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_sigaction(signum: {}, sigaction = {:#x?}, old_sigaction = {:#x?} ) = {}",
        signum,
        sa_ptr, // sigact,
        oldsa_ptr,
        0
    );
    return 0;
}

pub fn sys_sigreturn() -> isize {
    // #[cfg(feature = "sig_delay")]
    // for _ in 0..15 {
    //     suspend_current_and_run_next();
    // }
    let token = current_user_token();
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();

    let trap_cx = task_inner.get_trap_cx();
    let mc_pc_ptr = trap_cx.x[2] + UContext::pc_offset();
    drop(trap_cx);

    let (signum, flags) = task_inner.signal_context_restore();

    if flags.contains(SAFlags::SA_SIGINFO) {
        let mc_pc = *translated_ref(token, mc_pc_ptr as *mut u64) as usize;
        gdb_println!(
            SYSCALL_ENABLE,
            "original sepc: {:#x?}, mc_pc = {:#x?}",
            task_inner.get_trap_cx().sepc,
            mc_pc
        );
        task_inner.get_trap_cx().sepc = mc_pc; // 确保SIGCANCEL的正确性，使程序跳转到sig_exit
    }

    gdb_println!(SYSCALL_ENABLE, "sys_sigreturn() = 0");
    return 0;
}

const SIG_BLOCK: usize = 0;
const SIG_UNBLOCK: usize = 1;
const SIG_SETMASK: usize = 2;

pub fn sys_sigprocmask(how: usize, set: *const u64, old_set: *mut u64, sigsetsize: usize) -> isize {
    if sigsetsize != 8 {
        panic!("sigsetsize != 8");
    }
    let token = current_user_token();
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let mut mask = task_inner.sigmask;

    if old_set as usize != 0 {
        *translated_refmut(token, old_set) = mask;
    }

    if set as usize != 0 {
        let new_set = *translated_ref(token, set);
        match how {
            SIG_BLOCK => mask |= new_set,
            SIG_UNBLOCK => mask &= !new_set,
            SIG_SETMASK => mask = new_set,
            _ => panic!("ENOSYS"),
        }
        task_inner.sigmask = mask;
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_sigprocmask(how: {}, set: {:#x?}, old_set: {:#x?}, sigsetsize: {}) = 0",
        how,
        set,
        old_set,
        sigsetsize
    );
    0
}
