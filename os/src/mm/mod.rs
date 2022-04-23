mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod mmap;
mod page_table;

use address::VPNRange;
pub use address::{PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
use core::arch::asm;
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_token, MapPermission, MemorySet, KERNEL_SPACE};
pub use mmap::MmapArea;
use page_table::PTEFlags;
pub use page_table::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, PageTable,
    PageTableEntry, UserBuffer, UserBufferIterator,
};
use riscv::register::satp;

pub fn init() {
    println!("mm init 1");
    heap_allocator::init_heap();
    println!("mm init 2");
    frame_allocator::init_frame_allocator();
    println!("mm init 3");
    KERNEL_SPACE.lock().activate();
    println!("mm init 4");
}

pub fn init_other() {
    // KERNEL_SPACE.lock().activate_other();
    unsafe {
        satp::write(memory_set::SATP);
        asm!("sfence.vma");
    }
}
