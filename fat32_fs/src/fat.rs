use alloc::{sync::Arc, vec::Vec};
use spin::RwLock;
use super::{
    BLOCK_SZ,
    BlockDevice,
    CacheMode,
    get_info_block_cache,
    chain::*,
};

const FAT_ENTRY_PER_SEC: u32 = BLOCK_SZ as u32 / 4;
type FatentBlock = [u32; FAT_ENTRY_PER_SEC as usize];
pub const FREE_CLUSTER: u32 = 0x00000000;
pub const END_CLUSTER: u32 = 0x0FFFFFF8;
pub const BAD_CLUSTER: u32 = 0x0FFFFFF7;

#[derive(Clone, Copy)]
pub struct FAT {
    first_fat1_sector:  u32,
    first_fat2_sector:  u32,
    _fat_size:           u32,  // 扇区数
    max_cluster:        u32,  // FAT表所允许的最大簇数
}

impl FAT {
    pub fn new(first_fat1_sector: u32, first_fat2_sector: u32, fat_size: u32) -> Self{
        Self {
            first_fat1_sector,
            first_fat2_sector,
            _fat_size: fat_size,
            max_cluster: fat_size * FAT_ENTRY_PER_SEC,
        }
    }

    // 计算给定簇号对应FAT表项的位置
    fn cluster_position(&self, cluster: u32) -> (u32, u32, u32) {
        let fat1_sec = self.first_fat1_sector + cluster / FAT_ENTRY_PER_SEC;
        // let fat2_sec = self.first_fat2_sector + cluster / FAT_ENTRY_PER_SEC;
        let offset = 4 * (cluster % FAT_ENTRY_PER_SEC);
        // (fat1_sec, fat2_sec, offset)
        (fat1_sec, 0, offset)
    }

    // 计算从curr_cluster开始的下一个可用簇号,如果FAT表中没有空闲表项了，则返回0
    pub fn next_free_cluster(&self, curr_cluster: u32, block_device: &Arc<dyn BlockDevice>) -> u32 {
        // let mut curr_cluster = curr_cluster + 1;
        // while curr_cluster <= self.max_cluster {
        //     let (fat1_sec, _, offset) = self.cluster_position(curr_cluster);
        //     // 查看当前cluster的表项
        //     let entry_val = get_info_block_cache(
        //         fat1_sec as usize, 
        //         Arc::clone(block_device), 
        //         CacheMode::READ)
        //     .read()
        //     .read(offset as usize,|&entry_val: &u32|{
        //         entry_val
        //     });
        //     if entry_val == FREE_CLUSTER { 
        //         return curr_cluster & 0x0FFFFFFF;
        //     }else{
        //         curr_cluster += 1;
        //     }
        // }
        // 0
        let mut curr_cluster = curr_cluster + 1;
        let mut ret_cluster = curr_cluster / FAT_ENTRY_PER_SEC * FAT_ENTRY_PER_SEC;
        let mut fat1_sec = (self.first_fat1_sector + curr_cluster / FAT_ENTRY_PER_SEC) as usize;
        let end_sector = self.first_fat2_sector as usize;
        while fat1_sec <= end_sector {
            let cluster = get_info_block_cache(
                fat1_sec,
                Arc::clone(block_device),
                CacheMode::READ,
            ).read()
            .read(0, |fat_blk: &FatentBlock| {
                (curr_cluster % FAT_ENTRY_PER_SEC..FAT_ENTRY_PER_SEC)
                    .find(|&i| fat_blk[i as usize] == FREE_CLUSTER)
            });
            if cluster.is_some() {
                return (ret_cluster + cluster.unwrap()) & 0x0FFFFFFF;
            }
            curr_cluster = 0;
            fat1_sec += 1;
            ret_cluster += FAT_ENTRY_PER_SEC;
        }
        0
    }

    // 查询当前簇的下一个簇,如果空闲或坏簇则返回0
    pub fn get_next_cluster(&self, cluster: u32, block_device: &Arc<dyn BlockDevice>) -> u32 {
        assert!(cluster < self.max_cluster, "The current cluster number {} exceeds the maximum cluster number {}!"
                , cluster, self.max_cluster);
        let (fat1_sec, fat2_sec, offset) = self.cluster_position(cluster);
        let fat1_next_cluster = get_info_block_cache(
            fat1_sec as usize, 
            Arc::clone(block_device), 
            CacheMode::READ)
            .read()
            .read(offset as usize, |&next_cluster: &u32| {
                next_cluster
            });
        if fat1_next_cluster == BAD_CLUSTER {
            let fat2_next_cluster = get_info_block_cache(
                fat2_sec as usize, 
                Arc::clone(block_device),
                CacheMode::READ)
                .read()
                .read(offset as usize, |&next_cluster: &u32| {
                    next_cluster
                });
            if fat2_next_cluster == BAD_CLUSTER {
                0
            } else {
                fat2_next_cluster & 0x0FFFFFFF
            }
        } else {
            fat1_next_cluster & 0x0FFFFFFF
        }
    }

