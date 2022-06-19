mod sdcard;
mod sleep;
mod spi;
mod virtio_blk;
#[cfg(feature = "board_fu740")]
mod clock;

pub use sdcard::SDCardWrapper;
use spin::Lazy;
pub use virtio_blk::VirtIOBlock;

use crate::board::BlockDeviceImpl;
use alloc::sync::Arc;
use fat32_fs::BlockDevice;

#[cfg(feature = "board_fu740")]
pub use clock::core_freq;

pub static BLOCK_DEVICE: Lazy<Arc<dyn BlockDevice>> = Lazy::new(|| Arc::new(BlockDeviceImpl::new()));

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
    info!("block device 512 blocks loop[ write-read ]  test passed!");
    let mut write_buffer = [66u8; 512];
    let mut read_buffer = [0u8; 512];
    for i in 131072..131584 {
        block_device.write_block(i as usize, &write_buffer);
    }
    for i in 131072..131584 {
        block_device.read_block(i as usize, &mut read_buffer);
        assert_eq!(write_buffer, read_buffer);
    }
    info!("block device 512 blocks test passed!");
}
