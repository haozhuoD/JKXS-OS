#![no_std]
#![feature(once_cell)]
extern crate alloc;

mod block_cache;
mod block_dev;
mod layout;
mod fat;
mod fat32_manager;
mod vfs;
mod sbi;

#[macro_use]
mod console;

pub const BLOCK_SZ: usize = 512;
pub use block_dev::BlockDevice;
pub use block_cache::{
    CacheMode,
    get_data_block_cache,
    get_info_block_cache,
    set_start_sector,
    write_to_dev,
    sync_all,
    DATA_BLOCK_CACHE_MANAGER,
    INFO_BLOCK_CACHE_MANAGER
};
pub use layout::*;
pub use fat::FAT;
pub use fat32_manager::*;
pub use vfs::VFile;
