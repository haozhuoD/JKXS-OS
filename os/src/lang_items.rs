use crate::sbi::shutdown;
use crate::task::current_kstack_top;
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("Panicked: {}", info.message().unwrap());
    }
    unsafe {
        backtrace();
    }
    shutdown()
}

unsafe fn backtrace() {
    let mut fp: usize;
    let option_stop = current_kstack_top();
    if let Some(stop) = option_stop {
        asm!("mv {}, s0", out(reg) fp);
        info!("---START BACKTRACE---");
        for i in 0..10 {
            if fp == stop {
                break;
            }
            println!("#{}:ra={:#x}", i, *((fp - 8) as *const usize));
            fp = *((fp - 16) as *const usize);
        }
        info!("---END   BACKTRACE---");
    }
}