    // 设置簇curr_cluster的下一个簇为next_cluster
    pub fn set_next_cluster(
        &self,
        curr_cluster: u32,
        next_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        chain: &Arc<RwLock<Chain>>,
    ) {
        assert!(curr_cluster < self.max_cluster && curr_cluster > 1, "The current cluster number is out of range!");
        let (fat1_sec, _fat2_sec, offset) = self.cluster_position(curr_cluster);
        get_info_block_cache(
            fat1_sec as usize, 
            Arc::clone(block_device), 
            CacheMode::READ)
            .write()
            .modify(offset as usize, |val: &mut u32| {
                *val = next_cluster;
        });
        let mut chain_writer = chain.write();
        let mut index = chain_writer.chain.len();
        if index == 0 {
            chain_writer.chain.push(curr_cluster);
            chain_writer.chain_map.insert(curr_cluster, 0);
            index += 1;
        }
        if next_cluster != FREE_CLUSTER && next_cluster < END_CLUSTER {
            chain_writer.chain.push(next_cluster);
            chain_writer.chain_map.insert(next_cluster, index);
        }
        // get_info_block_cache(
        //     fat2_sec as usize, 
        //     Arc::clone(block_device), 
        //     CacheMode::READ)
        //     .write()
        //     .modify(offset as usize, |val: &mut u32| {
        //         *val = next_cluster;
        // });
    }

    // 设置当前簇为结束簇
    pub fn set_end_cluster(
        &self,
        curr_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        chain: &Arc<RwLock<Chain>>,
    ) {
        self.set_next_cluster(curr_cluster, END_CLUSTER, block_device, chain);
    }

    // 获取start_cluster所在簇链中，从start_cluster开始的第index个簇
    pub fn get_cluster_at(&self, start_cluster: u32, index: usize, block_device: &Arc<dyn BlockDevice>) -> u32 {
        let mut cluster = start_cluster;
        for _ in 0..index {
            assert!(cluster < self.max_cluster, "The current cluster number {} exceeds the maximum cluster number {}!"
            , cluster, self.max_cluster);
            cluster = self.get_next_cluster(cluster, block_device);
            if cluster == 0 {
                return 0;
            }
        }
        cluster & 0x0FFFFFFF
    }

    // 获取start_cluster所在簇链的结束簇的簇号
    pub fn get_final_cluster(&self, start_cluster: u32, block_device: &Arc<dyn BlockDevice>) -> u32 {
        let mut curr_cluster = start_cluster;
        loop {
            assert!(curr_cluster < self.max_cluster, "The current cluster number {} exceeds the maximum cluster number {}!"
            , curr_cluster, self.max_cluster);
            let next_cluster = self.get_next_cluster(curr_cluster, block_device);
            if next_cluster >= END_CLUSTER {  // 结束簇
                return curr_cluster & 0x0FFFFFFF;
            } else if next_cluster == 0 {  // 空闲或坏簇
                return 0;
            } else {
                curr_cluster = next_cluster;
            }
        }
    }

    // 获取start_cluster为首的簇链上的所有簇号
    pub fn get_all_clusters(&self, start_cluster: u32, block_device: &Arc<dyn BlockDevice>) -> Vec<u32> {
        // let mut curr_cluster = start_cluster;
        // let mut v_cluster: Vec<u32> = Vec::new();
        // loop {
        //     v_cluster.push(curr_cluster & 0x0FFFFFFF);
        //     assert!(curr_cluster < self.max_cluster, "The current cluster number {} exceeds the maximum cluster number {}!"
        //     , curr_cluster, self.max_cluster);
        //     let next_cluster = self.get_next_cluster(curr_cluster, block_device);
        //     if next_cluster >= END_CLUSTER || next_cluster == 0 {
        //         return v_cluster;
        //     } else {
        //         curr_cluster = next_cluster;
        //     }
        // }

        let mut v_cluster: Vec<u32> = Vec::new();
        v_cluster.push(start_cluster & 0x0FFFFFFF);
        let (mut fat1_sec, _, mut offset) = self.cluster_position(start_cluster);
        let mut tmp_sec = 0;
        let mut flag = false;
        loop {
            v_cluster = get_info_block_cache(
                fat1_sec as usize,
                Arc::clone(block_device),
                CacheMode::READ
            ).read()
            .read(0, |fat_blk: &FatentBlock| {
                loop {
                    let next_cluster = fat_blk[offset as usize / 4];
                    if next_cluster >= END_CLUSTER || next_cluster == 0 {
                        flag = true;
                        return v_cluster;
                    } else {
                        v_cluster.push(next_cluster & 0x0FFFFFFF);
                    }
                    (tmp_sec, _, offset) = self.cluster_position(next_cluster);
                    if fat1_sec != tmp_sec {
                        fat1_sec = tmp_sec;
                        break v_cluster;
                    }
                }
            });
            if flag {
                return v_cluster;
            }
        }
    }

    // 获取start_cluster所在簇链上簇的个数
    pub fn cluster_count(&self, start_cluster: u32, block_device: &Arc<dyn BlockDevice>) -> u32 {
        if start_cluster == 0 || start_cluster == 1 {
            return 0;
        }
        let mut curr_cluster = start_cluster;
        let mut count: u32 = 0;
        loop {
            assert!(curr_cluster < self.max_cluster, "The current cluster number {} exceeds the maximum cluster number {}!"
            , curr_cluster, self.max_cluster);
            count += 1;
            let next_cluster = self.get_next_cluster(curr_cluster, block_device);
            if next_cluster >= END_CLUSTER || next_cluster == 0 {
                return count;
            } else {
                curr_cluster = next_cluster;
            }
        }
    }
}