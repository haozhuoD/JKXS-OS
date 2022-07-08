mod aux;
mod context;
mod id;
mod manager;
mod process;
mod processor;
mod signal;
mod switch;
#[allow(clippy::module_inception)]
mod task;
mod utils;

use crate::{loader::get_initproc_binary, task::utils::user_backtrace, config::SIGRETURN_TRAMPOLINE};
use alloc::sync::Arc;
use manager::fetch_task;
use process::ProcessControlBlock;
use spin::Lazy;
use switch::__switch;
use crate::config::TRAMPOLINE;

pub use aux::*;
pub use context::TaskContext;
pub use id::{kstack_alloc, pid_alloc, KernelStack, PidHandle};
pub use manager::{add_task, pid2process, remove_from_pid2process};
pub use process::*;
pub use processor::*;
pub use signal::*;
pub use task::{TaskControlBlock, TaskStatus};

pub fn suspend_current_and_run_next() {
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

// pub fn block_current_and_run_next() {
//     let task = take_current_task().unwrap();
//     let mut task_inner = task.inner_exclusive_access();
//     let task_cx_ptr = &mut task_inner.task_cx as *mut TaskContext;
//     task_inner.task_status = TaskStatus::Blocking;
//     drop(task_inner);
//     schedule(task_cx_ptr);
// }

pub fn exit_current_and_run_next(exit_code: i32, is_exit_group: bool) -> ! {
    let task = take_current_task().unwrap();
    let mut task_inner = task.acquire_inner_lock();
    let process = task.process.upgrade().unwrap();
    let tid = task_inner.res.as_ref().unwrap().tid;
    // record exit code
    task_inner.exit_code = Some(exit_code);
    task_inner.res = None;
    // here we do not remove the thread since we are still using the kstack
    // it will be deallocated when sys_waittid is called
    drop(task_inner);
    drop(task);
    // however, if this is the main thread of current process
    // the process should terminate at once
    if tid == 0 || is_exit_group {
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
    let process = current_process();

    loop {
        // 取出pending的第一个signal
        let signum_option = process
            .acquire_inner_lock()
            .signal_info
            .pending_signals
            .pop_front();
        if signum_option.is_none() {
            break;
        }
        let signum = signum_option.unwrap();
        {
            let mut inner = process.acquire_inner_lock();
            if let Some(sigaction) = inner.signal_info.sigactions.get(&signum).clone() {
                let sigaction = sigaction.clone();
                // 如果信号对应的处理函数存在，则做好跳转到handler的准备
                if sigaction.handler != SIG_DFL && sigaction.handler != SIG_IGN { 
                    let task = current_task().unwrap();
                    let mut task_inner = task.acquire_inner_lock();
                    let mut trap_cx = task_inner.get_trap_cx();
                    // 保存当前trap_cx
                    task_inner.push_trap_cx();

                    // 准备跳到signal handler
                    extern "C" {
                        fn __sigreturn();
                        fn __alltraps();
                    }
                    trap_cx.x[1] = __sigreturn as usize - __alltraps as usize + SIGRETURN_TRAMPOLINE; // ra 
                    trap_cx.x[10] = signum; // a0 (args0 = signum)
                    trap_cx.sepc = sigaction.handler; // sepc
                    return;
                }
                if sigaction.handler == SIG_DFL{
                    //SIG_DFL 终止程序
                    // error!("[perform_signals_of_current]-fn pid:{} signal_num:{}, SIG_DFL kill process",current_pid(),signum);
                    drop(inner);
                    drop(process);
                    exit_current_and_run_next(-(signum as i32), false);
                }
                if sigaction.handler == SIG_IGN
                {
                    //SIG_IGN 忽略
                    // error!("[perform_signals_of_current]-fn pid:{} signal_num:{}, SIG_IGN ignore process",current_pid(),signum);
                    return ;
                }
            }
        }
        // 如果信号代表当前进程出错，则exit
        if let Some(msg) = SIGNAL_ERRORS.get(&signum) {
            error!("{}", msg);
            drop(process);
            exit_current_and_run_next(-(signum as i32), false);
        };
    }
}

pub fn current_add_signal(signum: usize) {
    let process = current_process();
    let mut process_inner = process.acquire_inner_lock();
    process_inner.signal_info.pending_signals.push_back(signum);
}