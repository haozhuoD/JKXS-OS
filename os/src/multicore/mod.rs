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
