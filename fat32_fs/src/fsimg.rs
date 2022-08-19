use spin::Lazy;
use crate::BlockDevice;

use super::{BLOCK_SZ, FSIMG_BASE};
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::RwLock;

pub struct BlockCache {
    block_id: usize,
}

impl BlockCache {
    pub fn new(block_id: usize) -> Self {
        Self {
            block_id,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        (FSIMG_BASE + BLOCK_SZ * self.block_id + offset) as *const u8 as usize
    }

    pub fn get_ref<T>(&self, offset: usize) -> &T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &*(addr as *const T) }
    }

    pub fn get_mut<T>(&mut self, offset: usize) -> &mut T
    where
        T: Sized,
    {
        let type_size = core::mem::size_of::<T>();
        assert!(offset + type_size <= BLOCK_SZ);
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }
}

const MAX_BLK_ID: usize = 65536;
pub struct BlockCacheManager {
    // start_sector: usize,
    // queue: Vec<(usize, Arc<RwLock<BlockCache>>)>,
    list: Vec<Arc<RwLock<BlockCache>>>
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            // start_sector: 0,
            // queue: Vec::new(),
            list: (0..MAX_BLK_ID).map(|block_id| {
                Arc::new(RwLock::new(BlockCache::new(block_id)))
            }).collect::<Vec<Arc<RwLock<BlockCache>>>>(),
        }
    }

    // pub fn set_start_sector(&mut self, start_sector: usize) {
    //     self.start_sector = start_sector;
    // }

    // pub fn get_start_sector(&self) -> usize {
    //     self.start_sector
    // }

    pub fn get_block_cache(
        &self, 
        block_id: usize
    ) -> Arc<RwLock<BlockCache>> {
        self.list[block_id].clone()
    }
}

pub static BLOCK_CACHE_MANAGER: Lazy<BlockCacheManager> =
    Lazy::new(|| BlockCacheManager::new());

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CacheMode {
    READ,
    WRITE,
}

pub fn get_data_block_cache(
    block_id: usize,
    _: Arc<dyn BlockDevice>,
    _: CacheMode,
) -> Arc<RwLock<BlockCache>> {
    // let phy_blk_id = BLOCK_CACHE_MANAGER.get_start_sector() + block_id;
    // BLOCK_CACHE_MANAGER.get_block_cache(phy_blk_id)
    BLOCK_CACHE_MANAGER.get_block_cache(block_id)
}

pub fn get_info_block_cache(
    block_id: usize,
    _: Arc<dyn BlockDevice>,
    _: CacheMode,
) -> Arc<RwLock<BlockCache>> {
    // let phy_blk_id = BLOCK_CACHE_MANAGER.get_start_sector() + block_id;
    // BLOCK_CACHE_MANAGER.get_block_cache(phy_blk_id)
    BLOCK_CACHE_MANAGER.get_block_cache(block_id)
}

pub fn set_start_sector(start_sector: usize) {
    // BLOCK_CACHE_MANAGER
    //     .write()
    //     .set_start_sector(start_sector);
    return ;
}