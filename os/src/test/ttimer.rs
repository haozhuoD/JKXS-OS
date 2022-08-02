#![allow(unused)]
#![allow(non_upper_case_globals)]
use spin::rwlock::RwLock;

use crate::timer::get_time_ns;

static ttimer0: RwLock<usize> = RwLock::new(0);
static ttimer1: RwLock<usize> = RwLock::new(0);
static ttimer_en: RwLock<bool> = RwLock::new(false);

pub fn start_ttimer() {
    *ttimer0.write() = get_time_ns();
}

pub fn stop_ttimer() {
    *ttimer1.write() = get_time_ns();
}

pub fn enable_ttimer_output() {
    *ttimer_en.write() = true;
}

pub fn disable_ttimer_output() {
    *ttimer_en.write() = false;
}

pub fn print_ttimer() {
    if (*ttimer_en.read()) {
        let t = *ttimer1.read() - *ttimer0.read();
        debug!("ttimer = {}.{:03} us", t / 1000, t % 1000);
    }
}