use alloc::collections::{BTreeMap, VecDeque};
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Lazy, RwLock};
use core::sync::atomic::{AtomicU32, Ordering};

use crate::gdb_println;
use crate::mm::translated_ref;

use crate::monitor::{QEMU, SYSCALL_ENABLE};
// use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{
    block_current_and_run_next, current_task, current_user_token, suspend_current_and_run_next,
    unblock_task, TaskControlBlock,
};
use crate::timer::{get_time_us, USEC_PER_SEC};

use super::errorno::{EAGAIN, EPERM};

pub fn sys_sleep(req: *mut u64) -> isize {
    let token = current_user_token();
    let sec = *translated_ref(token, req);
    let usec = *translated_ref(token, unsafe { req.add(1) });
    drop(token);

    let t = sec as usize * USEC_PER_SEC + usec as usize;
    let start_time = get_time_us();

    while get_time_us() - start_time < t {
        suspend_current_and_run_next();
    }
    gdb_println!(SYSCALL_ENABLE, "sys_sleep(s: {}, us: {})", sec, usec);
    0
}

pub struct FutexWaiter {
    pub task: Arc<TaskControlBlock>,
    expire_time: usize
}

impl FutexWaiter {
    pub fn new(task: Arc<TaskControlBlock>, current_time: usize, timeout: usize) -> Self {
        Self {
            task,
            expire_time: if current_time <= usize::MAX - timeout { current_time + timeout } else {usize::MAX}
        }
    }

    pub fn check_expire(&self) -> bool {
        // debug!("cur = {}, exp = {}", get_time_us(), self.expire_time);
        get_time_us() >= self.expire_time
    }
}

pub struct FutexQueue {
    pub waiters: RwLock<usize>,
    pub chain: RwLock<VecDeque<FutexWaiter>>,
}

impl FutexQueue {
    pub fn new() -> Self {
        Self {
            waiters: RwLock::new(0),
            chain: RwLock::new(VecDeque::new()),
        }
    }
    pub fn waiters(&self) -> usize {
        *self.waiters.read()
    }
    pub fn waiters_inc(&self) {
        let mut waiters = self.waiters.write();
        *waiters += 1;
    }
    pub fn waiters_dec(&self) {
        let mut waiters = self.waiters.write();
        *waiters -= 1;
    }
}

const FUTEX_WAIT: usize = 0;
const FUTEX_WAKE: usize = 1;
const FUTEX_REQUEUE: usize = 3; 

const FUTEX_PRIVATE_FLAG: usize = 128;
const FUTEX_CLOCK_REALTIME: usize = 256;
const FUTEX_CMD_MASK: usize = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

// const FLAGS_SHARED: usize = 1;
// const FLAGS_CLOCKRT: usize = 2;

pub static FUTEX_QUEUE: Lazy<RwLock<BTreeMap<usize, FutexQueue>>> =
    Lazy::new(|| RwLock::new(BTreeMap::new()));

// Simple implementation
pub fn sys_futex(
    uaddr: *const u32,
    futex_op: usize,
    val: u32,
    timeout: *const u64,
    uaddr2: *const u32,
    val3: u32,
) -> isize {
    let mut _flags = 0;
    let cmd = futex_op & FUTEX_CMD_MASK;
    let token = current_user_token();
    gdb_println!(
        SYSCALL_ENABLE, 
        "*****sys_futex(uaddr: {:#x?}, futex_op: {:x?}, val: {:x?}, timeout: {:#x?}, uaddr2: {:#x?}, val3: {:x?}) = ?", 
        uaddr, 
        futex_op,
        val,
        timeout,
        uaddr2,
        val3,
    );
    // if futex_op & FUTEX_PRIVATE_FLAG == 0 {
    //     flags |= FLAGS_SHARED;
    //     panic!("Todo: mmap shared!");
    // }
    if futex_op & FUTEX_CLOCK_REALTIME != 0 {
        if cmd != FUTEX_WAIT {
            return -EPERM; // ENOSYS
        }
    }
    let ret = match cmd {
        FUTEX_WAIT => {
            let t = if timeout as usize != 0 {
                let sec = *translated_ref(token, timeout);
                let usec = *translated_ref(token, unsafe { timeout.add(1) });
                sec as usize * USEC_PER_SEC + usec as usize
            } else {
                usize::MAX // inf
            };
            futex_wait(uaddr as usize, val, t)
        }
        FUTEX_WAKE => futex_wake(uaddr as usize, val),
        FUTEX_REQUEUE => futex_requeue(uaddr as usize, val, uaddr2 as usize, timeout as u32),
        _ => panic!("ENOSYS"),
    };
    gdb_println!(
        SYSCALL_ENABLE, 
        "sys_futex(uaddr: {:#x?}, futex_op: {:x?}, val: {:x?}, timeout: {:#x?}, uaddr2: {:#x?}, val3: {:x?}) = {}", 
        uaddr, 
        futex_op,
        val,
        timeout,
        uaddr2,
        val3,
        ret,
    );
    return ret;
}

