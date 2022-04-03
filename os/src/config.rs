#[allow(unused)]

pub const USER_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_STACK_SIZE: usize = 4096 * 4;
pub const KERNEL_HEAP_SIZE: usize = 0x60_0000;
pub const MEMORY_END: usize = 0x88000000;
pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT_BASE: usize = TRAMPOLINE - PAGE_SIZE;

/// 进程用户栈基址
pub const USER_STACK_BASE: usize = 0xf000_0000;

/// mmap基址
pub const MMAP_BASE: usize = 0x8000_0000;

pub use crate::board::{CLOCK_FREQ, MMIO};

pub fn aligned_up(addr: usize) -> usize {
    (addr + PAGE_SIZE - 1) / PAGE_SIZE * PAGE_SIZE
}

pub fn aligned_down(addr: usize) -> usize {
    addr / PAGE_SIZE * PAGE_SIZE
}

pub fn is_aligned(addr: usize) -> bool {
    addr % PAGE_SIZE == 0
}

pub const MAX_CPU_NUM: usize = 4;
