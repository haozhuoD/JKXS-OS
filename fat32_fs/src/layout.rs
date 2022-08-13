use crate::CacheMode;
use crate::fat::END_CLUSTER;

use super:: {
	BLOCK_SZ,
	BlockDevice,
	get_data_block_cache,
	get_info_block_cache,
	fat32_manager::FAT32Manager,
	fat::*,
	chain::*,
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::RwLock;

// FSInfo
const LEAD_SIGNATURE: 		u32 = 0x41615252;
const ANOTHER_SIGNATURE: 	u32 = 0x61417272;
const TRAIL_SIGNATURE: 		u32 = 0xaa550000;

// ShortDirEntry
pub const SHORT_NAME_LEN: 		usize = 8;
pub const SHORT_EXT_LEN: 		usize = 3;
pub const LONG_NAME_LEN:		usize = 13;
pub const ATTRIBUTE_READ_ONLY:	u8 = 0x01;
pub const ATTRIBUTE_HIDDEN   :	u8 = 0x02;
pub const ATTRIBUTE_SYSTEM   :	u8 = 0x04;
pub const ATTRIBUTE_VOLUME_ID:	u8 = 0x08;
pub const ATTRIBUTE_DIRECTORY:	u8 = 0x10;
pub const ATTRIBUTE_ARCHIVE  :	u8 = 0x20;
pub const ATTRIBUTE_LFN      :	u8 = 0x0F;
pub const DIRENT_SZ:			usize = 32;

type DataBlock = [u8; BLOCK_SZ];
type DirentBlock = [ShortDirEntry; BLOCK_SZ / DIRENT_SZ];

pub const EMPTY_DIRENT:		isize = -1;
pub const NOTFIND_DIRENT:	isize = -2;


#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct FatBS {
	_bootjmp:                [u8; 3],
	_oem_name:               [u8; 8],
	pub bytes_per_sector:       u16,
	pub sectors_per_cluster:    u8,
	reserved_sector_count:  u16,
	pub table_count:            u8,
	_root_entry_count:       u16,
	_total_sectors_16:       u16,
	_media_type:             u8,
	_table_size_16:          u16,
	_sectors_per_track:      u16,
	_head_side_count:        u16,
	_hidden_sector_count:    u32,
	total_sectors_32:       u32,
}

impl FatBS {
    pub fn total_sectors(&self) -> u32 {
        self.total_sectors_32
    }

    pub fn first_fat_sector(&self) -> u32 {
        self.reserved_sector_count as u32
    }
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct FatExtBS {
	table_size_32:      u32,
	_extended_flags:     u16,
	_fat_version:        u16,
	_root_cluster:       u32,
	fat_info:           u16,
    // backup_BS_sector:   u16,
	// reserved_0:         [u8; 12],
	// drive_number:       u8,
	// reserved_1:         u8,
	// boot_signature:     u8,
	// volume_id:          u32,
	// volume_label:       [u8; 11],
	// fat_type_label:     [u8; 8],
}

impl FatExtBS {
    pub fn fat_size(&self) -> u32 {
        self.table_size_32
    }

	pub fn fsinfo_sector(&self) -> u32 {
		self.fat_info as u32
	}
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct FSInfoInner {
	lead_signature: 	u32,
	_reserved_0: 		[u8; 480],
	another_signature: 	u32,
	free_cluster_count: u32,
	last_alloc_cluster: u32,
	_reserved_1: 		[u8; 12],
	trail_signature: 	u32,
}

impl FSInfoInner {
	pub fn is_valid(&self) -> bool {
		self.lead_signature == LEAD_SIGNATURE &&
		self.another_signature == ANOTHER_SIGNATURE &&
		self.trail_signature == TRAIL_SIGNATURE
	}

	pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as usize as *const u8,
                BLOCK_SZ,
            )
        }
    }
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct FSInfo {
	fsinfo_sector: 	u32,
	fsinfo_inner:	FSInfoInner,
}

impl FSInfo {
	pub fn new(fsinfo_sector: u32, fsinfo_inner: FSInfoInner) -> Self {
		Self {
			fsinfo_sector,
			fsinfo_inner,
		}
	}

	pub fn fsinfo_sector(&self) -> u32 {
		self.fsinfo_sector
	}

