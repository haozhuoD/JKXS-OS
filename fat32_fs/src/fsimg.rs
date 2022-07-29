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

pub struct BlockCacheManager {
    start_sector: usize,
    queue: Vec<(usize, Arc<RwLock<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            start_sector: 0,
            queue: Vec::new(),
        }
    }

    pub fn set_start_sector(&mut self, start_sector: usize) {
        self.start_sector = start_sector;
    }

    pub fn get_start_sector(&self) -> usize {
        self.start_sector
    }

    pub fn read_block_cache(
        &self, 
        block_id: usize
    ) -> Option<Arc<RwLock<BlockCache>>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            Some(Arc::clone(&pair.1))
        } else {
            None
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
    ) -> Arc<RwLock<BlockCache>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            Arc::clone(&pair.1)
        } else {
            let block_cache = Arc::new(RwLock::new(BlockCache::new(
                block_id,
            )));
            self.queue.push((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }
}

pub static DATA_BLOCK_CACHE_MANAGER: Lazy<RwLock<BlockCacheManager>> =
    Lazy::new(|| RwLock::new(BlockCacheManager::new()));

pub static INFO_BLOCK_CACHE_MANAGER: Lazy<RwLock<BlockCacheManager>> =
    Lazy::new(|| RwLock::new(BlockCacheManager::new()));


#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CacheMode {
    READ,
    WRITE,
}

pub fn get_data_block_cache(
    block_id: usize,
    _: Arc<dyn BlockDevice>,
    rw_mode: CacheMode,
) -> Arc<RwLock<BlockCache>> {
    let phy_blk_id = DATA_BLOCK_CACHE_MANAGER.read().get_start_sector() + block_id;
    if rw_mode == CacheMode::READ {
        let rlock = DATA_BLOCK_CACHE_MANAGER.read();
        match rlock.read_block_cache(phy_blk_id) {
            Some(blk) => blk,
            None => {
                drop(rlock);
                DATA_BLOCK_CACHE_MANAGER
                    .write()
                    .get_block_cache(phy_blk_id);
                DATA_BLOCK_CACHE_MANAGER
                    .read()
                    .read_block_cache(phy_blk_id)
                    .unwrap()
            }
        }
    } else {
        DATA_BLOCK_CACHE_MANAGER
            .write()
            .get_block_cache(phy_blk_id)
    }
}

pub fn get_info_block_cache(
    block_id: usize,
    _: Arc<dyn BlockDevice>,
    rw_mode: CacheMode,
) -> Arc<RwLock<BlockCache>> {
    let phy_blk_id = INFO_BLOCK_CACHE_MANAGER.read().get_start_sector() + block_id;
    if rw_mode == CacheMode::READ {
        let rlock = INFO_BLOCK_CACHE_MANAGER.read();
        match rlock.read_block_cache(phy_blk_id) {
            Some(blk) => blk,
            None => {
                drop(rlock);
                INFO_BLOCK_CACHE_MANAGER
                    .write()
                    .get_block_cache(phy_blk_id);
                INFO_BLOCK_CACHE_MANAGER
                    .read()
                    .read_block_cache(phy_blk_id)
                    .unwrap()
            }
        }
    } else {
        INFO_BLOCK_CACHE_MANAGER
            .write()
            .get_block_cache(phy_blk_id)
    }
}

pub fn set_start_sector(start_sector: usize) {
    INFO_BLOCK_CACHE_MANAGER
        .write()
        .set_start_sector(start_sector);
    DATA_BLOCK_CACHE_MANAGER
        .write()
        .set_start_sector(start_sector);
}