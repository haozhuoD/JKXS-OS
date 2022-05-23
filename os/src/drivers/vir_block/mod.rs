mod virtio_blk;

use spin::Lazy;
pub use virtio_blk::VirtIOBlock;

use crate::board::BlockDeviceImpl;
use alloc::sync::Arc;
use fat32_fs::BlockDevice;

pub static BLOCK_DEVICE: Lazy<Arc<dyn BlockDevice>> =
    Lazy::new(|| Arc::new(BlockDeviceImpl::new()));