	pub fn read_free_cluster_count(&self) -> u32 {
		self.fsinfo_inner.free_cluster_count
	}

	pub fn write_free_cluster_count(&mut self, free_cluster_count: u32) {
		self.fsinfo_inner.free_cluster_count = free_cluster_count;
	}

	pub fn read_last_alloc_cluster(&self) -> u32 {
		self.fsinfo_inner.last_alloc_cluster
	}

	pub fn write_last_alloc_cluster(&mut self, last_alloc_cluster: u32) {
		self.fsinfo_inner.last_alloc_cluster = last_alloc_cluster;
	}

	pub fn inner_as_bytes(&self) -> &[u8] {
		self.fsinfo_inner.as_bytes()
	}
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct ShortDirEntry {
	name: 				[u8; SHORT_NAME_LEN],
	extension: 			[u8; SHORT_EXT_LEN],
	attribute: 			u8,
	_reserved_0: 		u8,
	_creation_tenths: 	u8,
	creation_time: 		u16,
	creation_date: 		u16,
	last_accessed_date:	u16,
	first_cluster_high: u16,
	last_mod_time: 		u16,
	last_mod_date: 		u16,
	first_cluster_low: 	u16,
	size_in_bytes: 		u32,
}

impl ShortDirEntry {
	pub fn empty() -> Self {
		Self {
			name: 				[0; SHORT_NAME_LEN],
			extension: 			[0; SHORT_EXT_LEN],
			attribute: 			0,
			_reserved_0: 		0,
			_creation_tenths: 	0,
			creation_time: 		0,
			creation_date: 		0,
			last_accessed_date:	0,
			first_cluster_high: 0,
			last_mod_time: 		0,
			last_mod_date: 		0,
			first_cluster_low: 	0,
			size_in_bytes: 		0,
		}
	}

	// 生成短文件名目录项，会将传入的文件名转化为大写
	pub fn new(name: &str, extension: &str, attribute: u8) -> Self {
		let mut name_bytes = [0x20u8; SHORT_NAME_LEN];
		let name_len = name.len();
		if name_len <= SHORT_NAME_LEN {
			// to_ascii_uppercase()不会对.产生影响
			let _ = &mut name_bytes[..name_len].copy_from_slice(name.to_ascii_uppercase().as_bytes());
		} else {
			// &mut name_bytes[..6].copy_from_slice(name[..6].as_bytes());
			// name_bytes[6] = b'~';
			// name_bytes[7] = b'1';
			let _ = &mut name_bytes.copy_from_slice(format!("{}~1", name[..6].to_ascii_uppercase().to_string()).as_bytes());

		}

		// 扩展名只复制前3字节
		let mut ext_bytes = [0x20u8; SHORT_EXT_LEN];
		let extlen = extension.len().min(3);
		let _ = &mut ext_bytes[0..extlen].copy_from_slice(&extension.to_ascii_uppercase().as_bytes()[0..extlen]);


		Self{
			name: name_bytes,
			extension: ext_bytes,
			attribute,
			_reserved_0: 0,		
			_creation_tenths: 0, 	
			creation_time: 0, 		
			creation_date: 0x529c,		
			last_accessed_date:	0,
			first_cluster_high: 0,
			last_mod_time: 0,		
			last_mod_date: 0,		
			first_cluster_low: 0,	
			size_in_bytes: 0,		
		}
	}

	// 使用的簇的个数
	pub fn data_cluster_count(&self, bytes_per_cluster: u32) -> u32 {
		(self.size_in_bytes + bytes_per_cluster - 1) / bytes_per_cluster
	}

	pub fn is_dir(&self) -> bool {
		self.attribute == ATTRIBUTE_DIRECTORY
	}

	pub fn is_valid(&self) -> bool {
		self.name[0] != 0xE5 && self.name[0] != 0x00
	}

	pub fn is_deleted(&self) -> bool {
		self.name[0] == 0xE5
	}

	pub fn is_empty(&self) -> bool {
		self.name[0] == 0x00
	}

	pub fn is_long(&self) -> bool {
		self.attribute == ATTRIBUTE_LFN
	}

	pub fn is_short(&self) -> bool {
		self.attribute != ATTRIBUTE_LFN
	}

