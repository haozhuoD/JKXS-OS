mod aux;
mod context;
mod id;
mod manager;
mod process;
mod processor;
mod siginfo;
mod switch;
#[allow(clippy::module_inception)]
mod task;
mod time_info;
mod utils;

use core::mem::size_of;

use crate::{
    config::SIGRETURN_TRAMPOLINE,
    gdb_println,
    loader::get_initproc_binary,
    mm::{translated_byte_buffer, translated_refmut, UserBuffer},
    monitor::{QEMU, SYSCALL_ENABLE},
    syscall::futex_wake,
};
use alloc::sync::Arc;
use manager::fetch_task;
use process::ProcessControlBlock;
use spin::Lazy;
use switch::__switch;

pub use aux::*;
pub use context::TaskContext;
pub use id::{kstack_alloc, tid_alloc, KernelStack, TidHandle};
pub use manager::*;
pub use process::*;
pub use processor::*;
pub use siginfo::*;
pub use task::*;
pub use time_info::*;

pub fn suspend_current_and_run_next() {
    // wakeup_futex_waiters();
    // 将原来的take_current改为current_task，也就是说suspend之后，task仍然保留在processor中
    let task = current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();

    // 在内核态手动处理SIGKILL，否则可能导致进程在内核态卡死
    if task_inner.killed {
        drop(task_inner);
        drop(task);
        exit_current_and_run_next(-(SIGKILL as i32), false);
    }
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;

    // Change status to Ready
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    drop(task);
    // ---- release current TCB

    // 将“add_task”延迟到调度完成（即切换到idle控制流）之后
    // 若不这么做，该线程将自己挂到就绪队列，另一cpu核可能趁此机会取出该线程，并进入该线程的内核栈
    // 从而引发错乱。

    /*
    // push back to ready queue.
    add_task(task);
    */
    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

pub fn block_current_and_run_next() {
    let task = current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Blocking;
    block_task(task.clone());

    drop(task_inner);
    drop(task);

    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

pub fn exit_current_and_run_next(exit_code: i32, is_exit_group: bool) -> ! {
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let process = task.process.upgrade().unwrap();
    let rel_tid = task_inner.get_relative_tid();

    // do futex_wake if clear_child_tid is set
    if let Some(p) = &task_inner.clear_child_tid {
        // debug!("p = {:#x?}", p);
        *translated_refmut(
            process.acquire_inner_lock().get_user_token(),
            p.addr as *mut u32,
        ) = 0;
        futex_wake(p.addr, 1);
    }

    remove_from_tid2task(task_inner.gettid());

    // record exit code
    task_inner.res = None;

    drop(task_inner);
    drop(task);

    // however, if this is the main thread of current process
    // the process should terminate at once
    if rel_tid == 0 || is_exit_group {
        // remove_from_pid2process(process.getpid());
        let mut initproc_inner = INITPROC.acquire_inner_lock();
        let mut process_inner = process.acquire_inner_lock();
        // mark this process as a zombie process
        process_inner.is_zombie = true;
        // record exit code of main process
        process_inner.exit_code = exit_code;

        {
            // move all child processes under init process
            for child in process_inner.children.iter() {
                child.acquire_inner_lock().parent = Some(Arc::downgrade(&INITPROC));
                initproc_inner.children.push(child.clone());
            }
        }

        drop(initproc_inner);

        // deallocate user res (including tid/trap_cx/ustack) of all threads
        // it has to be done before we dealloc the whole memory_set
        // otherwise they will be deallocated twice
        for task in process_inner.tasks.iter().filter(|t| t.is_some()) {
            let task = task.as_ref().unwrap();
            let mut task_inner = task.acquire_inner_lock();
            task_inner.res = None;
        }

        process_inner.children.clear();
        // deallocate other data in user space i.e. program code/data section
        process_inner.memory_set.recycle_data_pages();
        // drop file descriptors
        process_inner.fd_table.clear();

        // notify parent to recycle me
        let ptask = process_inner
            .parent
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
            .acquire_inner_lock()
            .get_task(0);

        unblock_task(ptask.clone());
    }
    drop(process);
    // we do not have to save task context
    let mut _unused = TaskContext::zero_init();
    schedule(&mut _unused as *mut _);
    panic!("Shouldn't reach here in `exit_current_and_run_next`!")
}

pub static INITPROC: Lazy<Arc<ProcessControlBlock>> =
    Lazy::new(|| ProcessControlBlock::new(get_initproc_binary())); // add_task here

pub fn add_initproc() {
    let _initproc = INITPROC.clone();
}

pub fn perform_signals_of_current() {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();

    // 禁止中断嵌套 & 提前退出，
    if task_inner.pending_signals == 0 || task_inner.is_signaling() {
        return;
    }

    let process = current_process();

    loop {
        // 取出pending的第一个signal
        let signum;
        match task_inner.fetch_signal() {
            Some(s) => signum = s,
            None => return,
        };

        let process_inner = process.acquire_inner_lock();
        let sigaction = process_inner.sigactions[signum as usize];
        // 如果信号对应的处理函数存在，则做好跳转到handler的准备
        let handler = sigaction.sa_handler;
        if sigaction.sa_handler == SIG_IGN {
            return;
        }
        if sigaction.sa_handler == SIG_DFL {
            //SIG_DFL 终止程序
            if signum == SIGKILL || signum == SIGSEGV {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "[perform_signals_of_current]-fn pid:{} signal_num:{}, SIG_DFL kill process",
                    current_tid(),
                    signum
                );
                drop(process_inner);
                drop(process);
                drop(task_inner);
                drop(task);
                exit_current_and_run_next(-(signum as i32), false);
            }
            return;
        }

        // 准备跳到signal handler
        // 保存当前trap_cx
        task_inner.signal_context_save(signum, sigaction.sa_flags);

        extern "C" {
            fn __sigreturn();
            fn __alltraps();
        }
        let mut trap_cx = task_inner.get_trap_cx();
        trap_cx.x[1] = __sigreturn as usize - __alltraps as usize + SIGRETURN_TRAMPOLINE; // ra
        trap_cx.x[10] = signum as usize; // a0 (args0 = signum)

        if sigaction.sa_flags.contains(SAFlags::SA_SIGINFO) {
            let token = current_user_token();
            let mc_pc_ptr = trap_cx.x[2] + UContext::pc_offset();   
            trap_cx.x[2] -= size_of::<UContext>(); // sp -= sizeof(ucontext)
            trap_cx.x[12] = trap_cx.x[2]; // a2  = sp
            *translated_refmut(token, mc_pc_ptr as *mut u64) = trap_cx.sepc as u64;
        }
        // debug!("prepare to jump to `handler`, original sepc = {:#x?}", trap_cx.sepc);

        trap_cx.sepc = handler; // sepc = handler
        return;
    }
}

pub fn current_add_signal(signum: u32) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.add_signal(signum);
}
