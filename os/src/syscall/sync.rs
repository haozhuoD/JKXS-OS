use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{Lazy, RwLock};

use crate::gdb_println;
use crate::mm::translated_ref;

use crate::monitor::{QEMU, SYSCALL_ENABLE};
// use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{current_user_token, suspend_current_and_run_next, TaskControlBlock, current_task, TaskContext, TaskStatus, schedule};
use crate::timer::{get_time_us, USEC_PER_SEC};

use super::errorno::{EPERM, EAGAIN};

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

pub struct FutexQueue {
    waiters: RwLock<usize>,
    // chain: RwLock<Vec<FutexQ>>,
    chain: RwLock<Vec<Arc<TaskControlBlock>>>,
}

impl FutexQueue {
    pub fn new() -> Self {
        Self { 
            waiters: RwLock::new(0), 
            chain: RwLock::new(Vec::new())
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

// struct FutexQ {
//     task: Arc<TaskControlBlock>,
// }

// impl FutexQ {
//     pub fn new(task: Arc<TaskControlBlock>) -> Self {
//         Self { 
//             task 
//         }
//     }
// }

const FUTEX_WAIT: usize = 0;
const FUTEX_WAKE: usize = 1;
const FUTEX_PRIVATE_FLAG: usize = 128;
const FUTEX_CLOCK_REALTIME: usize = 256;
const FUTEX_CMD_MASK: usize = !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);

const FLAGS_SHARED: usize = 1;
const FLAGS_CLOCKRT: usize = 2;

pub static FUTEX_QUEUE: Lazy<RwLock<BTreeMap<u32, FutexQueue>>> = 
    Lazy::new(|| RwLock::new(BTreeMap::new()));

// Simple implementation
pub fn sys_futex(
    uaddr: *const u32,
    futex_op: usize,
    val: u32,
    timeout: *const u64,
    uaddr2: *const u32,
    val3: u32
) -> isize {
    let mut flags = 0;
    let cmd = futex_op & FUTEX_CMD_MASK;
    let token = current_user_token();
    let t;
    if timeout as usize != 0 {
        let sec = *translated_ref(token, timeout);
        let usec = *translated_ref(token, unsafe { timeout.add(1) });
        t = sec as usize * USEC_PER_SEC + usec as usize;
    } else {
        t = !0;
    }
    if futex_op & FUTEX_PRIVATE_FLAG == 0 {
        flags |= FLAGS_SHARED;
        panic!("Todo: mmap shared!");
    }
    if futex_op & FUTEX_CLOCK_REALTIME != 0 {
        if cmd != FUTEX_WAIT {
            return -EPERM;  // ENOSYS
        }
    }
    let ret = match cmd {
        FUTEX_WAIT => futex_wait(uaddr, val, t),
        FUTEX_WAKE => futex_wake(uaddr, val),
        _ => -1,
    };
    if ret < 0 {
        panic!("ENOSYS");
    }
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

fn futex_wait(
    uaddr: *const u32,
    val: u32,
    timeout: usize,
) -> isize {
    // futex_wait_setup
    let flag = FUTEX_QUEUE.read().contains_key(&val);
    let mut fq_writer = FUTEX_QUEUE.write();
    let fq = if flag {
        fq_writer.get(&val).unwrap()
    } else {
        fq_writer.insert(val, FutexQueue::new());
        fq_writer.get(&val).unwrap()
    };
    fq.waiters_inc();
    let mut fq_lock = fq.chain.write();
    let token = current_user_token();
    let uval = translated_ref(token, uaddr);  // Need to be atomic
    println!("uval: {:x?}, val: {:x?}", uval, val);
    if *uval != val {
        drop(fq_lock);
        fq.waiters_dec();
        if fq.waiters() == 0 {
            fq_writer.remove(&val);
        }
        drop(fq_writer);
        return -EAGAIN;
    }

    // futex_wait_queue_me
    let task = current_task().unwrap();
    let q = task.clone();
    fq_lock.push(q.clone());
    drop(fq_lock);
    drop(fq_writer);
    let start_time = get_time_us();
    while get_time_us() - start_time < timeout {
        suspend_current_and_run_next();
    }

    // unqueue_me
    let mut fq_writer = FUTEX_QUEUE.write();
    if let Some(fq) = fq_writer.get(&val) {
        let len = fq.chain.read().len();
        let mut fq_lock = fq.chain.write();
        (0..len).for_each(|i| {
            if Arc::ptr_eq(&fq_lock[i], &q) {
                fq_lock.remove(i);
            }
        });
        drop(fq_lock);
        fq.waiters_dec();
        if fq.waiters() == 0 {
            fq_writer.remove(&val);
        }
    }
    return 0;
}

fn futex_wake(
    uaddr: *const u32,
    nr_wake: u32,
) -> isize {
    let token = current_user_token();
    let uval = translated_ref(token, uaddr);
    if !FUTEX_QUEUE.read().contains_key(uval) {
        return 0;
    }
    let mut fq_writer = FUTEX_QUEUE.write();
    let fq = fq_writer.get(uval).unwrap();
    let mut fq_lock = fq.chain.write();
    let waiters = fq.waiters();
    if waiters == 0 {
        return 0;
    }
    let nr_wake = nr_wake.min(waiters as u32);
    (0..nr_wake as usize).for_each(|i| {
        // 加入唤醒队列中，但需要等到释放完锁之后才能唤醒
        fq_lock.remove(i);
        fq.waiters_dec();
    });
    drop(fq_lock);
    // todo: wake up
    if fq.waiters() == 0 {
        fq_writer.remove(uval);
    }
    return nr_wake as isize;
}