#![allow(unused)]

pub const FSIMG_START_PAGENUM : usize = 0x9000_0;
pub const FSIMG_END_PAGENUM : usize = 0xb000_0;

pub const MEMORY_END: usize = 0xc000_0000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 40;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 24;
pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x2000;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const SIGRETURN_TRAMPOLINE: usize = TRAMPOLINE - PAGE_SIZE;
pub const TRAP_CONTEXT_BASE: usize = SIGRETURN_TRAMPOLINE - PAGE_SIZE;

/// 进程用户栈基址
pub const USER_STACK_BASE: usize = 0xf000_0000;

/// mmap基址
pub const MMAP_BASE: usize = 0x8000_0000;

//1G = 0x0-0x3FFF_FFFF    256G = 0x0-0x3F_0000_0000
pub const DYNAMIC_LINKER:usize = 0x30_0000_0000;

// max fd
pub const FDMAX: usize = 1023;

pub use crate::board::{CLOCK_FREQ, MMIO};

#[allow(unused)]
pub fn aligned_up(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE
}

#[allow(unused)]
pub fn aligned_down(addr: usize) -> usize {
    addr / PAGE_SIZE * PAGE_SIZE
}

#[allow(unused)]
pub fn is_aligned(addr: usize) -> bool {
    addr % PAGE_SIZE == 0
}
