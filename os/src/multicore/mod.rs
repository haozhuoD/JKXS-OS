use crate::sbi::{sbi_get_hart_status, sbi_hart_start};

#[inline(always)]
pub fn get_hartid() -> usize {
    let mut hartid;
    unsafe {
        core::arch::asm!("mv {}, tp", out(reg) hartid);
    }
    hartid
}

pub fn save_hartid() {
    unsafe {
        // core::arch::asm!("mv tp, x10", in("x10") hartid);
        core::arch::asm!("mv tp, a0");
    }
}

#[allow(unused)]
pub fn wakeup_other_cores(boot_hartid: usize) {
    extern "C" {
        fn skernel();
    }
    let hart_min: usize;
    let hart_max: usize;
    #[cfg(feature = "board_fu740")]
    {
        hart_min = 1;
        hart_max = 4;
    }
    #[cfg(not(any(feature = "board_fu740")))]
    {
        hart_min = 0;
        hart_max = 3;
    }
    for i in hart_min..=hart_max {
        if i != boot_hartid {
            let hart_status = sbi_get_hart_status(i);
            debug!(
                "Wakeup other cores, hartid: {} status:{}",
                i, hart_status as isize
            );
            let ret = sbi_hart_start(i, skernel as usize, 0);
            // while sbi_hart_start(i, skernel as usize, 0)!=0 {

            //根据核状态做不同处理
            // if hart_status==1 {
            //     let ret = sbi_hart_start(i, skernel as usize, 0);
            //     println!("sbi_hart_start hartid: {}  ret: {}", i, ret as isize);
            // } else {
            //     println!("hartid: {}  is not in stopped ", i);
            //     sbi_send_ipi(1<<i);
            //     // println!("hartid: {}  ipi_ret: {} ", i, ipi_ret);
            // }
            if ret as isize == -6 {
                println!(
                    "sbi_hart_start hart:{} is already started  ret: {}  ",
                    i, ret as isize
                );
            }
            // println!("sbi_hart_start hartid: {}  ret: {}", i, ret as isize);
        }
    }
}
