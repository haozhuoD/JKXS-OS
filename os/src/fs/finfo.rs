#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use core::ops::Range;
use fat32_fs::{ATTRIBUTE_ARCHIVE, ATTRIBUTE_DIRECTORY};

#[repr(C)]
pub struct Kstat {
    // st_dev: u64,      /* ID of device containing file */
    // pub st_ino: u64,  /* VFile number */
    // pub st_mode: u32, /* File type and mode */
    // st_nlink: u32,    /* Number of hard links */
    // st_uid: u32,
    // st_gid: u32,
    // st_blksize: u32,
    // st_blocks: u64,
    // pub st_size: i64,
    // pub st_atime_sec: i64,
    // st_atime_nsec: i64,
    // pub st_mtime_sec: i64,
    // st_mtime_nsec: i64,
    // st_ctime_sec: i64,
    // st_ctime_nsec: i64,

    st_dev: u64,      /* ID of device containing file */
    pub st_ino: u64,  /* VFile number */
    pub st_mode: u32, /* File type and mode */
    st_nlink: u32,    /* Number of hard links */
    st_uid: u32,
    st_gid: u32,
    st_rdev: u64,
    _pad: u64,
    pub st_size: i64,
    st_blksize: u32,
    _pad2: u32,
    st_blocks: u64,
    pub st_atime_sec: i64,
    st_atime_nsec: i64,
    pub st_mtime_sec: i64,
    st_mtime_nsec: i64,
    st_ctime_sec: i64,
    st_ctime_nsec: i64,
}

