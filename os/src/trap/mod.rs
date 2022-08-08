mod context;

use crate::config::TRAMPOLINE;
use crate::multicore::get_hartid;
use crate::syscall::{syscall, SYSCALL_SIGRETURN};
use crate::task::{
    current_add_signal, current_process, current_tid, current_trap_cx,
    current_user_token, perform_signals_of_current, suspend_current_and_run_next, SIGILL, SIGSEGV, current_trap_cx_user_va,
};
use crate::test::{disable_ttimer_output, start_ttimer, stop_ttimer, print_ttimer};
use crate::timer::set_next_trigger;
use core::arch::{asm, global_asm};
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler() -> ! {
    disable_ttimer_output();
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    let mut is_sigreturn = false;
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            // debug!("syscall sepc = {:#x?}", cx.sepc);
            cx.sepc += 4;
            // get system call return value
            if cx.x[17] == SYSCALL_SIGRETURN {
                is_sigreturn = true;
            }
            let result = syscall(
                cx.x[17],
                [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14], cx.x[15]],
                cx.sepc
            );
            // cx is changed during sys_exec, so we have to call it again
            cx = current_trap_cx();
            cx.x[10] = result as usize;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionFault)
        | Trap::Exception(Exception::InstructionPageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            let process = current_process();
            let mut process_inner = process.acquire_inner_lock();
            if process_inner.check_lazy(stval) == -1 {
                // error!(
                //     "[tid={}] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.",
                //     current_tid(),
                //     scause.cause(),
                //     stval,
                //     current_trap_cx().sepc,
                // );
                // let cx = current_trap_cx();
                // for (i, v) in cx.x.iter().enumerate() {
                //     debug!("x[{}] = {:#x?}", i, v);
                // }
                current_add_signal(SIGSEGV);
            }
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!(
                "[tid={}] {:?} in application, bad addr(stval) = {:#x}, bad instruction(sepc) = {:#x}, kernel killed it.",
                current_tid(),
                scause.cause(),
                stval,
                current_trap_cx().sepc,
            );
            let cx = current_trap_cx();
            for (i, v) in cx.x.iter().enumerate() {
                debug!("x[{}] = {:#x?}", i, v);
            }
            current_add_signal(SIGILL);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    start_ttimer();
    // 处理当前进程的信号
    if !is_sigreturn {
        perform_signals_of_current();
    }
    stop_ttimer();
    print_ttimer("signal");
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
    start_ttimer();
    set_user_trap_entry();
    let trap_cx_user_va = current_trap_cx_user_va();
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;

    // 设置core_id
    current_trap_cx().core_id = get_hartid();

    stop_ttimer();
    print_ttimer("trap_return");

    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_user_va,
            in("a1") user_satp,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    use riscv::register::sepc;
    fatal!("stval = {:#x}, sepc = {:#x}", stval::read(), sepc::read());
    panic!("a trap {:?} from kernel!", scause::read().cause());
}

pub use context::TrapContext;