pub fn futex_wait(uaddr: usize, val: u32, timeout: usize) -> isize {
    // futex_wait_setup
    let mut fq_writer = FUTEX_QUEUE.write();
    let flag = fq_writer.contains_key(&uaddr);
    let fq = if flag {
        fq_writer.get(&uaddr).unwrap()
    } else {
        fq_writer.insert(uaddr, FutexQueue::new());
        fq_writer.get(&uaddr).unwrap()
    };
    fq.waiters_inc();
    let mut fq_lock = fq.chain.write();
    let token = current_user_token();
    let uval = translated_ref(token, uaddr as *const AtomicU32);
    // debug!(
    //     "futex_wait: uval: {:x?}, val: {:x?}, timeout: {}",
    //     uval, val, timeout
    // );
    // Ordering is Relaxed
    if uval.load(Ordering::Relaxed) != val { 
        drop(fq_lock);
        fq.waiters_dec();
        if fq.waiters() == 0 {
            fq_writer.remove(&uaddr);
        }
        drop(fq_writer);
        return -EAGAIN;
    }

    // futex_wait_queue_me
    let task = current_task().unwrap();
    fq_lock.push_back(FutexWaiter::new(task.clone(), get_time_us(), timeout));
    drop(fq_lock);
    drop(fq_writer);

    // warning: Auto waking-up has not been implemented yet
    block_current_and_run_next();
    // let start_time = get_time_us();
    // while get_time_us() - start_time < timeout {
    //     suspend_current_and_run_next();
    // }

    // // unqueue_me
    // let mut fq_writer = FUTEX_QUEUE.write();
    // if let Some(fq) = fq_writer.get(&uaddr) {
    //     let len = fq.chain.read().len();
    //     let mut fq_lock = fq.chain.write();
    //     for i in 0..len {
    //         if Arc::ptr_eq(&fq_lock[i], &q) {
    //             fq_lock.remove(i);
    //             break;
    //         }
    //     }
    //     drop(fq_lock);
    //     fq.waiters_dec();
    //     if fq.waiters() == 0 {
    //         fq_writer.remove(&uaddr);
    //     }
    // }
    return 0;
}

pub fn futex_wake(uaddr: usize, nr_wake: u32) -> isize {
    let mut fq_writer = FUTEX_QUEUE.write();
    if !fq_writer.contains_key(&uaddr) {
        return 0;
    }
    let fq = fq_writer.get(&uaddr).unwrap();
    let mut fq_lock = fq.chain.write();
    let waiters = fq.waiters();
    if waiters == 0 {
        return 0;
    }
    let nr_wake = nr_wake.min(waiters as u32);
    // debug!("futex_wake: uaddr: {:x?}, nr_wake: {:x?}", uaddr, nr_wake);

    let mut wakeup_queue = Vec::new();
    (0..nr_wake as usize).for_each(|_| {
        // 加入唤醒队列中，但需要等到释放完锁之后才能唤醒
        let task = fq_lock.pop_front().unwrap().task;
        wakeup_queue.push(task);
        fq.waiters_dec();
    });
    drop(fq_lock);

    if fq.waiters() == 0 {
        fq_writer.remove(&uaddr);
    }

    for task in wakeup_queue.into_iter() {
        unblock_task(task);
    }
    return nr_wake as isize;
}

pub fn futex_requeue(uaddr: usize, nr_wake: u32, uaddr2: usize, nr_limit: u32) -> isize {
    let mut fq_writer = FUTEX_QUEUE.write();
    if !fq_writer.contains_key(&uaddr) {
        return 0;
    }
    let flag2 = fq_writer.contains_key(&uaddr2);
    let fq = fq_writer.get(&uaddr).unwrap();
    let mut fq_lock = fq.chain.write();
    let waiters = fq.waiters();
    if waiters == 0 {
        return 0;
    }
    let nr_wake = nr_wake.min(waiters as u32);

    let mut wakeup_q = Vec::new();
    let mut requeue_q = Vec::new();

    (0..nr_wake as usize).for_each(|_| {
        // prepare to wake-up
        let task = fq_lock.pop_front().unwrap().task;
        wakeup_q.push(task);
        fq.waiters_dec();
    });

    let nr_limit = nr_limit.min(fq.waiters() as u32);
    (0..nr_limit as usize).for_each(|_| {
        // prepare to requeue
        let task = fq_lock.pop_front().unwrap();
        requeue_q.push(task);
        fq.waiters_dec();
    });
    drop(fq_lock);

    // wakeup sleeping tasks
    if fq.waiters() == 0 {
        fq_writer.remove(&uaddr);
    }
    for task in wakeup_q.into_iter() {
        unblock_task(task);
    }

    // requeue...
    if nr_limit == 0 {
        return nr_wake as isize;
    }

    let fq2 = if flag2 {
        fq_writer.get(&uaddr2).unwrap()
    } else {
        fq_writer.insert(uaddr2, FutexQueue::new());
        fq_writer.get(&uaddr2).unwrap()
    };

    let mut fq2_lock = fq2.chain.write();

    for task in requeue_q.into_iter() {
        fq2_lock.push_back(task);
        fq2.waiters_inc();
    }

    return nr_wake as isize;
}