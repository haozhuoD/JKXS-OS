use super::{chain::*, fat32_manager::*, get_info_block_cache, layout::*, BlockDevice, CacheMode};
use crate::println;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::{RwLock, RwLockWriteGuard};

const PAGE_SIZE: usize = 4096;
pub const PAGE_MASK: usize = !0xfff;

fn aligned_down(addr: usize) -> usize {
    addr & PAGE_MASK
}

#[derive(Clone)]
pub struct VFile {
    name: String,
    pub short_sector: usize,
    pub short_offset: usize,
    pub long_pos_vec: Vec<(usize, usize)>, // 长文件名目录项的(扇区, 偏移)
    attribute: u8,
    chain: Arc<RwLock<Chain>>,
    cache: Arc<RwLock<VFileCache>>,
    fs: Arc<FAT32Manager>,
    block_device: Arc<dyn BlockDevice>,
}

impl VFile {
    pub fn new(
        name: String,
        short_sector: usize,
        short_offset: usize,
        long_pos_vec: Vec<(usize, usize)>,
        attribute: u8,
        fs: Arc<FAT32Manager>,
        block_device: Arc<dyn BlockDevice>,
    ) -> Self {
        Self {
            name,
            short_sector,
            short_offset,
            long_pos_vec,
            attribute,
            chain: Arc::new(RwLock::new(Chain::new())),
            cache: Arc::new(RwLock::new(VFileCache::new())),
            fs,
            block_device,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_attribute(&self) -> u8 {
        self.attribute
    }

    pub fn get_size(&self) -> u32 {
        if !self.is_dir() {
            return self.read_short_dirent(|short_ent| short_ent.get_size());
        }
        0
    }

    pub fn set_size(&self, size: u32) {
        if !self.is_dir() {
            self.modify_short_dirent(|short_ent| {
                short_ent.set_size(size);
            });
        }
    }

    pub fn get_fs(&self) -> Arc<FAT32Manager> {
        self.fs.clone()
    }

    pub fn is_dir(&self) -> bool {
        self.attribute == ATTRIBUTE_DIRECTORY
    }

    pub fn is_short(&self) -> bool {
        self.long_pos_vec.len() == 0
    }

    pub fn first_cluster(&self) -> u32 {
        self.read_short_dirent(|short_ent| short_ent.first_cluster())
    }

    pub fn set_first_cluster(&self, cluster: u32) {
        self.modify_short_dirent(|short_ent| {
            short_ent.set_first_cluster(cluster);
        });
    }

    pub fn read_short_dirent<V>(&self, f: impl FnOnce(&ShortDirEntry) -> V) -> V {
        if self.short_sector == 0 {
            let root_dirent = self.fs.get_root_dirent();
            let root_dirent_reader = root_dirent.read();
            f(&root_dirent_reader)
        } else {
            get_info_block_cache(
                self.short_sector,
                self.block_device.clone(),
                CacheMode::READ,
            )
            .read()
            .read(self.short_offset, f)
        }
    }

    pub fn modify_short_dirent<V>(&self, f: impl FnOnce(&mut ShortDirEntry) -> V) -> V {
        if self.short_sector == 0 {
            let root_dirent = self.fs.get_root_dirent();
            let mut root_dirent_writer = root_dirent.write();
            f(&mut root_dirent_writer)
        } else {
            get_info_block_cache(
                self.short_sector,
                self.block_device.clone(),
                CacheMode::READ,
            )
            .write()
            .modify(self.short_offset, f)
        }
    }

    pub fn modify_long_dirent<V>(&self, index: usize, f: impl FnOnce(&mut LongDirEntry) -> V) -> V {
        let (sector, offset) = self.long_pos_vec[index];
        get_info_block_cache(sector, self.block_device.clone(), CacheMode::READ)
            .write()
            .modify(offset, f)
    }

    // 获取短文件名目录项所在的扇区和偏移
    pub fn get_pos(&self, offset: usize) -> (usize, usize) {
        let (_, sec, off) = self.read_short_dirent(|s_ent: &ShortDirEntry| {
            // s_ent.get_pos(offset, &self.fs, &self.fs.get_fat(), &self.block_device)
            s_ent.get_pos(
                offset,
                &self.fs,
                &self.fs.get_fat(),
                &self.block_device,
                &self.chain,
            )
        });
        (sec, off)
    }

    // 通过长文件名name查找目录dir_ent下的目录项
    fn find_long_name(&self, name: &str, dir_ent: &ShortDirEntry) -> Option<VFile> {
        // LongDirEntry通过get_name_format()获取到的文件名是不含'\0'的，因此split时fill0为false
        let name_vec = self.fs.long_name_split(name, false);
        let long_ent_count = name_vec.len();
        let mut offset: usize = 0;
        let mut long_ent = LongDirEntry::empty();
        let mut long_pos_vec: Vec<(usize, usize)> = Vec::new();
        loop {
            let mut read_sz = dir_ent.read_at(
                offset,
                long_ent.as_bytes_mut(),
                &self.fs,
                &self.fs.get_fat(),
                &self.block_device,
                &self.chain,
            );
            if read_sz != DIRENT_SZ || long_ent.is_empty() {
                return None;
            }
            if long_ent.get_name_format() == name_vec[long_ent_count - 1]
                && long_ent.attribute() == ATTRIBUTE_LFN
            {
                let mut order = long_ent.order();
                let checksum = long_ent.checksum();
                // 不是最后一个目录项或删除
                if order & 0x40 == 0 || order == 0xE5 {
                    offset += DIRENT_SZ;
                    continue;
                }
                order = order ^ 0x40;
                // 目录项个数与给定个数不等
                if order as usize != long_ent_count {
                    offset += DIRENT_SZ;
                    continue;
                }
                // 对剩余目录项进行匹配
                let mut is_match = true;
                for i in 1..order as usize {
                    read_sz = dir_ent.read_at(
                        offset + i * DIRENT_SZ,
                        long_ent.as_bytes_mut(),
                        &self.fs,
                        &self.fs.get_fat(),
                        &self.block_device,
                        &self.chain,
                    );
                    if read_sz != DIRENT_SZ {
                        return None;
                    }
                    if long_ent.get_name_format() != name_vec[long_ent_count - 1 - i]
                        || long_ent.attribute() != ATTRIBUTE_LFN
                    {
                        is_match = false;
                        break;
                    }
                }
                // 均匹配成功则读取短文件名目录项进行校验
                if is_match {
                    let mut short_ent = ShortDirEntry::empty();
                    let short_ent_offset = offset + long_ent_count * DIRENT_SZ;
                    read_sz = dir_ent.read_at(
                        short_ent_offset,
                        short_ent.as_bytes_mut(),
                        &self.fs,
                        &self.fs.get_fat(),
                        &self.block_device,
                        &self.chain,
                    );
                    if read_sz != DIRENT_SZ {
                        return None;
                    }
                    if short_ent.is_valid() && short_ent.checksum() == checksum {
                        let (short_sector, short_offset) = self.get_pos(short_ent_offset);
                        // 将长文件名目录项的位置存放进long_pos_vec，注意同样是按文件名倒序存入
                        for i in 0..order as usize {
                            let pos = self.get_pos(offset + i * DIRENT_SZ);
                            long_pos_vec.push(pos);
                        }
                        return Some(VFile::new(
                            String::from(name),
                            short_sector,
                            short_offset,
                            long_pos_vec,
                            short_ent.attribute(),
                            self.fs.clone(),
                            self.block_device.clone(),
                        ));
                    } else {
                        return None;
                    }
                } else {
                    offset += DIRENT_SZ;
                    continue;
                }
            } else {
                offset += DIRENT_SZ;
            }
        }
    }

    // 通过短文件名name(可以是小写)查找目录dir_ent下的目录项
    fn find_short_name(&self, name: &str, dir_ent: &ShortDirEntry) -> Option<VFile> {
        if dir_ent.first_cluster() == 2 && name == ".." {
            return Some(self.get_fs().get_root_vfile(&self.get_fs()));
        }
        let (offset, attribute) = dir_ent.find_short_name(
            0,
            name,
            &self.fs,
            &self.fs.get_fat(),
            &self.block_device,
            &self.chain,
        );
        if offset < 0 {
            return None;
        }
        let (short_sector, short_offset) = self.get_pos(offset as usize);
        let long_pos_vec: Vec<(usize, usize)> = Vec::new();
        Some(VFile::new(
            String::from(name),
            short_sector,
            short_offset,
            long_pos_vec,
            attribute as u8,
            self.fs.clone(),
            self.block_device.clone(),
        ))
    }

    // 通过name查找当前目录下的文件
    pub fn find_vfile_name(&self, name: &str) -> Option<VFile> {
        assert!(self.is_dir());
        let (name_, ext_) = name.rsplit_once(".").unwrap_or((name, ""));
        self.read_short_dirent(|short_ent| {
            if name_.len() > 8 || ext_.len() > 3 || name_.contains(".") {
                return self.find_long_name(name, short_ent);
            } else {
                return self.find_short_name(name, short_ent);
            }
        })
    }

    // 通过相对路径来查找文件
    pub fn find_vfile_path(&self, path: &Vec<&str>) -> Option<Arc<VFile>> {
        let len = path.len();
        let mut curr_vfile = self.clone();
        for i in 0..len {
            if path[i].is_empty() || path[i] == "." {
                continue;
            }
            curr_vfile = curr_vfile.find_vfile_name(path[i])?;
        }
        Some(Arc::new(curr_vfile))
    }

    fn increase_size(&self, new_size: u32) {
        let old_size = self.get_size();
        if new_size <= old_size {
            return;
        }
        let first_cluster = self.first_cluster();
        // let needed = self.fs.cluster_count_needed(old_size, new_size, self.is_dir(), first_cluster);
        let needed = self.fs.cluster_count_needed(
            old_size,
            new_size,
            self.is_dir(),
            first_cluster,
            &self.chain,
        );
        if needed == 0 {
            self.set_size(new_size);
            return;
        }

        if first_cluster == 0 {
            // 未分配簇则将cluster置为第一个簇
            let first_cluster = self
                .fs
                .alloc_cluster(needed, 0, &self.chain)
                .expect("SD Card has no space!");
            self.set_first_cluster(first_cluster);
        } else {
            // 已分配簇则将新分配的簇链连接到原来的簇链上
            // let fat = self.fs.get_fat();
            // let fat_writer = fat.write();
            // let final_cluster = fat_writer.get_final_cluster(first_cluster, &self.block_device);
            // fat_writer.set_next_cluster(final_cluster, cluster, &self.block_device);
            let fat = self.fs.get_fat();
            let final_cluster =
                self.chain
                    .write()
                    .get_final_cluster(first_cluster, &self.block_device, &fat);
            self.fs
                .alloc_cluster(needed, final_cluster, &self.chain)
                .expect("SD Card has no space!");
        }
        self.set_size(new_size);
    }

    // 在当前目录下查找可用目录项，返回offset，簇不够时也会返回相应的offset用于创建新目录项
    fn find_free_dirent(&self) -> usize {
        assert!(self.is_dir());
        self.read_short_dirent(|short_ent| {
            short_ent.find_free_dirent(
                0,
                &self.fs,
                &self.fs.get_fat(),
                &self.block_device,
                &self.chain,
            )
        })
    }

    fn check_cache_modified(&self, cache_writer: &mut RwLockWriteGuard<VFileCache>) {
        if cache_writer.modified {
            self.read_short_dirent(|short_ent| {
                // println!("in");
                let size = short_ent.get_size() as usize;
                let aligned_size = aligned_down(size) + PAGE_SIZE;
                // println!("size = {:#x?}, aligned_size = {:#x?}", size, aligned_size);
                cache_writer.data = Vec::with_capacity(aligned_size);
                // println!("ptr0={:#x?}", cache_writer.data.as_ptr() as usize);
                // println!("in2");
                let data = &mut cache_writer.data;
                unsafe {
                    data.set_len(size);
                }
                let data_start_pa = data.as_ptr() as usize;
                let data_offset = data_start_pa & 0xfff;
                drop(data);
                cache_writer.offset = if data_offset == 0 { 0 } else { PAGE_SIZE - data_offset };
                // assert!(cache_writer.get_data_slice().as_ptr() as usize & 0xfff == 0);
                // assert!(cache_writer.offset < PAGE_SIZE);
                short_ent.read_at(
                    0,
                    &mut cache_writer.data.as_mut_slice(),
                    &self.fs,
                    &self.fs.get_fat(),
                    &self.block_device,
                    &self.chain,
                )
            });
            cache_writer.modified = false;
        }
    }
    pub fn read_at(&self, offset: usize, buf: &mut [u8]) -> usize {
        let mut cache_writer = self.cache.write();
        self.check_cache_modified(&mut cache_writer);
        let data = cache_writer.get_data_slice();
        let end = (offset + buf.len()).min(data.len());
        let r_sz = end - offset;
        buf[..r_sz].copy_from_slice(&data[offset..end]);
        r_sz
    }

    pub fn write_at(&self, offset: usize, buf: &[u8]) -> usize {
        self.increase_size((offset + buf.len()) as u32);
        self.cache.write().modified = true;
        self.modify_short_dirent(|short_ent| {
            short_ent.write_at(
                offset,
                buf,
                &self.fs,
                &self.fs.get_fat(),
                &self.block_device,
                &self.chain,
            )
        })
    }

    /// 将整个文件的数据作为数组切片返回，实现零拷贝
    /// 注意：只能用于读取elf!!
    pub unsafe fn read_as_elf(&self) -> &'static [u8] {
        let mut cache_writer = self.cache.write();
        self.check_cache_modified(&mut cache_writer);
        let data = cache_writer.get_data_slice();
        // println!("******read as elf, {:#x?} -> {:#x?}", data.as_ptr(), data.as_ptr().add(data.len()));
        core::slice::from_raw_parts(data.as_ptr(), data.len())
    }

    pub unsafe fn get_data_cache_physaddr(&self, offset: usize) -> usize {
        let mut cache_writer = self.cache.write();
        self.check_cache_modified(&mut cache_writer);
        let data = cache_writer.get_data_slice();
        data.as_ptr().add(offset) as usize
    }

    pub fn write_at_uncached(&self, offset: usize, buf: &[u8]) -> usize {
        self.increase_size((offset + buf.len()) as u32);
        self.modify_short_dirent(|short_ent| {
            short_ent.write_at(
                offset,
                buf,
                &self.fs,
                &self.fs.get_fat(),
                &self.block_device,
                &self.chain,
            )
        })
    }

    // 在当前目录下创建文件
    pub fn create(&self, name: &str, attribute: u8) -> Option<Arc<VFile>> {
        // println!("creating file {}", name);
        assert!(self.is_dir());
        let mut dirent_offset = self.find_free_dirent();
        let (name_, ext_) = name.rsplit_once(".").unwrap_or((name, ""));
        let mut short_ent = ShortDirEntry::new(name_, ext_, attribute);
        let mut long_pos_vec = Vec::new();
        if name_.len() > 8 || ext_.len() > 3 {
            let checksum = short_ent.checksum();
            let mut name_vec = self.fs.long_name_split(name, true);
            let long_ent_count = name_vec.len();
            let mut long_ent = LongDirEntry::empty();
            // 写长文件名目录项
            for i in 0..long_ent_count {
                let mut order = (long_ent_count - i) as u8;
                if i == 0 {
                    order |= 0x40;
                }
                long_ent.initialize(name_vec.pop().unwrap().as_str(), order, checksum);
                assert_eq!(
                    self.write_at_uncached(dirent_offset, long_ent.as_bytes_mut()),
                    DIRENT_SZ
                );
                long_pos_vec.push(self.get_pos(dirent_offset));
                dirent_offset += DIRENT_SZ;
            }
        }
        // 写短文件名目录项
        assert_eq!(
            self.write_at_uncached(dirent_offset, short_ent.as_bytes_mut()),
            DIRENT_SZ
        );
        let (short_sector, short_offset) = self.get_pos(dirent_offset);
        // 如果是目录类型，需要创建.和..
        // let vfile = self.find_vfile_name(name)?;
        let vfile = VFile::new(
            String::from(name),
            short_sector,
            short_offset,
            long_pos_vec,
            attribute as u8,
            self.fs.clone(),
            self.block_device.clone(),
        );
        if attribute == ATTRIBUTE_DIRECTORY {
            let mut self_dir = ShortDirEntry::new(".", "", ATTRIBUTE_DIRECTORY);
            let mut parent_dir = ShortDirEntry::new("..", "", ATTRIBUTE_DIRECTORY);
            parent_dir.set_first_cluster(self.first_cluster());
            vfile.write_at_uncached(0, self_dir.as_bytes_mut()); // TODO：需要吗
            vfile.write_at_uncached(DIRENT_SZ, parent_dir.as_bytes_mut());
            let first_cluster = vfile.read_short_dirent(|short_ent| short_ent.first_cluster());
            self_dir.set_first_cluster(first_cluster);
            vfile.write_at_uncached(0, &self_dir.as_bytes_mut());
        }
        // println!("file created");
        Some(Arc::new(vfile))
    }

    // 获取当前目录下的所有文件名以及属性
    pub fn ls(&self) -> Option<Vec<(String, u8)>> {
        if !self.is_dir() {
            return None;
        }
        let mut list: Vec<(String, u8)> = Vec::new();
        let mut offset: usize = 0;
        let mut short_ent = ShortDirEntry::empty();
        loop {
            let mut read_sz = self.read_short_dirent(|curr_ent| {
                curr_ent.read_at(
                    offset,
                    short_ent.as_bytes_mut(),
                    &self.fs,
                    &self.fs.get_fat(),
                    &self.block_device,
                    &self.chain,
                )
            });
            if read_sz != DIRENT_SZ || short_ent.is_empty() {
                return Some(list);
            }
            if short_ent.is_deleted() {
                offset += DIRENT_SZ;
                continue;
            }
            if short_ent.is_long() {
                // 将短文件名目录项转化为长文件名目录项
                let mut long_ent: LongDirEntry = unsafe { core::mem::transmute(short_ent) };
                let mut order = long_ent.order();
                if order & 0x40 == 0 {
                    // 实际不会出现这种情况
                    offset += DIRENT_SZ;
                    continue;
                } else {
                    order = order ^ 0x40;
                }
                let mut name = long_ent.get_name_format();
                for _i in 1..order as usize {
                    offset += DIRENT_SZ;
                    read_sz = self.read_short_dirent(|curr_ent| {
                        curr_ent.read_at(
                            offset,
                            long_ent.as_bytes_mut(),
                            &self.fs,
                            &self.fs.get_fat(),
                            &self.block_device,
                            &self.chain,
                        )
                    });
                    if read_sz != DIRENT_SZ || long_ent.is_empty() || long_ent.is_deleted() {
                        return Some(list);
                    }
                    name.insert_str(0, long_ent.get_name_format().as_str());
                }
                // 从短文件名目录项获取类型
                offset += DIRENT_SZ;
                read_sz = self.read_short_dirent(|curr_ent| {
                    curr_ent.read_at(
                        offset,
                        long_ent.as_bytes_mut(),
                        &self.fs,
                        &self.fs.get_fat(),
                        &self.block_device,
                        &self.chain,
                    )
                });
                if read_sz != DIRENT_SZ || long_ent.is_empty() || long_ent.is_deleted() {
                    return Some(list);
                }
                list.push((name, long_ent.attribute()));
            } else {
                list.push((short_ent.get_name_lowercase(), short_ent.attribute()));
            }
            offset += DIRENT_SZ;
        }
    }

    // 获取目录中offset处目录项的信息
    // 返回(name, offset, first_cluster, attribute),其中offset对应的是短文件名目录项的offset
    pub fn dirent_info(&self, offset: usize) -> Option<(String, u32, u32, u8)> {
        if !self.is_dir() {
            return None;
        }
        let mut long_ent = LongDirEntry::empty();
        let mut offset = offset;
        let mut name = String::new();
        let mut is_long = false;
        loop {
            let read_sz = self.read_short_dirent(|curr_ent| {
                curr_ent.read_at(
                    offset,
                    long_ent.as_bytes_mut(),
                    &self.fs,
                    &self.fs.get_fat(),
                    &self.block_device,
                    &self.chain,
                )
            });
            if read_sz != DIRENT_SZ || long_ent.is_empty() {
                return None;
            }
            if long_ent.is_deleted() {
                offset += DIRENT_SZ;
                name.clear();
                is_long = false;
                continue;
            }
            if long_ent.attribute() != ATTRIBUTE_LFN {
                let short_ent: ShortDirEntry = unsafe { core::mem::transmute(long_ent) };
                if !is_long {
                    name = short_ent.get_name_lowercase();
                }
                let attribute = short_ent.attribute();
                let first_cluster = short_ent.first_cluster();
                return Some((name, offset as u32, first_cluster, attribute));
            } else {
                is_long = true;
                name.insert_str(0, long_ent.get_name_format().as_str());
            }
            offset += DIRENT_SZ;
        }
    }

    // 获取目录中offset处目录项的信息
    // 返回(size, accessed_time, modification_time, creation_time, first_cluster)
    pub fn stat(&self) -> (i64, i64, i64, i64, u64) {
        self.read_short_dirent(|short_ent| {
            let (_, _, _, _, _, _, ctime) = short_ent.get_creation_time();
            let atime = self.accessed_time();
            let mtime = self.modification_time();
            let mut size = short_ent.get_size();
            let first_cluster = short_ent.first_cluster();
            if self.is_dir() {
                let fat = self.fs.get_fat();
                let cluster_count = fat.read().cluster_count(first_cluster, &self.block_device);
                size = cluster_count * self.fs.bytes_per_cluster();
            }
            (
                size as i64,
                atime as i64,
                mtime as i64,
                ctime as i64,
                first_cluster as u64,
            )
        })
    }

    // 清除文件内容，仅改变文件size和FAT表项内容，不实际清除数据区
    pub fn clear(&self) {
        let first_cluster = self.first_cluster();
        if self.is_dir() || first_cluster == 0 {
            return;
        }
        self.modify_short_dirent(|short_ent| {
            short_ent.clear();
        });
        // let all_clusters = self.fs.get_fat().read().get_all_clusters(first_cluster, &self.block_device);
        let all_clusters = self.chain.write().get_all_clusters(
            first_cluster,
            &self.block_device,
            &self.fs.get_fat(),
        );
        self.fs.dealloc_cluster(all_clusters, &self.chain);
    }

    pub fn creation_time(&self) -> (u32, u32, u32, u32, u32, u32, u64) {
        self.read_short_dirent(|short_ent| short_ent.get_creation_time())
    }

    pub fn accessed_time(&self) -> u64 {
        let (date, time) = self.read_short_dirent(|short_ent| short_ent.get_accessed_time());
        fat2unix_time64(date, time)
    }

    pub fn set_accessed_time(&self, atime: u64) {
        let (date, _) = unix2fat_time64(atime);
        self.modify_short_dirent(|short_ent| short_ent.set_accessed_time(date));
    }

    pub fn modification_time(&self) -> u64 {
        let (date, time) = self.read_short_dirent(|short_ent| short_ent.get_modification_time());
        fat2unix_time64(date, time)
    }

    pub fn set_modification_time(&self, mtime: u64) {
        let (date, time) = unix2fat_time64(mtime);
        println!("mtime:{}, date:{}, time:{}", mtime, date, time);
        self.modify_short_dirent(|short_ent| short_ent.set_modification_time(date, time));
    }

    // 删除目录项，不清理fat表
    pub fn delete(&self) {
        (0..self.long_pos_vec.len()).for_each(|i| {
            self.modify_long_dirent(i, |long_ent| {
                long_ent.delete();
            });
        });
        self.modify_short_dirent(|short_ent| {
            short_ent.delete();
        });
    }

    // TODO：只支持删除自己
    pub fn remove(&self) -> usize {
        let first_cluster = self.first_cluster();
        (0..self.long_pos_vec.len()).for_each(|i| {
            self.modify_long_dirent(i, |long_ent| {
                long_ent.delete();
            });
        });
        self.modify_short_dirent(|short_ent| {
            short_ent.delete();
        });
        // 无文件内容时
        if first_cluster == 0 {
            return 0;
        }
        if self.is_dir() {
            let mut offset = 2 * DIRENT_SZ;
            let mut short_ent = ShortDirEntry::empty();
            loop {
                let read_sz = self.read_short_dirent(|curr_ent| {
                    curr_ent.read_at(
                        offset,
                        short_ent.as_bytes_mut(),
                        &self.fs,
                        &self.fs.get_fat(),
                        &self.block_device,
                        &self.chain,
                    )
                });
                if read_sz != DIRENT_SZ || short_ent.is_empty() {
                    break;
                }
                if short_ent.is_deleted() {
                    offset += DIRENT_SZ;
                    continue;
                }
                if short_ent.is_long() {
                    let mut long_ent: LongDirEntry = unsafe { core::mem::transmute(short_ent) };
                    long_ent.delete();
                    offset += DIRENT_SZ;
                    continue;
                }
                let (short_sector, short_offset) = self.get_pos(offset);
                let vfile = VFile::new(
                    String::new(),
                    short_sector,
                    short_offset,
                    Vec::new(),
                    short_ent.attribute(),
                    self.fs.clone(),
                    self.block_device.clone(),
                );
                vfile.remove();
                offset += DIRENT_SZ;
            }
        }
        // let all_clusters = self.fs.get_fat().read().get_all_clusters(first_cluster, &self.block_device);
        let all_clusters = self.chain.write().get_all_clusters(
            first_cluster,
            &self.block_device,
            &self.fs.get_fat(),
        );
        let len = all_clusters.len();
        // self.fs.dealloc_cluster(all_clusters);
        self.fs.dealloc_cluster(all_clusters, &self.chain);
        len
    }
}

#[derive(Clone)]
struct VFileCache {
    data: Vec<u8>,
    // 我们希望cache是按页对齐的，data的起始地址加上offset才是cache的真正开始位置
    offset: usize,
    modified: bool,
}

impl VFileCache {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            offset: 0,
            modified: true,
        }
    }

    pub fn get_data_slice(&self) -> &[u8] {
        &self.data.as_slice()[self.offset..]
    }
}
