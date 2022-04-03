use crate::gdb_println;
use crate::mm::translated_ref;
use crate::monitor::QEMU;
use crate::monitor::SYSCALL_ENABLE;
// use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{current_process, current_user_token, suspend_current_and_run_next};
use crate::timer::{get_time_us, USEC_PER_SEC};
use alloc::sync::Arc;

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

pub fn sys_mutex_create(blocking: bool) -> isize {
    todo!();
    // let process = current_process();
    // let mutex: Option<Arc<dyn Mutex>> = if !blocking {
    //     Some(Arc::new(MutexSpin::new()))
    // } else {
    //     Some(Arc::new(MutexBlocking::new()))
    // };
    // let mut process_inner = process.inner_exclusive_access();
    // if let Some(id) = process_inner
    //     .mutex_list
    //     .iter()
    //     .enumerate()
    //     .find(|(_, item)| item.is_none())
    //     .map(|(id, _)| id)
    // {
    //     process_inner.mutex_list[id] = mutex;
    //     id as isize
    // } else {
    //     process_inner.mutex_list.push(mutex);
    //     process_inner.mutex_list.len() as isize - 1
    // }
}

pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    // drop(process_inner);
    // drop(process);
    // mutex.lock();
    // 0
}

pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    // drop(process_inner);
    // drop(process);
    // mutex.unlock();
    // 0
}

pub fn sys_semaphore_create(res_count: usize) -> isize {
    todo!();
    // let process = current_process();
    // let mut process_inner = process.inner_exclusive_access();
    // let id = if let Some(id) = process_inner
    //     .semaphore_list
    //     .iter()
    //     .enumerate()
    //     .find(|(_, item)| item.is_none())
    //     .map(|(id, _)| id)
    // {
    //     process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count)));
    //     id
    // } else {
    //     process_inner
    //         .semaphore_list
    //         .push(Some(Arc::new(Semaphore::new(res_count))));
    //     process_inner.semaphore_list.len() - 1
    // };
    // id as isize
}

pub fn sys_semaphore_up(sem_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    // drop(process_inner);
    // sem.up();
    // 0
}

pub fn sys_semaphore_down(sem_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    // drop(process_inner);
    // sem.down();
    // 0
}

pub fn sys_condvar_create(_arg: usize) -> isize {
    todo!();
    // let process = current_process();
    // let mut process_inner = process.inner_exclusive_access();
    // let id = if let Some(id) = process_inner
    //     .condvar_list
    //     .iter()
    //     .enumerate()
    //     .find(|(_, item)| item.is_none())
    //     .map(|(id, _)| id)
    // {
    //     process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
    //     id
    // } else {
    //     process_inner
    //         .condvar_list
    //         .push(Some(Arc::new(Condvar::new())));
    //     process_inner.condvar_list.len() - 1
    // };
    // id as isize
}

pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    // drop(process_inner);
    // condvar.signal();
    // 0
}

pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    todo!();
    // let process = current_process();
    // let process_inner = process.inner_exclusive_access();
    // let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    // let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    // drop(process_inner);
    // condvar.wait(mutex);
    // 0
}
