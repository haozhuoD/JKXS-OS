use super::{ProcessControlBlock, TaskControlBlock};

use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use spin::{RwLock, Lazy};

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

pub static TASK_MANAGER: Lazy<RwLock<TaskManager>> = Lazy::new(|| RwLock::new(TaskManager::new()));
pub static PID2PCB: Lazy<RwLock<BTreeMap<usize, Arc<ProcessControlBlock>>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.write().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.write().fetch()
}

#[allow(unused)]
pub fn task_count() -> usize {
    TASK_MANAGER.read().ready_queue.clone().into_iter().count()
}

pub fn pid2process(pid: usize) -> Option<Arc<ProcessControlBlock>> {
    let map = PID2PCB.read();
    map.get(&pid).map(Arc::clone)
}

pub fn insert_into_pid2process(pid: usize, process: Arc<ProcessControlBlock>) {
    PID2PCB.write().insert(pid, process);
}

pub fn remove_from_pid2process(pid: usize) {
    let mut map = PID2PCB.write();
    if map.remove(&pid).is_none() {
        panic!("cannot find pid {} in pid2task!", pid);
    }
}
