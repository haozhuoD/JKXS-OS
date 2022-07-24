use super::BlockDevice;

use alloc::sync::Arc;
use spin::Mutex;

#[allow(unused)]
const VIRTIO0: usize = 0x10001000;
const FSIMG_BASE: usize = 0x90000000;
const BYTES_PER_SECTOR: usize = 512;

pub struct VirtIOFSImg;

impl BlockDevice for VirtIOFSImg {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        unsafe {
            buf.copy_from_slice(core::slice::from_raw_parts(
                (FSIMG_BASE + BYTES_PER_SECTOR * block_id) as *const u8,
                BYTES_PER_SECTOR,
            ));
        }
    }
    fn write_block(&self, block_id: usize, buf: &[u8]) {
        unsafe {
            core::slice::from_raw_parts_mut(
                (FSIMG_BASE + BYTES_PER_SECTOR * block_id) as *mut u8,
                BYTES_PER_SECTOR,
            )
            .copy_from_slice(buf);
        }
    }
}

impl VirtIOFSImg {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {}
    }
}