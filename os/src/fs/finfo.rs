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
