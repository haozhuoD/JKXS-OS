pub const CLOCK_FREQ: usize = 125000;

pub const MMIO: &[(usize, usize)] = &[(0x10001000, 0x1000)];

pub type BlockDeviceImpl = crate::drivers::block::VirtIOBlock;

pub const MAX_CPU_NUM: usize = 4;
