#[cfg(feature = "board_fu740")]
use fu740_hal::clock::PrciExt;
#[cfg(feature = "board_fu740")]
use fu740_pac::Peripherals;
#[cfg(feature = "board_fu740")]
use riscv::register::time;

#[cfg(feature = "board_fu740")]
pub fn time_sleep(n: usize) {
    let start = time::read();
    while time::read() < start + n {}
}

#[cfg(feature = "board_fu740")]
pub fn usleep(n: usize) {
    let peripherals = unsafe { Peripherals::steal() };
    let freq = peripherals.PRCI.get_coreclk();
    time_sleep(freq.0 as usize * n / 1000000);
}

#[cfg(feature = "board_fu740")]
pub fn core_freq(n: usize) -> usize {
    // let freq = peripherals.PRCI.get_coreclk();
    // freq.0
    0
}
