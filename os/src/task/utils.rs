use crate::{config::USER_STACK_BASE, mm::translated_ref};

const MAX_BACKTRACE_DEPTH: usize = 20;

pub unsafe fn user_backtrace(token: usize, s0: usize) {
    let mut fp = s0;
    info!("---user backtrace---");
    for i in 0..MAX_BACKTRACE_DEPTH {
        debug!("now fp = {:#x}", fp);
        if fp == USER_STACK_BASE {
            break;
        }
        debug!("#{}:ra={:#x}", i, *(translated_ref(token, (fp - 8) as *const usize)));
        fp = *(translated_ref(token, (fp - 16) as *const usize));
        if fp == 0 {
            warning!("corrupted stack frame");
            break;
        }
    }
    info!("---end  backtrace---");
}
