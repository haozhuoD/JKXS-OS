// use fu740_hal::clock;
// use fu740_hal::prelude::*;
use fu740_hal::clock::PrciExt;
use fu740_pac::Peripherals;
use riscv::register::time;

pub fn time_sleep(n: usize) {
    let start = time::read();
    while time::read() < start + n {}
}

pub fn usleep(n: usize) {
    let peripherals = unsafe { Peripherals::steal() };
    let freq = peripherals.PRCI.get_coreclk();
    time_sleep(freq.0 as usize * n / 1000000);
}