impl Kstat {
    pub fn new() -> Self {
        // Self {
        //     st_dev: 0,   /* ID of device containing file */
        //     st_ino: 0,   /* VFile number */
        //     st_mode: 0,  /* File type and mode */
        //     st_nlink: 1, /* Number of hard links */
        //     st_uid: 0,
        //     st_gid: 0,
        //     st_blksize: 0,
        //     st_blocks: 0,
        //     st_size: 0,
        //     st_atime_sec: 0,
        //     st_atime_nsec: 0,
        //     st_mtime_sec: 0,
        //     st_mtime_nsec: 0,
        //     st_ctime_sec: 0,
        //     st_ctime_nsec: 0,
        // }
        Self {
            st_dev: 0,   /* ID of device containing file */
            st_ino: 0,   /* VFile number */
            st_mode: 0,  /* File type and mode */
            st_nlink: 1, /* Number of hard links */
            st_uid: 0,
            st_gid: 0,
            st_rdev: 0,
            _pad: 0,
            st_size: 0,
            st_blksize: 0,
            _pad2: 0,
            st_blocks: 0,
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

pub enum DType {
    DT_UNKNOWN = 0,
    DT_DIR = 4,
    DT_REG = 8,
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

#[repr(C)]
pub struct IOVec {
    pub iov_base: *mut u8,
    pub iov_len: usize,
}

#[repr(C)]
pub struct Pollfd {
    pub fd: u32,
    pub events: u16,
    pub revents: u16,
}

pub const POLLIN: u16 = 0x001;
pub const POLLPRI: u16 = 0x002;
pub const POLLOUT: u16 = 0x004;
pub const POLLERR: u16 = 0x008;
pub const POLLHUP: u16 = 0x010;
pub const POLLNVAL: u16 = 0x020;
pub const POLLRDNORM: u16 = 0x040;
pub const POLLRDBAND: u16 = 0x080;

pub const S_IFMT: u32 = 0o170000; //bit mask for the file type bit field
pub const S_IFREG: u32 = 0o100000; //regular file
pub const S_IFBLK: u32 = 0o060000; //block device
pub const S_IFDIR: u32 = 0o040000; //directory
pub const S_IFCHR: u32 = 0o020000; //character device
pub const S_IFIFO: u32 = 0o010000; //FIFO

pub const S_ISUID: u32 = 0o4000; //set-user-ID bit (see execve(2))
pub const S_ISGID: u32 = 0o2000; //set-group-ID bit (see below)
pub const S_ISVTX: u32 = 0o1000; //sticky bit (see below)

pub const S_IRWXU: u32 = 0o0700; //owner has read, write, and execute permission
pub const S_IRUSR: u32 = 0o0400; //owner has read permission
pub const S_IWUSR: u32 = 0o0200; //owner has write permission
pub const S_IXUSR: u32 = 0o0100; //owner has execute permission

pub const S_IRWXG: u32 = 0o0070; //group has read, write, and execute permission
pub const S_IRGRP: u32 = 0o0040; //group has read permission
pub const S_IWGRP: u32 = 0o0020; //group has write permission
pub const S_IXGRP: u32 = 0o0010; //group has execute permission

pub const S_IRWXO: u32 = 0o0007; //others (not in group) have read, write,and execute permission
pub const S_IROTH: u32 = 0o0004; //others have read permission
pub const S_IWOTH: u32 = 0o0002; //others have write permission
pub const S_IXOTH: u32 = 0o0001; //others have execute permission

pub const SEEK_SET: usize = 0;
pub const SEEK_CUR: usize = 1;
pub const SEEK_END: usize = 2;

// #define FD_SETSIZE 256
// pub const FD_SETSIZE: usize = 128;
// typedef struct { uint32_t fd32[FD_SETSIZE/32]; } fd_set;
pub type FdSet = u128;

pub trait BitOpt {
    // length() 方法是一个静态方法,用于返回bit的长度
    fn length() -> usize;
    // get_bit() 方法用于获取size位的值,可以使用bool类型表示1和0
    fn u128_get_bit(&self, size: usize) -> bool;
    // get_bits() 方法用于获取指定范围的位,使用Range表示一个范围,例如2..5表示2(含2)到5(不包括5)的范围
    // (1..5) = Range{ start: 1, end: 5 }
    fn u128_get_bits(&self, range: Range<usize>) -> Self;
    // set_bit() 方法用于设置指定的位
    fn u128_set_bit(&mut self, bit: usize, value: bool) -> &mut Self;
    // set_bits 方法用于设置指定范围的位
    fn u128_set_bits(&mut self, range: Range<usize>, value: Self) -> &mut Self;
    // clear_all 方法用于清零
    fn u128_clear_all(&mut self) -> &mut Self;
}

impl BitOpt for u128 {
    fn length() -> usize {
        ::core::mem::size_of::<Self>() as usize * 8
    }

    fn u128_get_bit(&self, size: usize) -> bool {
        assert!(size < Self::length());
        (*self & (1 << size)) != 0
    }

    fn u128_set_bit(&mut self, bit: usize, value: bool) -> &mut Self {
        assert!(bit < Self::length());
        let mask = 1 << bit;
        // 如果要把某一位置1使用或运算
        if value {
            *self |= mask;
        } else {
            // 要把某一位置0进行和运算(需要取反)
            *self &= !(mask);
        }
        self
    }

    fn u128_get_bits(&self, range: Range<usize>) -> Self {
        assert!(range.start < Self::length());
        assert!(range.end <= Self::length());
        assert!(range.end > range.start);

        let shift_bits = Self::length() - range.end;
        let bits = *self << shift_bits >> shift_bits;

        bits >> range.start
    }

    fn u128_set_bits(&mut self, range: Range<usize>, value: Self) -> &mut Self {
        let length = Self::length();
        assert!(range.start < length);
        assert!(range.end <= length);
        assert!(range.start < range.end);
        assert!(
            value << (length - (range.end - range.start)) >> (length - (range.end - range.start))
                == value,
            "value does not fit into bit range"
        );

        let mask: Self =
            !(!0 << (length - range.end) >> (length - range.end) >> range.start << range.start);

        *self = (*self & mask) | (value << range.start);

        self
    }

    fn u128_clear_all(&mut self) -> &mut Self {
        *self = 0;
        self
    }
}

const FAT_SUPER_MAGIC: i64 = 0x4006;

#[repr(C)]
pub struct Statfs {
    f_type: i64,        // Type of filesystem
    f_bsize: i64,       // Optimal transfer block size
    f_blocks: i64,      // Total data blocks in filesystem
    f_bfree: i64,       // Free blocks in filesystem
    f_bavail: i64,      // Free blocks available to unprivileged user
    f_files: i64,       // Total inodes in filesystem
    f_ffree: i64,       // Free inodes in filesystem
    f_fsid: i64,        // Filesystem ID
    f_name_len: i64,    // Maximum length of filenames
    f_frsize: i64,      // Fragment size
    f_flags: i64,       // Mount flags of filesystem
    f_spare: [i64; 4],  // Padding bytes
}

impl Statfs {
    pub fn new() -> Self {
        Self {
            f_type: FAT_SUPER_MAGIC,
            f_bsize: 512,
            f_blocks: 1048576,
            f_bfree: 1048576,
            f_bavail: 0,
            f_files: 131072,
            f_ffree: 131072,
            f_fsid: 0,
            f_name_len: 255,
            f_frsize: 0,
            f_flags: 0,
            f_spare: [0; 4],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *mut u8, size) }
    }
}

