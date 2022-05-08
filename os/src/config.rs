#[allow(unused)]

pub const MEMORY_END: usize = 0x8f00_0000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const USER_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const KERNEL_STACK_SIZE: usize = PAGE_SIZE * 4;
pub const KERNEL_HEAP_SIZE: usize = PAGE_SIZE * 0x200;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

/// 进程用户栈基址
pub const USER_STACK_BASE: usize = 0xf000_0000;

/// mmap基址
pub const MMAP_BASE: usize = 0x6000_0000;

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

