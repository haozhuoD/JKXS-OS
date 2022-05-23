mod sdcard;
mod k210_sdcard;
mod sleep;
mod spi;
mod virtio_blk;


pub use sdcard::SDCardWrapper;
use spin::Lazy;
pub use virtio_blk::VirtIOBlock;

use crate::board::BlockDeviceImpl;
use alloc::sync::Arc;
use fat32_fs::BlockDevice;

// pub trait BlockDevice : Send + Sync {
//     fn read_block(&self, block_id: usize, buf: &mut [u8]);
//     fn write_block(&self, block_id: usize, buf: &[u8]);
//   }

pub static BLOCK_DEVICE: Lazy<Arc<dyn BlockDevice>> =
    Lazy::new(|| Arc::new(BlockDeviceImpl::new()));

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
    for i in 131072..131584 {
        for byte in write_buffer.iter_mut() {
            *byte = i as u8;
        }
        block_device.write_block(i as usize, &write_buffer);
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device 512 blocks loop[ write-read ]  test passed!");
    let mut write_buffer = [66u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 131072..131584 {
        block_device.write_block(i as usize, &write_buffer);
    }
    for i in 131072..131584 {
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    println!("block device 512 blocks test passed!");
}
