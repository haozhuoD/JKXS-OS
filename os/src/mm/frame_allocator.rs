use super::{PhysAddr, PhysPageNum};
use crate::config::FSIMG_START_PAGENUM;
use crate::config::FSIMG_END_PAGENUM;
use crate::config::MEMORY_END;

use alloc::vec::Vec;
use core::fmt::{self, Debug, Formatter};
use spin::Lazy;
use spin::RwLock;

pub struct FrameTracker {
    pub ppn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> Self {
        // page cleaning
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum) {
        self.current = l.0;
        self.end = r.0;
        info!("FrameAllocator [0x{:x} - 0x{:x}]", self.current, self.end );
        info!("Remain {} free physical frames", self.end - self.current);
    }
}
impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            if self.current >= (FSIMG_START_PAGENUM - 1) && self.current < FSIMG_END_PAGENUM {
                error!(" -------------------- FrameAllocator outoff 0x9000_0000 ------------------------- ");
                self.current += FSIMG_END_PAGENUM-FSIMG_START_PAGENUM + 1;
            }
            self.current += 1;
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        // validity check
        if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
            panic!("Frame ppn={:#x} has not been allocated!", ppn);
        }
        // recycle
        self.recycled.push(ppn);
    }
}

type FrameAllocatorImpl = StackFrameAllocator;

pub static FRAME_ALLOCATOR: Lazy<RwLock<FrameAllocatorImpl>> =
    Lazy::new(|| RwLock::new(FrameAllocatorImpl::new()));

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    // gdb_println!(
    //     MAPPING_ENABLE,
    //     "[frame_allocator] manage pa[0x{:X} - 0x{:X}]",
    //     PhysAddr::from(ekernel as usize).ceil().0,
    //     PhysAddr::from(MEMORY_END).floor().0
    // );
    FRAME_ALLOCATOR.write().init(
        PhysAddr::from(ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor(),
    );
    // frame_allocator_test();
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR.write().alloc().map(FrameTracker::new)
}

pub fn frame_dealloc(ppn: PhysPageNum) {
    FRAME_ALLOCATOR.write().dealloc(ppn);
}

#[allow(unused)]
pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    // for i in 0..0x10000 {
    for i in 0..0x10 {
        let frame = frame_alloc().unwrap();
        info!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    // for i in 0..0x10000 {
    for i in 0..0x10 {
        let frame = frame_alloc().unwrap();
        info!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    info!("frame_allocator_test passed!");
}
