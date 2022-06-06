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

    pub fn file_size(&self) -> usize {
        let inner = self.inner.lock();
        inner.vfile.get_size() as usize
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
        const _X9 = 1 << 9;
        const _X10 = 1 << 10;
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

pub fn open_file(cwd: &str, path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
    let cur_vfile = {
        if cwd == "/" {
            ROOT_VFILE.clone()
        } else {
            let wpath = path2vec(cwd);
            ROOT_VFILE.find_vfile_path(wpath).unwrap()
        }
    };
    let (readable, writable) = flags.read_write();

    let mut pathv = path2vec(path);

    if flags.contains(OpenFlags::CREATE) {
        // 先找到父级目录对应节点
        if let Some(inode) = cur_vfile.find_vfile_path(pathv.clone()) {
            inode.remove();
        }
        let name = pathv.pop().unwrap_or("/");
        if let Some(parent_dir) = cur_vfile.find_vfile_path(pathv.clone()) {
            let attribute = {
                if flags.contains(OpenFlags::DIRECTORY) {
                    ATTRIBUTE_DIRECTORY
                } else {
                    ATTRIBUTE_ARCHIVE
                }
            };
            parent_dir
                .create(name, attribute)
                .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
        } else {
            None
        }
    } else {
        cur_vfile
            .find_vfile_path(pathv)
            .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
    }
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
 