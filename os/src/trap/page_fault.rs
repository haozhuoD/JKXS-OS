use crate::{
    config::MMAP_BASE,
    mm::VirtAddr,
    task::current_process,
};

fn lazy_alloc_mmap_page(vaddr: usize) -> isize {
    let vpn = VirtAddr::from(vaddr).floor();
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let fd_table = inner.fd_table.clone();
    inner.memory_set.insert_mmap_dataframe(vpn, fd_table)
}

fn lazy_alloc_heap_page(vaddr: usize) -> isize {
    // println!("lazy_alloc_heap_page({:#x?})", vaddr);
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    let user_heap_base = inner.user_heap_base;
    let user_heap_top = inner.user_heap_top;
    inner
        .memory_set
        .insert_heap_dataframe(vaddr, user_heap_base, user_heap_top)
}

pub fn page_fault_handler(vaddr: usize) -> isize {
    let heap_base = current_process().inner_exclusive_access().user_heap_base;
    let heap_top = current_process().inner_exclusive_access().user_heap_top;
    let mmap_top = current_process().inner_exclusive_access().mmap_area_top;

    println!("va = {:#x?}, mmap_top = {:#x?}", vaddr, mmap_top);
    if vaddr >= heap_base && vaddr < heap_top {
        println!("[kernel] alloc heap memory {:#x?}", vaddr);
        lazy_alloc_heap_page(vaddr)
    } else if vaddr >= MMAP_BASE && vaddr < mmap_top {
        println!("[kernel] alloc mmap memory {:#x?}", vaddr);
        lazy_alloc_mmap_page(vaddr)
    } else {
        -1
    }
}
