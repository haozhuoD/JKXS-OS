#![allow(unused)]

use core::arch::asm;

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;
const SBI_HSM_EXT: usize = 0x48534d;
const HSM_HART_START_FUNID: usize = 0;
const HSM_HART_SUSPEND_FUNID: usize = 3;
const HSM_HART_GET_STATUS_FUNID: usize = 2;
const NONE: usize = 0;

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, args: [usize; 3]) -> (usize, usize) {
    let mut ret1;
    let mut ret2;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret1, //a0
            inlateout("x11") args[1] => ret2,   //a1
            in("x12") args[2],  //a1
            in("x16") fid, //a6
            in("x17") eid, //a7
        );
    }
    (ret1, ret2)
}

pub fn set_timer(timer: usize) {
    sbi_call(SBI_SET_TIMER, NONE, [timer, 0, 0]);
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, NONE, [c, 0, 0]);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, NONE, [0, 0, 0]).0
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, NONE, [0, 0, 0]);
    panic!("It should shutdown!");
}

// todo SBIv2.0之后 send—ipi 改变了
pub fn sbi_send_ipi(mask: usize) {
    sbi_call(SBI_SEND_IPI, NONE, [mask, 0, 0]);
}

pub fn sbi_hart_start(hartid: usize, start_addr: usize, a1: usize) -> usize {
    sbi_call(SBI_HSM_EXT, HSM_HART_START_FUNID, [hartid, start_addr, a1]).0
}

pub fn sbi_get_hart_status(hartid: usize) -> usize {
    sbi_call(SBI_HSM_EXT, HSM_HART_GET_STATUS_FUNID, [hartid, 0, 0]).0
}
