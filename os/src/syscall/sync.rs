use crate::gdb_println;
use crate::mm::translated_ref;

use crate::monitor::{QEMU, SYSCALL_ENABLE};
// use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{current_user_token, suspend_current_and_run_next};
use crate::timer::{get_time_us, USEC_PER_SEC};

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
