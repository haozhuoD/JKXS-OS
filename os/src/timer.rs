use core::cmp::Ordering;

use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use crate::sync::UPSafeCell;
use crate::task::{add_task, TaskControlBlock};
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use lazy_static::*;
use riscv::register::time;

// pub const MSEC_PER_SEC: usize = 1000;
pub const NSEC_PER_SEC: usize = 1000000000;
const TICKS_PER_SEC: usize = 100;

pub fn get_time() -> usize {
    time::read()
}

// pub fn get_time_ms() -> usize {
//     time::read() / (CLOCK_FREQ / MSEC_PER_SEC)
// }

pub fn get_time_ns() -> usize {
    time::read() * 100000 / (CLOCK_FREQ / 10000)
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}

pub struct TimerCondVar {
    pub expire_ns: usize,
    pub task: Arc<TaskControlBlock>,
}

impl PartialEq for TimerCondVar {
    fn eq(&self, other: &Self) -> bool {
        self.expire_ns == other.expire_ns
    }
}
impl Eq for TimerCondVar {}
impl PartialOrd for TimerCondVar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = -(self.expire_ns as isize);
        let b = -(other.expire_ns as isize);
        Some(a.cmp(&b))
    }
}

impl Ord for TimerCondVar {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

lazy_static! {
    static ref TIMERS: UPSafeCell<BinaryHeap<TimerCondVar>> =
        unsafe { UPSafeCell::new(BinaryHeap::<TimerCondVar>::new()) };
}

pub fn add_timer(expire_ns: usize, task: Arc<TaskControlBlock>) {
    let mut timers = TIMERS.exclusive_access();
    timers.push(TimerCondVar { expire_ns, task });
}

pub fn check_timer() {
    let current_ns = get_time_ns();
    println!("cur = {}", current_ns);
    let mut timers = TIMERS.exclusive_access();
    while let Some(timer) = timers.peek() {
        if timer.expire_ns <= current_ns {
            add_task(Arc::clone(&timer.task));
            timers.pop();
        } else {
            break;
        }
    }
}