	pub fn attribute(&self) -> u8 {
		self.attribute
	}

	pub fn get_creation_time(&self) -> (u32,u32,u32,u32,u32,u32,u64) {
        // year-month-day-Hour-min-sec-long_sec
        let year: u32  = ((self.creation_date & 0xFE00)>>9) as u32 + 1980;
        let month: u32  = ((self.creation_date & 0x01E0)>>5) as u32 ;    
        let day: u32  = (self.creation_date & 0x001F) as u32 ;    
        let hour: u32  = ((self.creation_time & 0xF800)>>11) as u32;    
        let min: u32  = ((self.creation_time & 0x07E0)>>5) as u32;    
        let sec: u32  = ((self.creation_time & 0x001F)<<1) as u32; // 秒数需要*2 
        let long_sec: u64 = ((((year - 1980) * 365 + month * 30 + day) * 24 + hour) * 3600 + min*60 + sec) as u64;
        (year,month,day,hour,min,sec,long_sec)
    }

	pub fn get_modification_time(&self) -> (u16, u16) {
		(self.last_mod_date, self.last_mod_time)
    }

	pub fn set_modification_time(&mut self, date: u16, time: u16) {
		self.last_mod_date = date;
		self.last_mod_time = time;
	}

	pub fn get_accessed_time(&self) -> (u16, u16) {
        (self.last_accessed_date, 0)
    }

	pub fn set_accessed_time(&mut self, date: u16) {
		self.last_accessed_date = date;
	}

	// 获取文件名的大写
	pub fn get_name_uppercase(&self) -> String {
		let name_len = (0usize..SHORT_NAME_LEN).find(|i| self.name[*i] == 0x20u8)
			.unwrap_or(SHORT_NAME_LEN);
		let name = core::str::from_utf8(&self.name[..name_len]).unwrap();

		let ext_len = (0usize..SHORT_EXT_LEN).find(|i| self.extension[*i] == 0x20u8)
			.unwrap_or(SHORT_EXT_LEN);
		let extension = core::str::from_utf8(&self.extension[..ext_len]).unwrap();
		if ext_len == 0 {
			name.to_string()
		} else {
			format!("{}.{}", name, extension)
		}
	}

	// 获取文件名的小写
	pub fn get_name_lowercase(&self) -> String {
		let name_len = (0usize..SHORT_NAME_LEN).find(|i| self.name[*i] == 0x20u8)
			.unwrap_or(SHORT_NAME_LEN);
		let name = core::str::from_utf8(&self.name[..name_len]).unwrap();

		let ext_len = (0usize..SHORT_EXT_LEN).find(|i| self.extension[*i] == 0x20u8)
			.unwrap_or(SHORT_EXT_LEN);
		if ext_len == 0 {
			name.to_string().to_ascii_lowercase()
		} else {
			let extension = core::str::from_utf8(&self.extension[..ext_len]).unwrap();
			format!("{}.{}", name.to_ascii_lowercase(), extension.to_ascii_lowercase())
		}
	}

	pub fn checksum(&self) -> u8 {
		self.name.iter().chain(self.extension.iter())
            .fold(0, |acc, &ch| acc.rotate_right(1).wrapping_add(ch))
	}

	pub fn set_size(&mut self, size: u32) {
		self.size_in_bytes = size;
	}

	pub fn get_size(&self) -> u32 {
		self.size_in_bytes
	}

	// 设置文件起始簇号
	pub fn set_first_cluster(&mut self, cluster: u32) {
		self.first_cluster_high = ((cluster & 0xFFFF0000) >> 16) as u16;
		self.first_cluster_low = (cluster & 0x0000FFFF) as u16;
	}

	// 获取文件起始簇号,在文件尚未分配簇时为0
	pub fn first_cluster(&self) -> u32 {
		((self.first_cluster_high as u32) << 16) | (self.first_cluster_low as u32)
	}

	pub fn delete(&mut self) {
		self.size_in_bytes = 0;
		self.name[0] = 0xE5;
		self.set_first_cluster(0);
	}

	pub fn clear(&mut self) {
		self.size_in_bytes = 0;
		self.set_first_cluster(0);
	}

