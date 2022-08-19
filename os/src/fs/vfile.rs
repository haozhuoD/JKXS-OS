use super::{File, find_vfile_idx, insert_vfile_idx, path2abs, remove_vfile_idx};
use super::{Kstat, S_IFCHR, S_IFDIR, S_IRWXU, S_IRWXG, S_IRWXO, S_IFREG};
use crate::drivers::BLOCK_DEVICE;
use crate::mm::UserBuffer;

use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::{string::String, sync::Arc};
use bitflags::*;
use fat32_fs::{FAT32Manager, VFile, ATTRIBUTE_ARCHIVE, ATTRIBUTE_DIRECTORY};
use spin::{Lazy, Mutex};

/// OSFile表示在磁盘上真实存在的文件
pub struct OSFile {
    readable: bool,
    writable: bool,
    vfile: Arc<VFile>,
    inner: Arc<Mutex<OSFileInner>>,
}

pub struct OSFileInner {
    offset: usize,
    atime: u64,
    mtime: u64,
}

impl OSFile {
    pub fn new(readable: bool, writable: bool, vfile: Arc<VFile>) -> Self {
        Self {
            readable,
            writable,
            vfile,
            inner: Arc::new(Mutex::new(OSFileInner { offset: 0, atime: 0, mtime: 0})),
        }
    }

    pub unsafe fn read_as_elf(&self) -> &[u8] {
        self.vfile.read_as_elf()
    }

    pub unsafe fn get_data_cache_physaddr(&self, offset: usize) -> usize {
        self.vfile.get_data_cache_physaddr(offset)
    }

    pub fn find(&self, path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
        let pathv = path2vec(path);
        let (readable, writable) = flags.read_write();
        self
            .vfile
            .find_vfile_path(&pathv)
            .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
    }

    pub fn remove(&self) -> usize {
        self.vfile.remove()
    }

    pub fn delete(&self) {
        self.vfile.delete()
    }

    pub fn file_size(&self) -> usize {
        self.vfile.get_size() as usize
    }

    pub fn set_file_size(&self, size: u32) {
        self.vfile.set_size(size);
    }

    pub fn dirent_info(&self, offset: usize) -> Option<(String, u32, u32, u8)> {
        self.vfile.dirent_info(offset)
    }

    pub fn is_dir(&self) -> bool {
        self.vfile.is_dir()
    }

    pub fn inode_id(&self) -> u32 {
        self.vfile.first_cluster()
    }

    pub fn set_inode_id(&self, inode_id: u32) {
        self.vfile.set_first_cluster(inode_id);
    }

    pub fn offset(&self) -> usize {
        self.inner.lock().offset
    }

    pub fn set_offset(&self, offset: usize) -> usize {
        self.inner.lock().offset = offset;
        offset
    }

    pub fn name(&self) -> String {
        self.vfile.get_name()
    }

    pub fn set_modification_time(&self, mtime: u64) {
        // self.inner.lock().vfile.set_modification_time(mtime);
        self.inner.lock().mtime = mtime;
    }

    pub fn modification_time(&self) -> u64 {
        // self.inner.lock().vfile.modification_time()
        self.inner.lock().mtime
    }

    pub fn set_accessed_time(&self, atime: u64) {
        // self.inner.lock().vfile.set_accessed_time(atime);
        self.inner.lock().atime = atime;
    }

    pub fn accessed_time(&self) -> u64 {
        // self.inner.lock().vfile.accessed_time()
        self.inner.lock().atime
    }

    pub fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = {
            if self.vfile.is_dir() {
                S_IFDIR | S_IRWXU | S_IRWXG | S_IRWXO
            } else {
                S_IFREG | S_IRWXU | S_IRWXG | S_IRWXO
            }
        };
        // kstat.st_ino = inner.vfile.first_cluster() as u64;
        kstat.st_size = self.vfile.get_size() as i64;
        // kstat.st_atime_sec = self.atime as i64;
        // kstat.st_mtime_sec = self.mtime as i64;
        kstat
    }
}

pub static ROOT_VFILE: Lazy<Arc<VFile>> = Lazy::new(|| {
    let fat32_fs = FAT32Manager::open(BLOCK_DEVICE.clone());
    Arc::new(fat32_fs.get_root_vfile(&fat32_fs))
});

