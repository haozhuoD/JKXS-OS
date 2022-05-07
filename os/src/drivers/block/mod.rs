mod sdcard;
mod virtio_blk;
mod spi;
mod sleep;

pub use sdcard::SDCardWrapper;
pub use virtio_blk::VirtIOBlock;

use crate::board::BlockDeviceImpl;
use alloc::sync::Arc;
use fat32_fs::BlockDevice;
use lazy_static::*;

// pub trait BlockDevice : Send + Sync {
//     fn read_block(&self, block_id: usize, buf: &mut [u8]);
//     fn write_block(&self, block_id: usize, buf: &[u8]);
//   }

lazy_static! {
    pub static ref BLOCK_DEVICE: Arc<dyn BlockDevice> = Arc::new(BlockDeviceImpl::new());
    // pub static ref BLOCK_DEVICE: Arc<sdcard::SDCardWrapper> = Arc::new(sdcard::SDCardWrapper::new());
}

// pub fn init_sdcard() {
//     // println!("init sdcard start!");
//     BLOCK_DEVICE.init();
//   }
  
// pub fn read_block(block_id: usize, buf: &mut [u8]) {
//   BLOCK_DEVICE.read_block(block_id, buf);
// }
// pub fn write_block(block_id: usize, buf: &[u8]) {
//   BLOCK_DEVICE.write_block(block_id, buf);
// }


#[allow(unused)]
pub fn block_device_test() {
    let block_device = BLOCK_DEVICE.clone();
    let mut write_buffer = [0u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 0..512 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device test passed!");
}
