use super::File;
use crate::drivers::BLOCK_DEVICE;
use crate::mm::UserBuffer;

use alloc::vec::Vec;
use alloc::{string::String, sync::Arc};
use bitflags::*;
use fat32_fs::{FAT32Manager, VFile, ATTRIBUTE_ARCHIVE, ATTRIBUTE_DIRECTORY};
use spin::{Lazy, Mutex};

pub struct OSFile {
    readable: bool,
    writable: bool,
    inner: Arc<Mutex<OSFileInner>>,
}

pub struct OSFileInner {
    offset: usize,
    vfile: Arc<VFile>,
}

impl OSFile {
    pub fn new(readable: bool, writable: bool, vfile: Arc<VFile>) -> Self {
        Self {
            readable,
            writable,
            inner: Arc::new(Mutex::new(OSFileInner { offset: 0, vfile })),
        }
    }

    pub fn read_all(&self) -> Vec<u8> {
        let mut inner = self.inner.lock();
        let mut buffer = [0u8; 512];
        let mut v: Vec<u8> = Vec::new();
        loop {
            let len = inner.vfile.read_at(inner.offset, &mut buffer);
            if len == 0 {
                break;
            }
            inner.offset += len;
            v.extend_from_slice(&buffer[..len]);
        }
        v
    }

    pub fn find(&self, path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
        let inner = self.inner.lock();
        let pathv = path2vec(path);
        let (readable, writable) = flags.read_write();
        inner
            .vfile
            .find_vfile_path(pathv)
            .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
    }

    pub fn remove(&self) -> usize {
        let inner = self.inner.lock();
        inner.vfile.remove()
    }

    pub fn delete(&self) {
        self.inner.lock().vfile.delete();
    }

    pub fn file_size(&self) -> usize {
        let inner = self.inner.lock();
        inner.vfile.get_size() as usize
    }

    pub fn set_file_size(&self, size: u32) {
        self.inner.lock().vfile.set_size(size);
    }

    pub fn dirent_info(&self, offset: usize) -> Option<(String, u32, u32, u8)> {
        let inner = self.inner.lock();
        inner.vfile.dirent_info(offset)
    }

    pub fn is_dir(&self) -> bool {
        let inner = self.inner.lock();
        inner.vfile.is_dir()
    }

    pub fn inode_id(&self) -> u32 {
        let inner = self.inner.lock();
        inner.vfile.first_cluster()
    }

    pub fn set_inode_id(&self, inode_id: u32) {
        self.inner.lock().vfile.set_first_cluster(inode_id);
    }

    pub fn offset(&self) -> usize {
        self.inner.lock().offset
    }

    pub fn seek(&self, offset: usize) -> usize {
        self.inner.lock().offset = offset;
        offset
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
    let _proc = open_file("/","proc", OpenFlags::CREATE | OpenFlags::DIRECTORY ).unwrap();
    let _mounts = open_file("/proc","mounts", OpenFlags::CREATE | OpenFlags::DIRECTORY).unwrap();
    let _meminfo = open_file("/proc","meminfo", OpenFlags::CREATE | OpenFlags::DIRECTORY).unwrap();
    // let file = open("/","ls", OpenFlags::CREATE, DiskInodeType::File).unwrap();
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
        const _X11 = 1 << 11;
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

fn do_create_file(
    cur_vfile: Arc<VFile>,
    mut pathv: Vec<&str>,
    flags: OpenFlags,
) -> Option<Arc<OSFile>> {
    let name = pathv.pop().unwrap_or("/");
    if let Some(parent_dir) = cur_vfile.find_vfile_path(pathv.clone()) {
        let attribute = {
            if flags.contains(OpenFlags::DIRECTORY) {
                ATTRIBUTE_DIRECTORY
            } else {
                ATTRIBUTE_ARCHIVE
            }
        };
        let (readable, writable) = flags.read_write();
        parent_dir
            .create(name, attribute)
            .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
    } else {
        None
    }
}

pub fn open_file(cwd: &str, path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
    let cur_vfile = {
        if cwd == "/" {
            ROOT_VFILE.clone()
        } else {
            let wpath = path2vec(cwd);
            ROOT_VFILE.find_vfile_path(wpath).unwrap()
        }
    }; // 当前工作路径对应节点
    let (readable, writable) = flags.read_write();
    // println!("open_file");

    let pathv = path2vec(path);

    // 节点是否存在？
    if let Some(inode) = cur_vfile.find_vfile_path(pathv.clone()) {
        if flags.contains(OpenFlags::TRUNC) {
            inode.remove();
            return do_create_file(cur_vfile, pathv, flags);
        }
        let vfile = OSFile::new(readable, writable, inode);
        if flags.contains(OpenFlags::APPEND) {
            vfile.seek(vfile.file_size());
        }
        return Some(Arc::new(vfile));
    }

    // 节点不存在
    if flags.contains(OpenFlags::CREATE) {
        return do_create_file(cur_vfile, pathv, flags);
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
        for slice in buf.buffers.iter_mut() {
            let read_size = inner.vfile.read_at(inner.offset, *slice);
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
        for slice in buf.buffers.iter() {
            let write_size = inner.vfile.write_at(inner.offset, *slice);
            assert_eq!(write_size, slice.len());
            inner.offset += write_size;
            total_write_size += write_size;
        }
        total_write_size
    }
}

pub fn path2vec(path: &str) -> Vec<&str> {
    path.split("/").filter(|x| *x != "").collect()
}
