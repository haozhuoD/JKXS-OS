mod context;

use crate::config::TRAMPOLINE;
use crate::gdb_println;
use crate::monitor::{QEMU, SYSCALL_ENABLE};
use crate::multicore::get_hartid;
use crate::syscall::{SYSCALL_SIGRETURN, SYSCALL_TABLE, SYSCALL_READ, SYSCALL_WRITE, SYSCALL_READDIR};
use crate::task::{
    current_add_signal, current_process, current_tid, current_trap_cx,
    current_user_token, perform_signals_of_current, suspend_current_and_run_next, SIGILL, SIGSEGV, current_trap_cx_user_va,
};
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

extern "C" {
    fn __trap_from_kernel();
}

fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(__trap_from_kernel as usize, TrapMode::Direct);
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
    set_kernel_trap_entry();
    let scause = scause::read();
    let mut is_sigreturn = false;
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // jump to next instruction anyway
            let mut cx = current_trap_cx();
            // debug!("syscall sepc = {:#x?}", cx.sepc);
            cx.sepc += 4;
            let syscall_id = cx.x[17];
            // get system call return value
            if syscall_id == SYSCALL_SIGRETURN {
                is_sigreturn = true;
            }
            let result: usize;
            
            if ((syscall_id != SYSCALL_READ && syscall_id != SYSCALL_WRITE) || (cx.x[10] > 2))
                && syscall_id != SYSCALL_READDIR
            {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "\x1b[034msyscall({}), args = {:x?}, sepc = {:#x?}\x1b[0m",
                    syscall_id,
                    [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14], cx.x[15]],
                    cx.sepc - 4
                );
            }

            unsafe {
                let sysptr = SYSCALL_TABLE[cx.x[17]];
                asm!(
                    "jalr {s}",
                    s = in(reg) sysptr,
                    inlateout("x10") cx.x[10] => result,
                    in("x11") cx.x[11],
                    in("x12") cx.x[12],
                    in("x13") cx.x[13],
                    in("x14") cx.x[14],
                    in("x15") cx.x[15],
                );
            }
            // let result = syscall(
            //     cx.x[17],
            //     [cx.x[10], cx.x[11], cx.x[12], cx.x[13], cx.x[14], cx.x[15]],
            // );
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
            let stval = stval::read();
            let is_store = scause.cause() == Trap::Exception(Exception::StoreFault) || scause.cause() == Trap::Exception(Exception::StorePageFault);
            let process = current_process();
            let mut process_inner = process.acquire_inner_lock();
            let ret_lazy = process_inner.check_lazy(stval);
            let mut ret_cow:isize = 0;
            if is_store && ret_lazy==-1 {
                // info!("[tid={}] is_store cow_handle start ... vaddr:0x{:x}",current_tid(),stval);
                ret_cow = process_inner.cow_handle(stval);
            }
            // let erro =( is_store && (ret_cow==0) ) ? false : (ret_lazy == -1);
            let erro =if is_store && (ret_cow==0) { false } else {ret_lazy == -1};
            if erro {    
                // info!("ret_lazy = {}",ret_lazy);
                // info!("is_store = {}  ret_cow = {} ",is_store, ret_cow);
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
            let stval = stval::read();
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
            let stval = stval::read();
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    // 处理当前进程的信号
    if !is_sigreturn {
        perform_signals_of_current();
    }
    trap_return();
}

#[no_mangle]
pub fn trap_return() -> ! {
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