	// 获取文件偏移量所在的簇、扇区和扇区内偏移
	pub fn get_pos(
		&self,
		offset: usize,
		manager: &Arc<FAT32Manager>,
		fat: &Arc<RwLock<FAT>>,
		block_device: &Arc<dyn BlockDevice>,
		chain: &Arc<RwLock<Chain>>,
	) -> (u32, usize, usize) {
		// let fat_reader = fat.read();
		// let bytes_per_sector = manager.bytes_per_sector() as usize;
		// let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		// let cluster_index = offset / bytes_per_cluster;
		// let curr_cluster = fat_reader.get_cluster_at(
		// 	self.first_cluster(), 
		// 	cluster_index, 
		// 	block_device
		// );
		// let curr_sector = manager.first_sector_of_cluster(curr_cluster)
		// 							+ (offset - cluster_index as usize * bytes_per_cluster)
		// 							/ bytes_per_sector;
		// (curr_cluster, curr_sector, offset % bytes_per_sector)
		let bytes_per_sector = manager.bytes_per_sector() as usize;
		let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		let cluster_index = offset / bytes_per_cluster;
		let curr_cluster = chain.write().get_cluster_at(
			self.first_cluster(), 
			cluster_index, 
			block_device,
			fat
		);
		let curr_sector = manager.first_sector_of_cluster(curr_cluster)
									+ (offset - cluster_index as usize * bytes_per_cluster)
									/ bytes_per_sector;
		(curr_cluster, curr_sector, offset % bytes_per_sector)
	}

	// 以偏移量读取文件
	pub fn read_at(
		&self,
		offset: usize,
		buf: &mut [u8],
		manager: &Arc<FAT32Manager>,
		fat: &Arc<RwLock<FAT>>,
		block_device: &Arc<dyn BlockDevice>,
		chain: &Arc<RwLock<Chain>>,
	) -> usize {
		// let fat_reader = fat.read();
		let bytes_per_sector = manager.bytes_per_sector() as usize;
		let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		let mut curr_offset = offset;
		let end: usize;
		// 边界检查
		if self.is_dir() {
			// let size = bytes_per_cluster * fat_reader.cluster_count(self.first_cluster(), block_device) as usize;
			let size = bytes_per_cluster * chain.write().cluster_count(self.first_cluster(), block_device, fat) as usize;
			end = (offset + buf.len()).min(size);
		} else {
			end = (offset + buf.len()).min(self.size_in_bytes as usize);
		}
		if curr_offset >= end {
			return 0;
		}

		// let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device);
		let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device, chain);
		if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
			return 0;
		}

