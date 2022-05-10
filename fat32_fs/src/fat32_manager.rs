use alloc::string::ToString;
use alloc::{sync::Arc, string::String};
use alloc::vec::Vec;
use super::{
    BlockDevice,
    set_start_sector,
    get_info_block_cache,
    get_data_block_cache,
    FAT,
    CacheMode,
    VFile,
};
use crate::println;
use crate::{layout::*, fat::FREE_CLUSTER};
use spin::RwLock;

#[derive(Debug)]
pub struct FAT32ManagerInner {
    pub bytes_per_sector:       u32,
    pub bytes_per_cluster:      u32,
    pub sectors_per_cluster:    u32,
    pub root_sector:            u32,
}

pub struct FAT32Manager{
    pub inner:          FAT32ManagerInner,
    block_device:   Arc<dyn BlockDevice>,
    fsinfo:         Arc<RwLock<FSInfo>>,
    fat:            Arc<RwLock<FAT>>,
    root_dirent:    Arc<RwLock<ShortDirEntry>>,
}

impl FAT32Manager {
    pub fn bytes_per_sector(&self) -> u32 {
        self.inner.bytes_per_sector
    }

    pub fn bytes_per_cluster(&self) -> u32 {
        self.inner.bytes_per_cluster
    }

    pub fn sectors_per_cluster(&self) -> u32 {
        self.inner.sectors_per_cluster
    }

    // 第一个数据扇区(根目录的第一个扇区)
    pub fn first_data_sector(&self) -> u32 {
        self.inner.root_sector
    }

    // 给定簇号的第一个扇区
    pub fn first_sector_of_cluster(&self, cluster: u32) -> usize {
        ((cluster - 2) * self.inner.sectors_per_cluster + self.inner.root_sector) as usize
    }

    pub fn open(block_device: Arc<dyn BlockDevice>) -> Arc<Self> {
        // 读入并设置起始扇区
        println!("setting start sector...");
        let start_sector: u32 = get_info_block_cache(
            0,
            Arc::clone(&block_device),
            CacheMode::READ)
            .read()
            .read(0x1c6, |ssec_bytes: &[u8; 4]| {
                let mut start_sector: u32 = 0;
                for i in 0..4 {
                    let tmp = ssec_bytes[i] as u32;
                    start_sector = start_sector + (tmp << (8*i));
                }
                start_sector
            });
        set_start_sector(start_sector as usize);

        // 读入BPB
        println!("reading BPB...");
        let bpb = get_info_block_cache(
            0, 
            Arc::clone(&block_device), 
            CacheMode::READ)
            .read()
            .read(0, |&bpb: &FatBS| bpb);
        
        // 读入EBR
        println!("reading EBR...");
        let ebr = get_info_block_cache(
            0, 
            Arc::clone(&block_device), 
            CacheMode::READ)
            .read()
            .read(36, |&ebr: &FatExtBS| ebr);
        
        // 读入FSInfo
        println!("reading FSInfo...");
        let fsinfo_inner = get_info_block_cache(
            ebr.fsinfo_sector() as usize, 
            Arc::clone(&block_device), 
            CacheMode::READ)
            .read()
            .read(0, |&fsinfo_inner: &FSInfoInner| fsinfo_inner);
        assert!(fsinfo_inner.is_valid(), "Error loading fat32! Illegal signature");
        let fsinfo_sector = ebr.fsinfo_sector() + start_sector;  // 需要加上起始扇区偏移
        let fsinfo = FSInfo::new(fsinfo_sector, fsinfo_inner);

        // FAT表的内存结构
        let first_fat1_sector = bpb.first_fat_sector();
        let fat_size = ebr.fat_size();
        let first_fat2_sector = first_fat1_sector + fat_size;
        let fat = FAT::new(first_fat1_sector, first_fat2_sector, fat_size);
        println!("first_fat1_sector: {}", first_fat1_sector);
        println!("first_fat2_sector: {}", first_fat2_sector);

        // Inner内容
        let sectors_per_cluster = bpb.sectors_per_cluster as u32;
        let bytes_per_sector = bpb.bytes_per_sector as u32;
        let bytes_per_cluster = sectors_per_cluster * bytes_per_sector;
        let root_sector = first_fat1_sector +  bpb.table_count as u32 * fat_size;
        println!("root_sector: {}", root_sector);
        
        // 根目录
        let mut root_dirent = ShortDirEntry::new(
            "/",
            "",
            ATTRIBUTE_DIRECTORY
        );
        root_dirent.set_first_cluster(2);

        let fat32_manager = Self {
            inner: FAT32ManagerInner { 
                bytes_per_sector, 
                bytes_per_cluster, 
                sectors_per_cluster, 
                root_sector,
            },
            block_device,
            fsinfo: Arc::new(RwLock::new(fsinfo)),
            fat: Arc::new(RwLock::new(fat)),
            root_dirent: Arc::new(RwLock::new(root_dirent)),
        };
        Arc::new(fat32_manager)
    }

    pub fn get_root_vfile(&self, fs_manager: &Arc<Self>) -> VFile{
        let long_pos_vec: Vec<(usize, usize)> = Vec::new();
        VFile::new(
            String::from("/"), 
            0, 
            0, 
            long_pos_vec, 
            ATTRIBUTE_DIRECTORY, 
            Arc::clone(fs_manager), 
            self.block_device.clone()
        )
    }

