#![no_std]
#![no_main]

use user_lib::brk;

#[macro_use]
extern crate user_lib;
extern crate alloc;

const MAGIC: usize = 0x12345678;
#[no_mangle]
pub fn main() -> i32 {
    let addr0 = brk(0);
    // make sure initial `brk` is aligned 
    assert_eq!(addr0 % 1024, 0);
    println!("before brk, heap_top = {:#x?}", addr0);
    let addr1 = brk(addr0 as usize + 160);
    println!("after brk, heap_top = {:#x?}", addr1);

    // assert 1
    assert_eq!(addr1 - addr0, 160);
    println!("try to write...");
    let ptr = (addr0 + 48) as usize as *mut usize;
    unsafe {
        ptr.write_volatile(MAGIC);
        // assert 2
        assert_eq!(ptr.read_volatile(), MAGIC);
    }
    println!("Heap_test ok. Now shrink the heap, and try to cause a page fault...");

    let ret = brk(addr0 as usize);
    assert_eq!(ret, addr0);

    unsafe {
        (addr0 as usize as *mut usize).write_volatile(MAGIC);
    }
    panic!("Should not reach here!");
}
