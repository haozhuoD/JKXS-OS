use riscv::register::sstatus::{self, FS};

pub fn init() {
    unsafe {
        sstatus::set_fs(FS::Clean);
    }
}
