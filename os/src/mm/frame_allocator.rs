use super::{PhysAddr, PhysPageNum};
use crate::config::FSIMG_START_PAGENUM;
use crate::config::FSIMG_END_PAGENUM;
use crate::config::MEMORY_END;

use alloc::collections::BTreeMap;
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
        ppn.clear_page();
        Self { ppn }
    }
    pub fn from_ppn(ppn: PhysPageNum) -> Self {
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
    fn add_ref(&mut self, ppn: PhysPageNum);
    fn reduce_ref(&mut self, ppn: PhysPageNum);
    fn enquire_ref(& self, ppn: PhysPageNum)-> usize;
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>,
    refcounter: BTreeMap<usize, u8>,
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
            recycled: Vec::with_capacity(0x10000),
            refcounter: BTreeMap::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(ppn) = self.recycled.pop() {
            // error!("+++ alloc ppn={:?}",ppn);
            self.refcounter.insert(ppn, 1);
            Some(ppn.into())
        } else if self.current == self.end {
            None
        } else {
            if self.current >= (FSIMG_START_PAGENUM - 1) && self.current < FSIMG_END_PAGENUM {
                error!(" -------------------- FrameAllocator outoff 0x9000_0000 ------------------------- ");
                self.current += FSIMG_END_PAGENUM-FSIMG_START_PAGENUM + 1;
            }
            self.current += 1;
            self.refcounter.insert(self.current - 1, 1);
            // error!("+++ alloc ppn={:?}",self.current - 1);
            Some((self.current - 1).into())
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let ppn = ppn.0;
        if let Some(ref_times) = self.refcounter.get_mut(&ppn){
            *ref_times -= 1;
            if *ref_times == 0 {
                // info!("dealloc drop ppn:{:x}",ppn);
                self.refcounter.remove(&ppn);
                // validity check
                if ppn >= self.current || self.recycled.iter().any(|&v| v == ppn) {
                    panic!("Frame ppn={:#x} has not been allocated!", ppn);
                }
                // recycle
                self.recycled.push(ppn);
            }
            // else {
            //     // info!("dealloc reduce_ref ppn:{:x}",ppn);
            // }
        }else {
            error!("dealloc ppn={:#x} no ref_times", ppn);
        }
    }
    fn add_ref(&mut self, ppn: PhysPageNum) {
        // info!("add_ref ppn:{:?}",ppn);
        let ppn = ppn.0; 
        let ref_times = self.refcounter.get_mut(&ppn).unwrap();
        *ref_times += 1;
        // if *ref_times>=3 {
        //     println!("ref_times >=3");
        // }
    }
    fn reduce_ref(&mut self, ppn: PhysPageNum) {
        // info!("reduce_ref ppn:{:?}",ppn);
        let ppn = ppn.0; 
        let ref_times = self.refcounter.get_mut(&ppn).unwrap();
        *ref_times -= 1;
    }
    fn enquire_ref(&self, ppn: PhysPageNum) -> usize{
        let ppn = ppn.0; 
        let ref_times = self.refcounter.get(&ppn).unwrap();
        (*ref_times).clone() as usize
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

pub fn frame_add_ref(ppn: PhysPageNum) {
    FRAME_ALLOCATOR
        .write()
        .add_ref(ppn)
}

#[allow(unused)]
pub fn frame_reduce_ref(ppn: PhysPageNum) {
    FRAME_ALLOCATOR
        .write()
        .reduce_ref(ppn)
}

pub fn frame_enquire_ref(ppn: PhysPageNum) -> usize {
    FRAME_ALLOCATOR
        .read()
        .enquire_ref(ppn)
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
