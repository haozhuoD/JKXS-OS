use alloc::{vec::Vec, collections::BTreeMap, sync::Arc};
use spin::RwLock;

use crate::{FAT, BlockDevice, println};
const END_CLUSTER: u32 = 0x0FFFFFF8;

pub struct Chain {
    pub chain: Vec<u32>,                    // index -> cluster
    pub chain_map: BTreeMap<u32, usize>,    // cluster -> index
}

impl Chain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            chain_map: BTreeMap::new(),
        }
    }

    // 查询当前簇的下一个簇,如果空闲或坏簇则返回0
    pub fn get_next_cluster(
        &self,
        cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        if let Some(&index) =  self.chain_map.get(&cluster) {
            self.chain.get(index+1).copied().unwrap_or(END_CLUSTER)
        } else {
            fat.read().get_next_cluster(cluster, block_device)
        }
    }

    // // 设置簇curr_cluster的下一个簇为next_cluster
    // pub fn set_next_cluster(
    //     &mut self,
    //     curr_cluster: u32,
    //     next_cluster: u32,
    //     block_device: &Arc<dyn BlockDevice>,
    //     fat: &Arc<RwLock<FAT>>,
    // ) {
    //     // right ?
    //     if next_cluster < END_CLUSTER {
    //         let index = self.chain.len();
    //         self.chain.push(next_cluster);
    //         self.chain_map.insert(next_cluster, index);
    //     }
    //     fat.write().set_next_cluster(curr_cluster, next_cluster, block_device);
    // }

    // 获取start_cluster所在簇链中，从start_cluster开始的第index个簇
    pub fn get_cluster_at(
        &self,
        start_cluster: u32,
        index: usize,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        if let Some(&start_idx) = self.chain_map.get(&start_cluster) {
            self.chain.get(start_idx+index).copied().unwrap()
        } else {
            fat.read().get_cluster_at(start_cluster, index, block_device)
        }
    }

    // 获取start_cluster所在簇链的结束簇的簇号
    pub fn get_final_cluster(
        &self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        let len = self.chain.len();
        self.chain.get(len-1).copied().unwrap_or_else(|| {
            fat.read().get_final_cluster(start_cluster, block_device)
        })
    }

    // 获取start_cluster为首的簇链上的所有簇号
    pub fn get_all_clusters(
        &self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> Vec<u32> {
        if self.chain.len() > 0 {
            self.chain.clone()
        } else {
            fat.read().get_all_clusters(start_cluster, block_device)
        }
    }

    // 获取start_cluster所在簇链上簇的个数
    pub fn cluster_count(
        &self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        let len = self.chain.len();
        if len == 0 {
            return fat.read().cluster_count(start_cluster, block_device);
        }
        len as u32
    }

    pub fn clear_all(&mut self) {
        self.chain.clear();
        self.chain_map.clear();
    }

}