		let mut read_size = 0usize;
		loop {
			// 将偏移量向上对齐扇区大小
			let mut end_current_block = (curr_offset / bytes_per_sector + 1) * bytes_per_sector;
			end_current_block = end_current_block.min(end);
			let block_read_size = end_current_block - curr_offset;
			let dst = &mut buf[read_size..read_size + block_read_size];
			// if self.is_dir() {
			// 	get_info_block_cache(  // 目录项通过Info block cache读取
			// 		curr_sector, 
			// 		Arc::clone(block_device), 
			// 		CacheMode::READ
			// 	)
			// 	.read()
			// 	.read(0, |data_block: &DataBlock| {
			// 		let src = &data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_read_size];
			// 		dst.copy_from_slice(src);
			// 	});
			// } else {
			// 	get_data_block_cache(  // 文件内容通过Data block cache读取
			// 		curr_sector, 
			// 		Arc::clone(block_device), 
			// 		CacheMode::READ
			// 	)
			// 	.read()
			// 	.read(0, |data_block: &DataBlock| {
			// 		let src = &data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_read_size];
			// 		dst.copy_from_slice(src);
			// 	});
			// }
			get_data_block_cache(
				curr_sector, 
				Arc::clone(block_device), 
				CacheMode::READ
			)
			.read()
			.read(0, |data_block: &DataBlock| {
				let src = &data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_read_size];
				dst.copy_from_slice(src);
			});
			read_size += block_read_size;
			if end_current_block == end {
				break;
			}
			curr_offset = end_current_block;
			// 如果读完了一个簇，则需要到下一个簇的起始扇区，否则读取当前簇的下一个扇区
			if curr_offset % bytes_per_cluster == 0 {
				// curr_cluster = fat_reader.get_next_cluster(curr_cluster, block_device);
				curr_cluster = chain.write().get_next_cluster(curr_cluster, block_device, fat);
				if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
					break;
				}
				curr_sector = manager.first_sector_of_cluster(curr_cluster);
			} else {
				curr_sector += 1;
			}
		}
		read_size
	}
	
	// 找不到时返回(-1, -1),找到了返回短文件名目录项的offset和attribute
	pub fn find_short_name(
		&self,
		offset: usize,
		name: &str,
		manager: &Arc<FAT32Manager>,
		fat: &Arc<RwLock<FAT>>,
		block_device: &Arc<dyn BlockDevice>,
		chain: &Arc<RwLock<Chain>>,
	) -> (isize, isize) {
		assert!(self.is_dir());
		// let fat_reader = fat.read();
		let bytes_per_sector = manager.bytes_per_sector() as usize;
		let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		let mut curr_offset = offset;
		// 边界检查
		// let end = bytes_per_cluster * fat_reader.cluster_count(self.first_cluster(), block_device) as usize;
		let end = bytes_per_cluster * chain.write().cluster_count(self.first_cluster(), block_device, fat) as usize;
		if curr_offset >= end {
			return (EMPTY_DIRENT, EMPTY_DIRENT);
		}
		// let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device);
		let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device, chain);
		if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
			return (EMPTY_DIRENT, EMPTY_DIRENT);
		}

		let name_upper = name.to_ascii_uppercase();
		let mut short_offset;
		let mut short_attr;
		loop {
			// 将偏移量向上对齐扇区大小
			let mut end_current_block = (curr_offset / bytes_per_sector + 1) * bytes_per_sector;
			end_current_block = end_current_block.min(end);

			(short_offset, short_attr) = get_info_block_cache(  // 目录项通过Info block cache读取
				curr_sector, 
				Arc::clone(block_device), 
				CacheMode::READ
			)
			.read()
			.read(0, |dirent_blk: &DirentBlock| {
				for (off, short_ent) in dirent_blk.iter().enumerate() {
					if short_ent.is_empty() {
						return (EMPTY_DIRENT, EMPTY_DIRENT);
					} else if short_ent.is_valid() && short_ent.is_short() && name_upper == short_ent.get_name_uppercase() {
						return ((off * DIRENT_SZ + (curr_offset / bytes_per_sector) * bytes_per_sector) as isize, short_ent.attribute() as isize); 
					}
				}
				(NOTFIND_DIRENT, NOTFIND_DIRENT)
			});

			if short_offset != NOTFIND_DIRENT {
				break;
			}
			if end_current_block == end {
				break;
			}
			curr_offset = end_current_block;
			// 如果读完了一个簇，则需要到下一个簇的起始扇区，否则读取当前簇的下一个扇区
			if curr_offset % bytes_per_cluster == 0 {
				// curr_cluster = fat_reader.get_next_cluster(curr_cluster, block_device);
				curr_cluster = chain.write().get_next_cluster(curr_cluster, block_device, fat);
				if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
					panic!("Can not find the short dirent");
					// break;
				}
				curr_sector = manager.first_sector_of_cluster(curr_cluster);
			} else {
				curr_sector += 1;
			}
		}
		(short_offset, short_attr)
	}

	pub fn find_free_dirent(
		&self,
		offset: usize,
		manager: &Arc<FAT32Manager>,
		fat: &Arc<RwLock<FAT>>,
		block_device: &Arc<dyn BlockDevice>,
		chain: &Arc<RwLock<Chain>>,
	) -> usize {
		assert!(self.is_dir());
		// let fat_reader = fat.read();
		let bytes_per_sector = manager.bytes_per_sector() as usize;
		let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		let mut curr_offset = offset;
		// 边界检查
		// let end = bytes_per_cluster * fat_reader.cluster_count(self.first_cluster(), block_device) as usize;
		let end = bytes_per_cluster * chain.write().cluster_count(self.first_cluster(), block_device, fat) as usize;
		if curr_offset >= end {
			return end;
		}
		// let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device);
		let (mut curr_cluster, mut curr_sector, _) = self.get_pos(offset, manager, fat, block_device, chain);
		if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
			return 0;
		}

		loop {
			// 将偏移量向上对齐扇区大小
			let mut end_current_block = (curr_offset / bytes_per_sector + 1) * bytes_per_sector;
			end_current_block = end_current_block.min(end);

			let short_offset = get_info_block_cache(  // 目录项通过Info block cache读取
				curr_sector, 
				Arc::clone(block_device), 
				CacheMode::READ
			)
			.read()
			.read(0, |dirent_blk: &DirentBlock| {
				dirent_blk.iter().enumerate()
					.find(|&(_, short_ent)| short_ent.is_empty())
					.map(|pair| pair.0 * DIRENT_SZ + (curr_offset / bytes_per_sector) * bytes_per_sector)
			});

			if short_offset.is_some() {
				return short_offset.unwrap();
			}
			curr_offset = end_current_block;
			if end_current_block == end {
				break;
			}
			// 如果读完了一个簇，则需要到下一个簇的起始扇区，否则读取当前簇的下一个扇区
			if curr_offset % bytes_per_cluster == 0 {
				// curr_cluster = fat_reader.get_next_cluster(curr_cluster, block_device);
				curr_cluster = chain.write().get_next_cluster(curr_cluster, block_device, fat);
				if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
					break;  // TODO: panic?
				}
				curr_sector = manager.first_sector_of_cluster(curr_cluster);
			} else {
				curr_sector += 1;
			}
		}
		curr_offset
	}

	// 以偏移量写文件，需要保证offset开始的区间在文件范围内
	pub fn write_at(
		&self,
		offset: usize,
		buf: &[u8],
		manager: &Arc<FAT32Manager>,
		fat: &Arc<RwLock<FAT>>,
		block_device: &Arc<dyn BlockDevice>,
		chain: &Arc<RwLock<Chain>>,
	) -> usize {
		// 不会修改文件size，因此不用获取FAT写锁
		// let fat_reader = fat.read();
		let bytes_per_sector = manager.bytes_per_sector() as usize;
		let bytes_per_cluster = manager.bytes_per_cluster() as usize;
		let mut curr_offset = offset;
		let end: usize;
		// 边界检查
		if self.is_dir() {
			// let size = bytes_per_cluster * fat_reader.cluster_count(self.first_cluster(), block_device) as usize;
			let size = bytes_per_cluster * chain.write().cluster_count(self.first_cluster(), block_device, fat) as usize;
			end = (offset + buf.len()).min(size);
		} else {
			end = (offset + buf.len()).min(self.size_in_bytes as usize);
		}
		assert!(curr_offset <= end);

		// let (curr_cluster, curr_sector, _) = self.get_pos(offset, manager, fat, block_device);
		let (curr_cluster, curr_sector, _) = self.get_pos(offset, manager, fat, block_device, chain);
		if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
			panic!("Write error! offset = {}, chain = {:?}, fc = {}", offset, chain.read().chain, self.first_cluster());
		}
		let mut curr_cluster = curr_cluster;
		let mut curr_sector = curr_sector;

		let mut write_size = 0usize;
		loop {
			// 将偏移量向上对齐扇区大小
			let mut end_current_block = (curr_offset / bytes_per_sector + 1) * bytes_per_sector;
			end_current_block = end_current_block.min(end);
			let block_write_size = end_current_block - curr_offset;
			// if self.is_dir() {
			// 	get_info_block_cache(  // 目录项通过Info block cache读取
			// 		curr_sector, 
			// 		Arc::clone(block_device), 
			// 		CacheMode::READ
			// 	)
			// 	.write()
			// 	.modify(0, |data_block: &mut DataBlock| {
			// 		let src = &buf[write_size..write_size + block_write_size];
			// 		let dst = &mut data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_write_size];
			// 		dst.copy_from_slice(src);
			// 	});
			// } else {
			// 	get_data_block_cache(  // 文件内容通过Data block cache读取
			// 		curr_sector, 
			// 		Arc::clone(block_device), 
			// 		CacheMode::READ
			// 	)
			// 	.write()
			// 	.modify(0, |data_block: &mut DataBlock| {
			// 		let src = &buf[write_size..write_size + block_write_size];
			// 		let dst = &mut data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_write_size];
			// 		dst.copy_from_slice(src);
			// 	});
			// }
			get_data_block_cache(
				curr_sector, 
				Arc::clone(block_device), 
				CacheMode::READ
			)
			.write()
			.modify(0, |data_block: &mut DataBlock| {
				let src = &buf[write_size..write_size + block_write_size];
				let dst = &mut data_block[curr_offset % BLOCK_SZ..curr_offset % BLOCK_SZ + block_write_size];
				dst.copy_from_slice(src);
			});
			write_size += block_write_size;
			if end_current_block == end {
				break;
			}
			curr_offset = end_current_block;
			// 如果读完了一个簇，则需要到下一个簇的起始扇区，否则读取当前簇的下一个扇区
			if curr_offset % bytes_per_cluster == 0 {
				// curr_cluster = fat_reader.get_next_cluster(curr_cluster, block_device);
				curr_cluster = chain.write().get_next_cluster(curr_cluster, block_device, fat);
				if curr_cluster >= END_CLUSTER || curr_cluster == 0 {
					panic!("Write error!");
				}
				curr_sector = manager.first_sector_of_cluster(curr_cluster);
			} else {
				curr_sector += 1;
			}
		}
		write_size
	}

	pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as usize as *const u8,
                DIRENT_SZ,
            )
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut _ as usize as *mut u8,
                DIRENT_SZ,
            )
        }
    }
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct LongDirEntry {
	order: 		u8,
	name1: 		[u8; 10],
	attribute: 	u8,
	_reserved_0: u8,
	checksum: 	u8,
	name2: 		[u8; 12],
	_reserved_1: [u8; 2],
	name3: 		[u8; 4],
}

