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
mod utils;

use core::mem::size_of;

use crate::{
    config::SIGRETURN_TRAMPOLINE,
    loader::get_initproc_binary,
    mm::{translated_byte_buffer, UserBuffer, translated_refmut}, syscall::futex_wake, timer::wakeup_futex_waiters,
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

pub fn suspend_current_and_run_next() {
    wakeup_futex_waiters();
    // There must be an application running.
    // 将原来的take_current改为current_task，也就是说suspend之后，task仍然保留在processor中
    let task = current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
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
    // There must be an application running.
    // 将原来的take_current改为current_task，也就是说blocking之后，task仍然保留在processor中
    let task = current_task().unwrap();

    // ---- access current TCB exclusively
    let mut task_inner = task.acquire_inner_lock();
    let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
    // Change status to Ready
    task_inner.task_status = TaskStatus::Blocking;
    drop(task_inner);
    drop(task);

    // jump to scheduling cycle
    schedule(task_cx_ptr);
}

/// 需要保证该task目前没有上锁
pub fn unblock_task(task: Arc<TaskControlBlock>) {
    let mut task_inner = task.acquire_inner_lock();
    assert!(task_inner.task_status == TaskStatus::Blocking);
    task_inner.task_status = TaskStatus::Ready;
    drop(task_inner);
    add_task(task);
}

pub fn exit_current_and_run_next(exit_code: i32, is_exit_group: bool) -> ! {
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let process = task.process.upgrade().unwrap();
    let rel_tid = task_inner.get_relative_tid();

    // do futex_wake if clear_child_tid is set
    if let Some(p) = &task_inner.clear_child_tid {
        // debug!("p = {:#x?}", p);
        *translated_refmut(process.acquire_inner_lock().get_user_token(), p.addr as *mut u32) = 0;
        futex_wake(p.addr, 1);
    }

    remove_from_tid2task(task_inner.gettid());

    // record exit code
    task_inner.exit_code = Some(exit_code);
    task_inner.res = None;

    // here we do not remove the thread since we are still using the kstack
    // it will be deallocated when sys_waittid is called
    drop(task_inner);
    drop(task);
    // however, if this is the main thread of current process
    // the process should terminate at once
    if rel_tid == 0 || is_exit_group {
        remove_from_pid2process(process.getpid());
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
    if task_inner.pending_signals.is_empty() || task_inner.is_signaling() {
        return;
    }

    let process = current_process();

    loop {
        // 取出pending的第一个signal
        let signum_option = task_inner.pending_signals.pop_front();
        if signum_option.is_none() {
            break;
        }
        let signum = signum_option.unwrap();
        {
            let process_inner = process.acquire_inner_lock();
            if let Some(sigaction) = process_inner.sigactions.get(&signum) {
                // 如果信号对应的处理函数存在，则做好跳转到handler的准备
                let handler = sigaction.sa_handler;
                let token = process_inner.get_user_token();
                if sigaction.sa_handler != SIG_DFL && sigaction.sa_handler != SIG_IGN {
                    let mut trap_cx = task_inner.get_trap_cx();
                    // 保存当前trap_cx
                    task_inner.signal_context_save(signum, sigaction.sa_flags);

                    // 准备跳到signal handler
                    extern "C" {
                        fn __sigreturn();
                        fn __alltraps();
                    }
                    trap_cx.x[1] =
                        __sigreturn as usize - __alltraps as usize + SIGRETURN_TRAMPOLINE; // ra
                    trap_cx.x[10] = signum as usize; // a0 (args0 = signum)

                    if sigaction.sa_flags.contains(SAFlags::SA_SIGINFO) {
                        trap_cx.x[2] -= size_of::<UContext>(); // sp -= sizeof(ucontext)
                        trap_cx.x[12] = trap_cx.x[2];          // a2  = sp
                        let mut userbuf = UserBuffer::new(translated_byte_buffer(
                            token,
                            trap_cx.x[2] as *const u8,
                            size_of::<UContext>(),
                        ));
                        let mut ucontext = UContext::new();
                        *ucontext.mc_pc() = trap_cx.sepc;
                        userbuf.write(ucontext.as_bytes()); // copy ucontext to userspace
                    }
                    // debug!("prepare to jump to `handler`, original sepc = {:#x?}", trap_cx.sepc);

                    trap_cx.sepc = handler; // sepc = handler
                    return;
                }
                if sigaction.sa_handler == SIG_DFL {
                    //SIG_DFL 终止程序
                    // error!("[perform_signals_of_current]-fn pid:{} signal_num:{}, SIG_DFL kill process",current_pid(),signum);
                    drop(process_inner);
                    drop(process);
                    exit_current_and_run_next(-(signum as i32), false);
                }
                if sigaction.sa_handler == SIG_IGN {
                    //SIG_IGN 忽略
                    // error!("[perform_signals_of_current]-fn pid:{} signal_num:{}, SIG_IGN ignore process",current_pid(),signum);
                    return;
                }
            }
        }
        // 如果信号代表当前进程出错，则exit
        if let Some(msg) = SIGNAL_DFL_EXIT.get(&signum) {
            error!("[tid={}] {}", current_tid(), msg);
            drop(process);
            exit_current_and_run_next(-(signum as i32), false);
        };
    }
}

pub fn current_add_signal(signum: u32) {
    let task = current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    task_inner.pending_signals.push_back(signum);
}
