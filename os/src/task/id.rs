use super::ProcessControlBlock;
use crate::config::{KERNEL_STACK_SIZE, PAGE_SIZE, TRAMPOLINE, TRAP_CONTEXT_BASE, USER_STACK_SIZE};
use crate::mm::{MapPermission, PhysPageNum, VirtAddr, KERNEL_SPACE, MmapArea, MapAreaType};

use crate::gdb_println;
use crate::monitor::{MAPPING_ENABLE, QEMU};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use spin::{Lazy, RwLock};

pub struct RecycleAllocator {
    current: usize,
    recycled: Vec<usize>,
}

impl RecycleAllocator {
    pub fn new() -> Self {
        RecycleAllocator {
            current: 0,
            recycled: Vec::with_capacity(0x1000),
        }
    }
    pub fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
            id
        } else {
            self.current += 1;
            self.current - 1
        }
    }
    // pub fn dealloc(&mut self, id: usize) {
    //     assert!(id < self.current);
    //     assert!(
    //         !self.recycled.iter().any(|i| *i == id),
    //         "id {} has been deallocated!",
    //         id
    //     );
    //     self.recycled.push(id);
    // }
}

static TID_ALLOCATOR: Lazy<RwLock<RecycleAllocator>> =
    Lazy::new(|| RwLock::new(RecycleAllocator::new()));
static KSTACK_ALLOCATOR: Lazy<RwLock<RecycleAllocator>> =
    Lazy::new(|| RwLock::new(RecycleAllocator::new()));

pub struct TidHandle(pub usize);

pub fn tid_alloc() -> TidHandle {
    TidHandle(TID_ALLOCATOR.write().alloc())
}

// impl Drop for TidHandle {
//     fn drop(&mut self) {
//         TID_ALLOCATOR.write().dealloc(self.0);
//     }
// }

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(kstack_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - kstack_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}

pub struct KernelStack(pub usize);

pub fn kstack_alloc() -> KernelStack {
    let kstack_id = KSTACK_ALLOCATOR.write().alloc();
    let (kstack_bottom, kstack_top) = kernel_stack_position(kstack_id);
    gdb_println!(
        MAPPING_ENABLE,
        "[kstack-map] kstack_id:{}  va[0x{:X} - 0x{:X}] Framed",
        kstack_id,
        kstack_bottom,
        kstack_top
    );
    KERNEL_SPACE.write().insert_framed_area(
        MapAreaType::KernelStack,
        kstack_bottom.into(),
        kstack_top.into(),
        MapPermission::R | MapPermission::W,
    );
    KernelStack(kstack_id)
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let (kernel_stack_bottom, _) = kernel_stack_position(self.0);
        let kernel_stack_bottom_va: VirtAddr = kernel_stack_bottom.into();
        KERNEL_SPACE
            .write()
            .remove_area_with_start_vpn(kernel_stack_bottom_va.into());
    }
}

impl KernelStack {
    pub fn get_top(&self) -> usize {
        let (_, kernel_stack_top) = kernel_stack_position(self.0);
        kernel_stack_top
    }
}

pub struct TaskUserRes {
    pub tid: TidHandle,
    pub rel_tid: usize, // 相对主线程的tid值（主线程为0，其余的为1, 2, 3, ...)
    pub ustack_base: usize,
    pub process: Weak<ProcessControlBlock>,
}

fn trap_cx_bottom_from_tid(rel_tid: usize) -> usize {
    // debug!("trap_cx_bottom_from_tid: rel_tid = {}", rel_tid);
    TRAP_CONTEXT_BASE - rel_tid * PAGE_SIZE
}

fn ustack_bottom_from_tid(ustack_base: usize, rel_tid: usize) -> usize {
    ustack_base + rel_tid * (PAGE_SIZE + USER_STACK_SIZE)
}

impl TaskUserRes {
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        pid: isize,
        alloc_user_res: bool,
    ) -> Self {
        let tid = tid_alloc();
        let rel_tid = if pid < 0 { 0 } else { tid.0 - pid as usize };
        let task_user_res = Self {
            tid,
            rel_tid,
            ustack_base,
            process: Arc::downgrade(&process),
        };
        if alloc_user_res {
            task_user_res.alloc_user_res();
        }
        task_user_res
    }

    pub fn alloc_user_res(&self) {
        let process = self.process.upgrade().unwrap();
        let mut process_inner = process.acquire_inner_lock();
        // alloc user stack
        let ustack_bottom = ustack_bottom_from_tid(self.ustack_base, self.rel_tid);
        let ustack_top = ustack_bottom + USER_STACK_SIZE;
        gdb_println!(
            MAPPING_ENABLE,
            "[user-stack-map] tid:{} va[0x{:X} - 0x{:X}]",
            self.tid.0,
            ustack_bottom,
            ustack_top
        );
        process_inner.memory_set.insert_framed_area(
            MapAreaType::UserStack,
            ustack_bottom.into(),
            ustack_top.into(),
            MapPermission::R | MapPermission::W | MapPermission::U,
        );
        // alloc trap_cx
        let trap_cx_bottom = trap_cx_bottom_from_tid(self.rel_tid);
        let trap_cx_top = trap_cx_bottom + PAGE_SIZE;
        gdb_println!(
            MAPPING_ENABLE,
            "[trap_cx-map] onepage va[0x{:X} - 0x{:X}]",
            trap_cx_bottom,
            trap_cx_top
        );
        process_inner.memory_set.insert_framed_area(
            MapAreaType::TrapContext,
            trap_cx_bottom.into(),
            trap_cx_top.into(),
            MapPermission::R | MapPermission::W,
        );
    }

    fn dealloc_user_res(&self) {
        // dealloc tid
        let process = self.process.upgrade().unwrap();
        let mut process_inner = process.acquire_inner_lock();
        // dealloc ustack manually
        let ustack_bottom_va: VirtAddr =
            ustack_bottom_from_tid(self.ustack_base, self.rel_tid).into();
        process_inner
            .memory_set
            .remove_area_with_start_vpn(ustack_bottom_va.into());
        // dealloc trap_cx manually
        let trap_cx_bottom_va: VirtAddr = trap_cx_bottom_from_tid(self.rel_tid).into();
        process_inner
            .memory_set
            .remove_area_with_start_vpn(trap_cx_bottom_va.into());
    }

    pub fn trap_cx_user_va(&self) -> usize {
        trap_cx_bottom_from_tid(self.rel_tid)
    }

    pub fn trap_cx_ppn(&self) -> PhysPageNum {
        let process = self.process.upgrade().unwrap();
        let process_inner = process.acquire_inner_lock();
        let trap_cx_bottom_va: VirtAddr = trap_cx_bottom_from_tid(self.rel_tid).into();
        process_inner
            .memory_set
            .translate(trap_cx_bottom_va.into())
            .unwrap()
            .ppn()
    }

    pub fn ustack_base(&self) -> usize {
        self.ustack_base
    }

    pub fn ustack_top(&self) -> usize {
        ustack_bottom_from_tid(self.ustack_base, self.rel_tid) + USER_STACK_SIZE
    }
}

impl Drop for TaskUserRes {
    fn drop(&mut self) {
        self.dealloc_user_res();
    }
}
