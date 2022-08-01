use crate::{config::MMAP_BASE, mm::VirtAddr, task::ProcessInnerLock};

fn lazy_alloc_mmap_page(process_inner: &mut ProcessInnerLock, vaddr: usize) -> isize {
    let vpn = VirtAddr::from(vaddr).floor();
    process_inner.memory_set
        .insert_mmap_dataframe(vpn)
}

fn lazy_alloc_heap_page(process_inner: &mut ProcessInnerLock, vaddr: usize) -> isize {
    // println!("lazy_alloc_heap_page({:#x?})", vaddr);
    let user_heap_base = process_inner.user_heap_base;
    let user_heap_top = process_inner.user_heap_top;
    process_inner
        .memory_set
        .insert_heap_dataframe(vaddr, user_heap_base, user_heap_top)
}

pub fn page_fault_handler(process_inner: &mut ProcessInnerLock, vaddr: usize) -> isize {
    if vaddr == 0 {
        error!("Assertion failed in user space");
        return -1;
    }
    let heap_base = process_inner.user_heap_base;
    let heap_top = process_inner.user_heap_top;
    let mmap_top = process_inner.mmap_area_top;

    // debug!("page fault: va = {:#x?}", vaddr);
    if vaddr >= heap_base && vaddr < heap_top {
        // println!("[kernel] lazy_alloc heap memory {:#x?}", vaddr);
        lazy_alloc_heap_page(process_inner, vaddr)
    } else if vaddr >= MMAP_BASE && vaddr < mmap_top {
        // println!("[kernel] lazy_alloc mmap memory {:#x?}", vaddr);
        lazy_alloc_mmap_page(process_inner, vaddr)
    } else {
        -1
    }
}
