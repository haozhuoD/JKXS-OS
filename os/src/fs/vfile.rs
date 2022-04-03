use super::File;
use crate::drivers::BLOCK_DEVICE;
use crate::mm::UserBuffer;

use alloc::sync::Arc;
use alloc::vec::Vec;
use bitflags::*;
use fat32_fs::{FAT32Manager, VFile, ATTRIBUTE_ARCHIVE, ATTRIBUTE_DIRECTORY};
use lazy_static::*;
use spin::Mutex;

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

    pub fn file_size(&self) -> usize {
        let inner = self.inner.lock();
        inner.vfile.get_size() as usize
    }

    pub fn set_offset(&self, offset: usize) -> usize {
        self.inner.lock().offset = offset;
        offset
    }
}

lazy_static! {
    pub static ref ROOT_VFILE: Arc<VFile> = {
        let fat32_fs = FAT32Manager::open(BLOCK_DEVICE.clone());
        let fs_inner = fat32_fs.read();
        Arc::new(fs_inner.get_root_vfile(&fat32_fs))
    };
}

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
        const CREATE = 1 << 6;
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

pub fn open_file(path: &str, flags: OpenFlags) -> Option<Arc<OSFile>> {
    let (readable, writable) = flags.read_write();

    let mut pathv: Vec<&str> = path.split("/").collect();

    // println!("pathv = {:#x?}", pathv);

    if flags.contains(OpenFlags::CREATE) {
        // 先找到父级目录对应节点
        let filename = pathv.pop().unwrap();
        if let Some(parent_dir) = ROOT_VFILE.find_vfile_bypath(pathv) {
            if let Some(vfile) = parent_dir.find_vfile_byname(filename) {
                // 删除已存在的文件
                vfile.remove();
            }
            // 新建文件
            let mut filetype = ATTRIBUTE_ARCHIVE;
            if flags.contains(OpenFlags::DIRECTORY) {
                filetype = ATTRIBUTE_DIRECTORY;
            }
            parent_dir
                .create(filename, filetype)
                .map(|vfile| Arc::new(OSFile::new(readable, writable, vfile)))
        } else {
            None
        }
    }
    else {
        ROOT_VFILE.find_vfile_bypath(pathv).map(|vfile| {
            Arc::new(OSFile::new(readable, writable, vfile))
        })
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
