#![allow(unused)]
#![allow(non_upper_case_globals)]

use riscv::register::time;
use spin::rwlock::RwLock;

use crate::config::CLOCK_FREQ;

static mut ttimer0: usize = 0;
static mut ttimer1: usize = 0;
static mut ttimer_en: bool = false;

// ttimer时钟自身有一定开销，大约为200-600ns

pub fn start_ttimer() {
    unsafe {
        ttimer0 = time::read();
    }
}

pub fn stop_ttimer() {
    unsafe {
        ttimer1 = time::read();
    }
}

pub fn enable_ttimer_output() {
    unsafe {
        ttimer_en = true;
    }
}

pub fn disable_ttimer_output() {
    unsafe {
        ttimer_en = false;
    }
}

pub fn print_ttimer(msg: &str) {
    unsafe {
        if (ttimer_en) {
            let t = (ttimer1 - ttimer0) * 10000 / (CLOCK_FREQ / 100000);
            debug!("ttimer ({}) = {} ns", msg, t);
        }
    }
}


// pub fn start_ttimer() {
// }

// pub fn stop_ttimer() {
// }

// pub fn enable_ttimer_output() {
// }

// pub fn disable_ttimer_output() {
// }

// pub fn print_ttimer(msg: &str) {
// }