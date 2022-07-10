use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::mm::PhysPageNum;
use crate::trap::TrapContext;
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
    trap_cx_backup: Vec<TrapContext>
}

impl TaskControlBlockInner {
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
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

    pub fn gettid(&self) -> usize {
        self.res.unwrap().tid.0
    }
}

impl TaskControlBlock {
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
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
                trap_cx_backup: Vec::new()
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
