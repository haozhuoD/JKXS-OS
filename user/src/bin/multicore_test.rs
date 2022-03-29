#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exit, fork, wait, get_time};

const MAX_CHILD: usize = 48;
const ITERATIONS: usize = 30000000;
const MAGIC: u32 = 3137;

#[no_mangle]
pub fn main() -> i32 {
    let start = get_time();
    for y in 0..MAX_CHILD {
        let pid = fork();
        if pid == 0 {
            let mut x: u32 = 17777;
            // println!("child is calculating... ");
            for i in 0..ITERATIONS {
                x = x * (i as u32) % MAGIC * y as u32 % MAGIC;
            }
            exit(x as i32);
        } else {
            // println!("forked child pid = {}", pid);
        }
        assert!(pid > 0);
    }

    let mut wstatus = 0;

    for i in 0..MAX_CHILD {
        println!("recycled child {}", i);
        wait(&mut wstatus);
    }

    if wait(&mut wstatus) > 0 {
        panic!("wait got too many");
    }

    let end = get_time();
    println!("multicore_test pass, times = {} ms", end - start);

    0
}
