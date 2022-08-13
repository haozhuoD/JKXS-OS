use super::{ProcessControlBlock, TaskControlBlock};

use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use spin::{Lazy, Mutex, RwLock};

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    waiting_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            waiting_queue: VecDeque::new(),
        }
    }
    pub fn add_to_ready_queue(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch_from_ready_queue(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
    pub fn add_to_waiting_queue(&mut self, task: Arc<TaskControlBlock>) {
        self.waiting_queue.push_back(task);
    }
}

pub static TASK_MANAGER: Lazy<Mutex<TaskManager>> = Lazy::new(|| Mutex::new(TaskManager::new()));
pub static PID2PCB: Lazy<RwLock<BTreeMap<usize, Arc<ProcessControlBlock>>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));
pub static TID2TCB: Lazy<RwLock<BTreeMap<usize, Arc<TaskControlBlock>>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add_to_ready_queue(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.lock().fetch_from_ready_queue()
}

pub fn block_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add_to_waiting_queue(task);
}

pub fn unblock_task(task: Arc<TaskControlBlock>) {
    let mut mlock = TASK_MANAGER.lock();
    let p = mlock
        .waiting_queue
        .iter()
        .enumerate()
        .find(|(_, t)| Arc::ptr_eq(t, &task))
        .map(|(idx, t)| (idx, t.clone()));

    if let Some((idx, task)) = p {
        mlock.waiting_queue.remove(idx);
        mlock.add_to_ready_queue(task);
    }
}

#[allow(unused)]
pub fn task_count() -> usize {
    TASK_MANAGER.lock().ready_queue.clone().into_iter().count()
}

// #[allow(unused)]
// pub fn pid2process(pid: usize) -> Option<Arc<ProcessControlBlock>> {
//     let map = PID2PCB.read();
//     map.get(&pid).map(Arc::clone)
// }

// pub fn insert_into_pid2process(pid: usize, process: Arc<ProcessControlBlock>) {
//     PID2PCB.write().insert(pid, process);
// }

// pub fn remove_from_pid2process(pid: usize) {
//     let mut map = PID2PCB.write();
//     if map.remove(&pid).is_none() {
//         panic!("cannot find pid {} in pid2process!", pid);
//     }
// }

pub fn tid2task(tid: usize) -> Option<Arc<TaskControlBlock>> {
    let map = TID2TCB.read();
    map.get(&tid).map(Arc::clone)
}

pub fn insert_into_tid2task(tid: usize, task: Arc<TaskControlBlock>) {
    TID2TCB.write().insert(tid, task);
}

pub fn remove_from_tid2task(tid: usize) {
    let mut map = TID2TCB.write();
    if map.remove(&tid).is_none() {
        panic!("cannot find pid {} in tid2task!", tid);
    }
}