    pub fn get_root_dirent(&self) -> Arc<RwLock<ShortDirEntry>> {
        self.root_dirent.clone()
    }

    pub fn get_fat(&self) -> Arc<RwLock<FAT>> {
        self.fat.clone()
    }

    pub fn get_fsinfo(&self) -> Arc<RwLock<FSInfo>> {
        self.fsinfo.clone()
    }

    // 对应字节大小所需要的簇数
    pub fn size_to_cluster(&self, size: u32) -> u32 {
        (size + self.bytes_per_cluster() - 1) / self.bytes_per_cluster()
    }

    // 偏移量所在簇号
    pub fn cluster_of_offset(&self, offset: usize) -> u32 {
        offset as u32 / self.bytes_per_cluster()
    }

    pub fn free_cluster_count(&self) -> u32 {
        self.fsinfo.read().read_free_cluster_count()
    }

    // 清除簇中的所有扇区
    pub fn clear_cluster(&self, cluster: u32) {
        let start_sector = self.first_sector_of_cluster(cluster);
        for i in 0..self.sectors_per_cluster() {
            get_data_block_cache(
                start_sector + i as usize, 
                self.block_device.clone(), 
                CacheMode::WRITE
            )
            .write()
            .modify(0, |data_block: &mut [u8; 512]| {
                for byte in data_block.iter_mut() { *byte = 0; }
            });
        }
    }

    // 在FAT表上分配num个簇，成功时返回第一个簇，失败时返回None
    pub fn alloc_cluster(&self, num: u32) -> Option<u32> {
        let free_clusters = self.free_cluster_count();
        if num > free_clusters {
            return None;
        }
        let fat_writer = self.fat.write();
        let last_alloc_cluster = self.fsinfo.read().read_last_alloc_cluster();
        let first_free_cluster = fat_writer.next_free_cluster(last_alloc_cluster, &self.block_device);
        let mut curr_cluster = first_free_cluster;
        for _ in 1..num {
            self.clear_cluster(curr_cluster);
            let next_clutser = fat_writer.next_free_cluster(curr_cluster, &self.block_device);
            assert_ne!(next_clutser, 0, "No free cluster!");
            fat_writer.set_next_cluster(curr_cluster, next_clutser, &self.block_device);
            curr_cluster = next_clutser;
        }
        // 填写最后一个FAT表项
        fat_writer.set_end_cluster(curr_cluster, &self.block_device);
        // 修改FSInfo块
        let mut fsinfo_writer = self.fsinfo.write();
        fsinfo_writer.write_free_cluster_count(free_clusters - num);
        fsinfo_writer.write_last_alloc_cluster(curr_cluster);

        Some(first_free_cluster)
    }

    // 簇的去分配，仅修改FAT表，不更改数据区
    pub fn dealloc_cluster(&self, clusters: Vec<u32>) {
        let free_clusters = self.free_cluster_count();
        let num = clusters.len();
        if num > 0 {
            // 修改FAT表项
            let fat_writer = self.fat.write();
            (0..num).for_each(|i| 
                fat_writer.set_next_cluster(clusters[i], FREE_CLUSTER, &self.block_device)
            );
            self.fsinfo.write().write_free_cluster_count(free_clusters + num as u32);
            // 如果释放的簇号小于fsinfo中开始搜索空闲簇的字段，则更新该字段
            if clusters[0] > 2 && clusters[0] < self.fsinfo.read().read_last_alloc_cluster() {
                self.fsinfo.write().write_last_alloc_cluster(clusters[0] - 1);
            }
        }
    }

    // 扩大至new_sz需要多少个簇
    pub fn cluster_count_needed(&self, old_sz: u32, new_sz: u32, is_dir: bool, first_cluster: u32) -> u32 {
        if old_sz >= new_sz {
            0
        } else {
            if is_dir {
                let old_clusters= self.fat.read().cluster_count(first_cluster, &self.block_device);
                self.size_to_cluster(new_sz) - old_clusters
            } else {
                self.size_to_cluster(new_sz) - self.size_to_cluster(old_sz)
            }
        }
    }

    // 拆分长文件名，根据fill0决定是否补充'\0'，不补充0xFF
    pub fn long_name_split(&self, name: &str, fill0: bool) -> Vec<String> {
        let mut vec_name: Vec<String> = name.as_bytes()
            .chunks(LONG_NAME_LEN)
            .map(|x| core::str::from_utf8(x).unwrap().to_string())
            .collect();
        // 填充'\0'
        if fill0 {
            let last = vec_name.len();
            if last != 0 {
                if vec_name[last - 1].len() < LONG_NAME_LEN {
                    vec_name[last - 1].push('\0');
                }
            }
        }
        vec_name
    }

    // 拆分文件名和扩展名
    pub fn split_name_ext<'a>(&self, name: &'a str) -> (&'a str, &'a str) {
        let mut name_ext: Vec<&str> = name.split(".").collect();
        let name_ = name_ext[0];
        if name_ext.len() == 1 {
            name_ext.push("");
        }
        let ext_ = name_ext[1];
        (name_, ext_)
    }

    // 将fsinfo写回磁盘
    pub fn sync_fsinfo(&mut self) {
        self.block_device.write_block(
            self.fsinfo.read().fsinfo_sector() as usize, 
            self.fsinfo.read().inner_as_bytes()
        );
    }
}

impl Drop for FAT32Manager {
    fn drop(&mut self) {
        self.sync_fsinfo();
    }
}