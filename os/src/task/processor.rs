use core::cell::{RefCell, RefMut};

use super::{__switch, add_task};
use super::{fetch_task, TaskStatus};
use super::{ProcessControlBlock, TaskContext, TaskControlBlock};

use crate::board::MAX_CPU_NUM;
use crate::multicore::get_hartid;
use crate::trap::TrapContext;
use alloc::sync::Arc;
use spin::{Lazy, RwLock};

pub struct Processor {
    inner: RefCell<ProcessorInner>,
}

pub struct ProcessorInner {
    current: Option<Arc<TaskControlBlock>>,
    idle_task_cx: TaskContext,
}

pub static mut __FA: [FastAccessStruct; MAX_CPU_NUM] = [FastAccessStruct {
    __tid: 0,
    __trap_cx_va: 0,
    __trap_cx_pa: 0,
    __user_token: 0,
}; MAX_CPU_NUM];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FastAccessStruct {
    pub __tid: usize,
    pub __trap_cx_va: usize,
    pub __trap_cx_pa: usize,
    pub __user_token: usize,
}

unsafe impl Sync for Processor {}

impl Processor {
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ProcessorInner {
                current: None,
                idle_task_cx: TaskContext::zero_init(),
            }),
        }
    }
    pub fn inner_exclusive_access(&self) -> RefMut<'_, ProcessorInner> {
        self.inner.borrow_mut()
    }
}

impl ProcessorInner {
    fn get_idle_task_cx_ptr(&mut self) -> *mut TaskContext {
        &mut self.idle_task_cx as *mut _
    }

    pub fn take_current(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.current.take()
    }

    pub fn current(&self) -> Option<Arc<TaskControlBlock>> {
        self.current.as_ref().map(Arc::clone)
    }
}

#[cfg(feature = "board_fu740")]
pub static PROCESSORS: Lazy<[Processor; MAX_CPU_NUM]> = Lazy::new(|| {
    [
        Processor::new(),
        Processor::new(),
        Processor::new(),
        Processor::new(),
        Processor::new(),
    ]
});

#[cfg(not(any(feature = "board_fu740")))]
pub static PROCESSORS: Lazy<[Processor; MAX_CPU_NUM]> = Lazy::new(|| {
    [
        Processor::new(),
        Processor::new(),
        Processor::new(),
        Processor::new(),
    ]
});

pub fn run_tasks() {
    loop {
        let mut processor = PROCESSORS[get_hartid()].inner_exclusive_access();

        // 本来下面这段代码应该由suspend_current_and_run_next完成
        // 但是若如此做，则内核栈会被其他核“趁虚而入”
        // 将suspend_current_and_run_next中的add_task延后到调度完成后

        // 将核与任务进行简单绑定 √
        // if let Some(last_task) = processor.take_current() {
        //         // add_task(last_task);
        //         if let Some(task) = fetch_task() {
        //             let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
        //             // access coming task TCB exclusively
        //             let mut task_inner = task.inner_exclusive_access();
        //             let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
        //             task_inner.task_status = TaskStatus::Running;
        //             drop(task_inner);
        //             // release coming task TCB manually
        //             // println!("[cpu {}] switch to process {}", get_hartid(), task.process.upgrade().unwrap().pid.0);
        //             processor.current = Some(task);

        //             // release processor manually
        //             drop(processor);
        //             add_task(last_task);
        //             unsafe {
        //                 __switch(idle_task_cx_ptr, next_task_cx_ptr);
        //             }
        //         }else {
        //             let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
        //             let mut task_inner = last_task.inner_exclusive_access();
        //             let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
        //             task_inner.task_status = TaskStatus::Running;
        //             drop(task_inner);
        //             // release coming task TCB manually
        //             // println!("[cpu {}] switch to process {}", get_hartid(), task.process.upgrade().unwrap().pid.0);
        //             processor.current = Some(last_task);

        //             // release processor manually
        //             drop(processor);
        //             unsafe {
        //                 __switch(idle_task_cx_ptr, next_task_cx_ptr);
        //             }
        //         }
        // //first in
        // }else {
        //         if let Some(task) = fetch_task() {
        //             let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
        //             // access coming task TCB exclusively
        //             let mut task_inner = task.inner_exclusive_access();
        //             let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
        //             task_inner.task_status = TaskStatus::Running;
        //             drop(task_inner);
        //             // release coming task TCB manually
        //             // println!("[cpu {}] switch to process {}", get_hartid(), task.process.upgrade().unwrap().pid.0);
        //             processor.current = Some(task);

        //             // release processor manually
        //             drop(processor);
        //             unsafe {
        //                 __switch(idle_task_cx_ptr, next_task_cx_ptr);
        //             }
        //         }
        // }

        // 核不绑定任务  √
        if let Some(last_task) = processor.take_current() {
            // Do not enqueue blocking tasks!
            if last_task.acquire_inner_lock().task_status == TaskStatus::Ready {
                add_task(last_task);
            }
        }

        if let Some(task) = fetch_task() {
            let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();

            unsafe {
                let user_token = task.process.upgrade().unwrap().acquire_inner_lock().get_user_token();
                __FA[get_hartid()].__user_token = user_token;
            }

            let mut task_inner = task.acquire_inner_lock();
            let next_task_cx_ptr = &task_inner.task_cx as *const TaskContext;
            task_inner.task_status = TaskStatus::Running;
            task_inner.__save_info_to_fast_access();
            drop(task_inner);
            // release coming task TCB manually
            // println!("[cpu {}] switch to process {}", get_hartid(), task.process.upgrade().unwrap().getpid());
            processor.current = Some(task);

            // release processor manually
            drop(processor);
            unsafe {
                __switch(idle_task_cx_ptr, next_task_cx_ptr);
            }
        }
    }
}

pub fn take_current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[get_hartid()]
        .inner_exclusive_access()
        .take_current()
}

#[inline(always)]
pub fn current_task() -> Option<Arc<TaskControlBlock>> {
    PROCESSORS[get_hartid()].inner_exclusive_access().current()
}

#[inline(always)]
pub fn current_process() -> Arc<ProcessControlBlock> {
    current_task().unwrap().process.upgrade().unwrap()
}

#[inline(always)]
pub fn current_tid() -> usize {
    unsafe { __FA[get_hartid()].__tid }
}

#[inline(always)]
pub fn current_user_token() -> usize {
    unsafe { __FA[get_hartid()].__user_token }
}

#[inline(always)]
pub fn current_trap_cx() -> &'static mut TrapContext {
    unsafe { (__FA[get_hartid()].__trap_cx_pa as *mut TrapContext).as_mut().unwrap() }
}

#[inline(always)]
pub fn current_trap_cx_user_va() -> usize {
    unsafe { __FA[get_hartid()].__trap_cx_va }
}

#[inline(always)]
pub fn current_kstack_top() -> Option<usize> {
    // backtrace时一些核心可能没有current_task
    if let Some(task) = current_task() {
        Some(task.kstack.get_top())
    } else {
        None
    }
}

pub fn schedule(switched_task_cx_ptr: *mut TaskContext) {
    let mut processor = PROCESSORS[get_hartid()].inner_exclusive_access();
    let idle_task_cx_ptr = processor.get_idle_task_cx_ptr();
    drop(processor);
    unsafe {
        __switch(switched_task_cx_ptr, idle_task_cx_ptr);
    }
}
