use alloc::{vec::Vec, sync::Arc};
use hashbrown::HashMap;
use spin::RwLock;

use crate::{FAT, BlockDevice};
const END_CLUSTER: u32 = 0x0FFFFFF8;

pub struct Chain {
    pub chain: Vec<u32>,                    // index -> cluster
    pub chain_map: HashMap<u32, usize>,    // cluster -> index
}

impl Chain {
    pub fn new() -> Self {
        Self {
            chain: Vec::new(),
            chain_map: HashMap::new(),
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

    // 获取start_cluster所在簇链中，从start_cluster开始的第index个簇
    pub fn get_cluster_at(
        &mut self,
        start_cluster: u32,
        index: usize,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        if let Some(_) = self.chain_map.get(&start_cluster) {
            self.chain.get(index).copied().unwrap()
        } else {
            // fat.read().get_cluster_at(start_cluster, index, block_device)
            self.generate_chain(start_cluster, block_device, fat);
            self.chain[index]
        }
    }

    // 获取start_cluster所在簇链的结束簇的簇号
    pub fn get_final_cluster(
        &mut self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        if let Some(_) = self.chain_map.get(&start_cluster) {
            self.chain[self.chain.len()-1]
        } else {
            // fat.read().get_final_cluster(start_cluster, block_device)
            self.generate_chain(start_cluster, block_device, fat);
            self.chain[self.chain.len()-1]
        }
    }

    // 获取start_cluster为首的簇链上的所有簇号
    pub fn get_all_clusters(
        &mut self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> Vec<u32> {
        if let Some(_) = self.chain_map.get(&start_cluster) {
            self.chain.clone()
        } else {
            // fat.read().get_all_clusters(start_cluster, block_device)
            self.generate_chain(start_cluster, block_device, fat);
            self.chain.clone()
        }
    }

    // 获取start_cluster所在簇链上簇的个数
    pub fn cluster_count(
        &mut self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>,
    ) -> u32 {
        if start_cluster == 0 || start_cluster == 1 {
            return 0;
        }
        if let Some(_) = self.chain_map.get(&start_cluster) {
            self.chain.len() as u32
        } else {
            // fat.read().cluster_count(start_cluster, block_device)
            self.generate_chain(start_cluster, block_device, fat);
            self.chain.len() as u32
        }
    }

    pub fn clear_all(&mut self) {
        self.chain.clear();
        self.chain_map.clear();
    }

    fn generate_chain(
        &mut self,
        start_cluster: u32,
        block_device: &Arc<dyn BlockDevice>,
        fat: &Arc<RwLock<FAT>>
    ) {
        self.chain = fat.read().get_all_clusters(start_cluster, block_device);
        self.generate_map();
    }

    fn generate_map(&mut self) {
        (0..self.chain.len())
            .for_each(|i| { 
                self.chain_map.insert(self.chain[i], i); 
            });
    }
}