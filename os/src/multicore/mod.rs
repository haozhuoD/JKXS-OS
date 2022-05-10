use crate::sbi::sbi_hart_start;

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
            println!("sbi_hart_start   hartid: {}" ,i);
            sbi_hart_start(i, skernel as usize, 0);
        }
    }
}
