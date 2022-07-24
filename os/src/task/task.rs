use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::mm::PhysPageNum;
use crate::trap::TrapContext;
use alloc::collections::VecDeque;
use alloc::sync::{Arc, Weak};

use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};

pub struct TaskControlBlock {
    // immutable
    pub process: Weak<ProcessControlBlock>,
    pub kstack: KernelStack,
    // mutable
    inner: Arc<Mutex<TaskControlBlockInner>>,
}

impl TaskControlBlock {
    pub fn acquire_inner_lock(&self) -> MutexGuard<'_, TaskControlBlockInner> {
        self.inner.lock()
    }

    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.acquire_inner_lock();
        inner.memory_set.token()
    }
}

pub struct TaskControlBlockInner {
    pub res: Option<TaskUserRes>,
    pub trap_cx_ppn: PhysPageNum,
    pub task_cx: TaskContext,
    pub task_status: TaskStatus,
    pub exit_code: Option<i32>,
    pub pending_signals: VecDeque<u32>,
    pub sigmask: u64,
    pub clear_child_tid: Option<ClearChildTid>,
    trap_cx_backup: Vec<TrapContext>,
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        // debug!("trap_cx_ppn = {:#x?}", self.trap_cx_ppn);
        self.trap_cx_ppn.get_mut()
    }

    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }

    pub fn pop_trap_cx(&mut self) {
        *self.get_trap_cx() = self.trap_cx_backup.pop().unwrap();
    }

    pub fn push_trap_cx(&mut self) {
        self.trap_cx_backup.push((*self.get_trap_cx()).clone());
    }

    pub fn is_signaling(&self) -> bool {
        !self.trap_cx_backup.is_empty()
    }

    pub fn gettid(&self) -> usize {
        self.res.as_ref().unwrap().tid.0
    }

    pub fn get_relative_tid(&self) -> usize {
        self.res.as_ref().unwrap().rel_tid
    }
}

impl TaskControlBlock {
    /// pid == -1 means that the main thread is being created.
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        pid: isize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, pid, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kstack,
            inner: Arc::new(Mutex::new(TaskControlBlockInner {
                res: Some(res),
                trap_cx_ppn,
                task_cx: TaskContext::goto_trap_return(kstack_top),
                task_status: TaskStatus::Ready,
                exit_code: None,
                pending_signals: VecDeque::new(),
                sigmask: 0,
                trap_cx_backup: Vec::new(),
                clear_child_tid: None,
            })),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Ready,
    Running,
    Blocking,
}

#[derive(Debug)]
pub struct ClearChildTid {
    pub ctid: u32,
    pub addr: usize,
}
