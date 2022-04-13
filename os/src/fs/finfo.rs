#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use fat32_fs::{ATTRIBUTE_DIRECTORY, ATTRIBUTE_ARCHIVE};

#[repr(C)]
pub struct Kstat {
    st_dev: u64,   /* ID of device containing file */
    st_ino: u64,   /* VFile number */
    st_mode: u32,  /* File type and mode */
    st_nlink: u32, /* Number of hard links */
    st_uid: u32,
    st_gid: u32,
    st_blksize: u32,
    st_blocks: u64,
    pub st_size: i64,
    st_atime_sec: i64,
    st_atime_nsec: i64,
    st_mtime_sec: i64,
    st_mtime_nsec: i64,
    st_ctime_sec: i64,
    st_ctime_nsec: i64,
}

impl Kstat {
    pub fn empty() -> Self {
        Self {
            st_dev: 0,   /* ID of device containing file */
            st_ino: 0,   /* VFile number */
            st_mode: 0,  /* File type and mode */
            st_nlink: 0, /* Number of hard links */
            st_uid: 0,
            st_gid: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_size: 0,
            st_atime_sec: 0,
            st_atime_nsec: 0,
            st_mtime_sec: 0,
            st_mtime_nsec: 0,
            st_ctime_sec: 0,
            st_ctime_nsec: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *mut u8, size) }
    }
}

#[repr(packed)]
pub struct FSDirent {
    d_ino: u64,    // 索引结点号
    d_off: i64,    // 到下一个dirent的偏移
    d_reclen: u16, // 当前dirent的长度
    d_type: u8,    // 文件类型
}

impl FSDirent {
    pub fn new(d_ino: u64, d_off: i64, d_reclen: u16, d_type: u8) -> Self {
        Self {
            d_ino,
            d_off,
            d_reclen,
            d_type,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *mut u8, size) }
    }
}

pub enum DType
{
    DT_UNKNOWN = 0,
    // DT_FIFO = 1,
    // DT_CHR = 2,
    DT_DIR = 4,
    // DT_BLK = 6,
    DT_REG = 8,
    // DT_LNK = 10,
    // DT_SOCK = 12,
    // DT_WHT = 14
}

impl DType {
    pub fn from_attribute(attribute: u8) -> Self {
        if attribute & ATTRIBUTE_DIRECTORY != 0 {
            Self::DT_DIR
        } else if attribute & ATTRIBUTE_ARCHIVE != 0 {
            Self::DT_REG
        } else {
            Self::DT_UNKNOWN
        }
    }
}