impl LongDirEntry {
	pub fn empty() -> Self {
		Self {
			order: 			0,
			name1: 			[0; 10],
			attribute: 		0,
			_reserved_0: 	0,
			checksum: 		0,
			name2: 			[0; 12],
			_reserved_1: 	[0; 2],
			name3: 			[0; 4],
		}
	}

	pub fn attribute(&self) -> u8 {
		self.attribute
	}

	pub fn is_empty(&self) -> bool {
		self.order == 0x00
	}

	pub fn is_deleted(&self) -> bool {
		self.order == 0xE5
	}

	pub fn order(&self) -> u8 {
		self.order
	}

	pub fn checksum(&self) -> u8 {
		self.checksum
	}

	// 最后一个str如果小于LONG_NAME_LEN，则给进name_buffer需要包含'\0‘
	pub fn initialize(&mut self, name_buffer: &str, order: u8, checksum: u8) {
		let name_buffer = name_buffer.as_bytes();
		let len = name_buffer.len();
		let mut name: [u8; 26] = [0; 26];
		(0..len).for_each(|i| name[i<<1] = name_buffer[i]);
		((len<<1)..26).for_each(|i| name[i] = 0xFF);
		
		let mut name1: [u8; 10] = [0; 10];
        let mut name2: [u8; 12] = [0; 12];
        let mut name3: [u8; 4] = [0; 4];
		let _ = &mut name1[..].copy_from_slice(&name[..10]);
		let _ = &mut name2[..].copy_from_slice(&name[10..22]);
		let _ = &mut name3[..].copy_from_slice(&name[22..26]);

		*self = Self {
			order,
			name1,
			attribute: ATTRIBUTE_LFN,
			_reserved_0: 0,
			checksum,
			name2,
			_reserved_1: [0u8, 2],
			name3,
		}
	}

	pub fn delete(&mut self) {
		self.order = 0xE5;
	}

	// 获取长文件名，不包含填充字符0xFF和结束字符\0
	pub fn get_name_format(&self) -> String {
		let name1: String = self.name1
			.iter().filter(|x| **x != 0xFF && **x != 0)
			.map(|x| *x as char).collect();
		let name2: String = self.name2
			.iter().filter(|x| **x != 0xFF && **x != 0)
			.map(|x| *x as char).collect();
		let name3: String = self.name3
			.iter().filter(|x| **x != 0xFF && **x != 0)
			.map(|x| *x as char).collect();
		format!("{}{}{}", name1, name2, name3)
	}

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const _ as usize as *const u8,
                DIRENT_SZ,
            )
        }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut _ as usize as *mut u8,
                DIRENT_SZ,
            )
        }
    }
}