pub fn list_apps() {
    println!("/**** APPS ****");
    for (app, _) in ROOT_VFILE.ls().unwrap() {
        println!("{}", app);
    }
    println!("**************/")
}

pub fn init_rootfs(){
    let _proc = open_common_file("/","proc", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _mounts = open_common_file("/proc","mounts", OpenFlags::CREATE | OpenFlags::DIRECTORY).unwrap();
    let _meminfo = open_common_file("/proc","meminfo", OpenFlags::CREATE | OpenFlags::DIRECTORY).unwrap();
    let _var = open_common_file("/","var", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _tmp = open_common_file("/","tmp", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _var_tmp = open_common_file("/","/var/tmp", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _dev = open_common_file("/", "dev", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _null = open_common_file("/", "dev/null", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _invalid = open_common_file("/", "dev/null/invalid", OpenFlags::CREATE | OpenFlags::RDWR ).unwrap();
}

bitflags! {
    pub struct OpenFlags: u32 {
        const RDONLY = 0;
        const WRONLY = 1 << 0;
        const RDWR = 1 << 1;
        const _X2 = 1 << 2;
        const _X3 = 1 << 3;
        const _X4 = 1 << 4;
        const _X5 = 1 << 5;
        const CREATE = 1 << 6;
        const EXCL = 1 << 7;
        const _X8 = 1 << 8;
        const TRUNC = 1 << 9;
        const APPEND = 1 << 10;
        const NONBLOCK = 1 << 11;
        const _X12 = 1 << 12;
        const _X13 = 1 << 13;
        const _X14 = 1 << 14;
        const LARGEFILE = 1 << 15;
        const DIRECTORY_ = 1 << 16;
        const _X17 = 1 << 17;
        const _X18 = 1 << 18;
        const CLOEXEC = 1 << 19;
        const _X20 = 1 << 20;
        const DIRECTORY = 1 << 21;
    }
}

impl OpenFlags {
    /// Do not check validity for simplicity
    /// Return (readable, writable)
    pub fn read_write(&self) -> (bool, bool) {
        if self.is_empty() {
            (true, false)
        } else if self.contains(Self::WRONLY) {
            (false, true)
        } else {
            (true, true)
        }
    }
}

fn do_create_common_file(
    cwd: &str,
    path: &str,
    flags: OpenFlags,
    abs_path: &str,
    parent_path: &str,
    child_name: &str,
) -> Option<Arc<OSFile>> {
    if let Some(parent_dir) = find_vfile_idx(parent_path) {
        let attribute = {
            if flags.contains(OpenFlags::DIRECTORY) {
                ATTRIBUTE_DIRECTORY
            } else {
                ATTRIBUTE_ARCHIVE
            }
        };
        let (readable, writable) = flags.read_write();
        return parent_dir
            .create(child_name, attribute)
            .map(|vfile| {
                insert_vfile_idx(abs_path, vfile.clone());
                Arc::new(OSFile::new(readable, writable, vfile))
            });
    }
    let mut pathv = path2vec(path);
    pathv.pop();
    let cur_vfile = {
        if cwd == "/" {
            ROOT_VFILE.clone()
        } else {
            ROOT_VFILE.find_vfile_path(&path2vec(cwd)).unwrap()
        }
    };
    if let Some(parent_dir) = cur_vfile.find_vfile_path(&pathv) {
        let attribute = {
            if flags.contains(OpenFlags::DIRECTORY) {
                ATTRIBUTE_DIRECTORY
            } else {
                ATTRIBUTE_ARCHIVE
            }
        };
        let (readable, writable) = flags.read_write();
        parent_dir
            .create(child_name, attribute)
            .map(|vfile| {
                insert_vfile_idx(abs_path, vfile.clone());
                Arc::new(OSFile::new(readable, writable, vfile))
            })
    } else {
        None
    }
}

pub fn open_common_file(cwd: &str, path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
    let abs_path = if is_abs_path(path) {
        path.to_string()
    } else {
        let mut wpath = {
            if cwd == "/" {
                Vec::with_capacity(32)
            } else {
                path2vec(cwd)
            }
        };
        path2abs(&mut wpath, &path2vec(path))
    };
    // 首先在FSIDX中查找文件是否存在
    if let Some(inode) = find_vfile_idx(&abs_path) {
        if flags.contains(OpenFlags::TRUNC) {
            let (mut parent_path, child_name) = abs_path.rsplit_once("/").unwrap();
            if parent_path.is_empty() {
                parent_path = "/";
            }
            remove_vfile_idx(&abs_path);
            inode.remove();
            return do_create_common_file(cwd, path, flags, &abs_path, parent_path, child_name);
        }
        let (readable, writable) = flags.read_write();
        let vfile = OSFile::new(readable, writable, inode);
        if flags.contains(OpenFlags::APPEND) {
            vfile.set_offset(vfile.file_size());
        }
        return Some(Arc::new(vfile));
    }

    // 若在FSIDX中无法找到，尝试在FSIDX寻找父级目录
    let (mut parent_path, child_name) = abs_path.rsplit_once("/").unwrap();
    if parent_path.is_empty() {
        parent_path = "/";
    }
    if let Some(parent_inode) = find_vfile_idx(parent_path) {
        if let Some(inode) = parent_inode.find_vfile_name(child_name).map(|f| Arc::new(f)) {
            if flags.contains(OpenFlags::TRUNC) {
                remove_vfile_idx(&abs_path);
                inode.remove();
                return do_create_common_file(cwd, path, flags, &abs_path, parent_path, child_name);
            }
            insert_vfile_idx(&abs_path, inode.clone());
            let (readable, writable) = flags.read_write();
            let vfile = OSFile::new(readable, writable, inode);
            if flags.contains(OpenFlags::APPEND) {
                vfile.set_offset(vfile.file_size());
            }
            return Some(Arc::new(vfile));
        } 
    } else {
        let cur_vfile = {
            if cwd == "/" {
                ROOT_VFILE.clone()
            } else {
                ROOT_VFILE.find_vfile_path(&path2vec(cwd)).unwrap()
            }
        };
        if let Some(inode) = cur_vfile.find_vfile_path(&path2vec(path)) {
            if flags.contains(OpenFlags::TRUNC) {
                remove_vfile_idx(&abs_path);
                inode.remove();
                return do_create_common_file(cwd, path, flags, &abs_path, parent_path, child_name);
            }
            insert_vfile_idx(&abs_path, inode.clone());
            let (readable, writable) = flags.read_write();
            let vfile = OSFile::new(readable, writable, inode);
            if flags.contains(OpenFlags::APPEND) {
                vfile.set_offset(vfile.file_size());
            }
            return Some(Arc::new(vfile));
        }
    }

    // 节点不存在
    if flags.contains(OpenFlags::CREATE) {
        return do_create_common_file(cwd, path, flags, &abs_path, parent_path, child_name);
    }
    None
}

impl File for OSFile {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, mut buf: UserBuffer) -> usize {
        let mut inner = self.inner.lock();
        let mut total_read_size = 0usize;
        for slice in buf.bufvec.bufs[0..buf.bufvec.sz].iter_mut() {
            let read_size = self.vfile.read_at(inner.offset, unsafe {
                core::slice::from_raw_parts_mut(slice.0 as *mut u8, slice.1 - slice.0)
            });
            if read_size == 0 {
                break;
            }
            inner.offset += read_size;
            total_read_size += read_size;
        }
        total_read_size
    }
    fn write(&self, buf: UserBuffer) -> usize {
        let mut inner = self.inner.lock();
        let mut total_write_size = 0usize;
        for slice in buf.bufvec.bufs[0..buf.bufvec.sz].iter() {
            let write_size = self.vfile.write_at(inner.offset, unsafe {
                core::slice::from_raw_parts(slice.0 as *const u8, slice.1 - slice.0)
            });
            assert_eq!(write_size, slice.1 - slice.0);
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
}

#[inline(always)]
pub fn path2vec(path: &str) -> Vec<&str> {
    path.split('/').filter(|s| !s.is_empty()).collect()
}

#[inline(always)]
pub fn is_abs_path(path: &str) -> bool {
    unsafe { *path.as_ptr() == '/' as u8}
}