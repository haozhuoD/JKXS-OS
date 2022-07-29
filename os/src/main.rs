#![no_std]
#![no_main]
#![feature(once_cell)]
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
mod loader;
mod mm;
mod monitor;
mod multicore;
mod sbi;
mod syscall;
mod task;
mod timer;
mod trap;

use crate::multicore::{get_hartid, save_hartid, wakeup_other_cores};
use core::arch::global_asm;
#[allow(unused)]
use drivers::block_device_test;
use spin::{Lazy, Mutex, RwLock};

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

static BOOT_CORE_READY: Lazy<RwLock<bool>> = Lazy::new(|| RwLock::new(false));
static BOOT_COUNT: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

#[no_mangle]
pub fn rust_main() -> ! {
    save_hartid(); // 这句话之前不能加任何函数调用，否则a0的值会被覆盖
    let hartid = get_hartid();
    info!("Riscv hartid {} init ", hartid);
    if *(BOOT_CORE_READY.read()) {
        // 如果BOOT_CORE已经准备完毕，则其他核通过others_main启动。否则说明是启动核，直接fall through
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
    fs::init_rootfs();
    // block_device_test();
    task::add_initproc();
    mm::load_libc_so();
    info!("(Boot Core) Riscv hartid {} run ", hartid);
    // // get core_clock
    // #[cfg(feature = "board_fu740")]
    // {
    //     let core_f = drivers::core_freq();
    //     info!("core_freq is 0x{:X} ", core_f);
    // }

    *(BOOT_CORE_READY.write()) = true;
    // wakeup_other_cores(hartid);

    // while *(BOOT_COUNT.lock()) != 2 {};
    task::run_tasks();
    panic!("Unreachable in rust_main!");
}

fn others_main(hartid: usize) -> ! {
    mm::init_other();
    fpu::init();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    info!("(Other Cores) Riscv hartid {} run ", hartid);
    {
        let mut boot_count = BOOT_COUNT.lock();
        // println!("==== boot_count++ before:{:?} ==== ",boot_count);
        *boot_count += 1;
    }
    task::run_tasks();
    panic!("Unreachable in others_main!");
}
