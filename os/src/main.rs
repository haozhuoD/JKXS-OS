#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(btree_drain_filter)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

#[cfg(feature = "board_fu740")]
#[path = "boards/fu740.rs"]
mod board;
#[cfg(not(any(feature = "board_fu740")))]
#[path = "boards/qemu.rs"]
mod board;


#[macro_use]
mod console;
mod config;
mod drivers;
mod fpu;
mod fs;
mod lang_items;
mod mm;
mod monitor;
mod multicore;
mod sbi;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;
mod loader;

use crate::multicore::{get_hartid, save_hartid};
use core::arch::global_asm;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::sbi::sbi_hart_start;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("userbin.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

static AP_CAN_INIT: AtomicBool = AtomicBool::new(false);
// static mut BOOTHART:isize = -1;

#[no_mangle]
pub fn rust_main() -> ! {
    save_hartid();
    let hartid = get_hartid();
    println!("[kernel] Riscv hartid {} init ", hartid);
    if AP_CAN_INIT.load(Ordering::Relaxed) {
        others_main(hartid);
    }
    clear_bss();
    mm::init();
    mm::remap_test();
    fpu::init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    fs::list_apps();
    task::add_initproc();
    println!("[kernel] Riscv hartid {} run ", hartid);
    AP_CAN_INIT.store(true, Ordering::Relaxed);
    extern "C" {
        fn skernel();
    }
    for i in 0..=3 {
        // println!("i: {}   hartid: {}" ,i,hartid);
        if i!=hartid {
            // println!("sbi_hart_start   hartid: {}" ,i);
            sbi_hart_start(i, skernel as usize, 0);
        }
        // let mask:usize = 1 << i;
        // sbi_send_ipi(&mask as *const usize as usize); 
    }
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

fn others_main(hartid: usize) -> ! {
    println!("[kernel] Riscv hartid {} run ", hartid);
    mm::init_other();
    fpu::init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_tasks();
    panic!("Unreachable in others_main!");
}
