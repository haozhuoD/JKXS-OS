use spin::Lazy;
use super::{BlockDevice, BLOCK_SZ};
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::RwLock;

pub struct BlockCache {
    cache: [u8; BLOCK_SZ],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    /// Load a new BlockCache from disk.
    pub fn new(block_id: usize, block_device: Arc<dyn BlockDevice>) -> Self {
        let mut cache = [0u8; BLOCK_SZ];
        block_device.read_block(block_id, &mut cache);
        Self {
            cache,
            block_id,
            block_device,
            modified: false,
        }
    }

    fn addr_of_offset(&self, offset: usize) -> usize {
        &self.cache[offset] as *const _ as usize
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
        self.modified = true;
        let addr = self.addr_of_offset(offset);
        unsafe { &mut *(addr as *mut T) }
    }

    // 在BlockCache缓冲区偏移量为offset的位置获取一个
    // 类型为T的磁盘上数据结构的不可变引用，并让它执行传入
    // 的闭包f中所定义的操作。
    pub fn read<T, V>(&self, offset: usize, f: impl FnOnce(&T) -> V) -> V {
        f(self.get_ref(offset))
    }

    // 在BlockCache缓冲区偏移量为offset的位置获取一个
    // 类型为T的磁盘上数据结构的可变引用，并让它执行传入
    // 的闭包f中所定义的操作。
    pub fn modify<T, V>(&mut self, offset: usize, f: impl FnOnce(&mut T) -> V) -> V {
        f(self.get_mut(offset))
    }

    pub fn sync(&mut self) {
        if self.modified {
            self.modified = false;
            self.block_device.write_block(self.block_id, &self.cache);
        }
    }
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        self.sync()
    }
}

const BLOCK_CACHE_SIZE: usize = 10;

pub struct BlockCacheManager {
    start_sector: usize,
    queue: VecDeque<(usize, Arc<RwLock<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        Self {
            start_sector: 0,
            queue: VecDeque::new(),
        }
    }

    pub fn set_start_sector(&mut self, start_sector: usize) {
        self.start_sector = start_sector;
    }

    pub fn get_start_sector(&self) -> usize {
        self.start_sector
    }

    pub fn read_block_cache(&self, block_id: usize) -> Option<Arc<RwLock<BlockCache>>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            // println!("[read_block_cache] find");
            Some(Arc::clone(&pair.1))
        } else {
            // println!("[read_block_cache] nofind");
            None
        }
    }

    pub fn get_block_cache(
        &mut self,
        block_id: usize,
        block_device: Arc<dyn BlockDevice>,
    ) -> Arc<RwLock<BlockCache>> {
        if let Some(pair) = self.queue.iter().find(|pair| pair.0 == block_id) {
            // println!("[get_block_cache] find block_id:{}",block_id);
            Arc::clone(&pair.1)
        } else {
            // println!("[get_block_cache] nofind block_id:{}",block_id);
            // substitute
            if self.queue.len() == BLOCK_CACHE_SIZE {
                // from front to tail
                if let Some((idx, _)) = self
                    .queue
                    .iter()
                    .enumerate()
                    .find(|(_, pair)| Arc::strong_count(&pair.1) == 1)
                {
                    self.queue.drain(idx..=idx);
                } else {
                    panic!("Run out of BlockCache!");
                }
            }
            // load block into mem and push back
            let block_cache = Arc::new(RwLock::new(BlockCache::new(
                block_id,
                Arc::clone(&block_device),
            )));
            self.queue.push_back((block_id, Arc::clone(&block_cache)));
            block_cache
        }
    }

    pub fn drop_all(&mut self) {
        self.queue.clear();
    }

    pub fn sync_all(&self) {
        self.queue.iter().for_each(|pair| pair.1.write().sync());
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
    block_device: Arc<dyn BlockDevice>,
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
                    .get_block_cache(phy_blk_id, block_device);
                DATA_BLOCK_CACHE_MANAGER
                    .read()
                    .read_block_cache(phy_blk_id)
                    .unwrap()
            }
        }
    } else {
        DATA_BLOCK_CACHE_MANAGER
            .write()
            .get_block_cache(phy_blk_id, block_device)
    }
}

pub fn get_info_block_cache(
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    rw_mode: CacheMode,
) -> Arc<RwLock<BlockCache>> {
    let phy_blk_id = INFO_BLOCK_CACHE_MANAGER.read().get_start_sector() + block_id;
    // println!("[get_info_block_cache]  block_id:{}  phy_blk_id:{}",block_id,phy_blk_id);
    if rw_mode == CacheMode::READ {
        // println!("   mode:READ");
        let rlock = INFO_BLOCK_CACHE_MANAGER.read();
        match rlock.read_block_cache(phy_blk_id) {
            Some(blk) => blk,
            None => {
                // println!("try to load block");
                drop(rlock);
                INFO_BLOCK_CACHE_MANAGER
                    .write()
                    .get_block_cache(phy_blk_id, block_device);
                INFO_BLOCK_CACHE_MANAGER
                    .read()
                    .read_block_cache(phy_blk_id)
                    .unwrap()
            }
        }
    } else {
        // println!("   mode:WRITE");
        INFO_BLOCK_CACHE_MANAGER
            .write()
            .get_block_cache(phy_blk_id, block_device)
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

pub fn write_to_dev() {
    INFO_BLOCK_CACHE_MANAGER.write().drop_all();
    DATA_BLOCK_CACHE_MANAGER.write().drop_all();
}

pub fn sync_all() {
    INFO_BLOCK_CACHE_MANAGER.write().sync_all();
    DATA_BLOCK_CACHE_MANAGER.write().sync_all();
}
