use alloc::sync::Arc;

use crate::mm::UserBuffer;

use super::{File, path2vec, OpenFlags, S_IFCHR, Kstat};

pub struct DevZero;
pub struct DevNull;
pub struct DevRtc;

impl DevZero {
    pub fn new() -> Self {
        Self
    }
}

impl File for DevZero {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        user_buf.clear()
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        // do nothing
        user_buf.len()
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
    fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = S_IFCHR;
        kstat
    }
}

impl DevNull {
    pub fn new() -> Self {
        Self
    }
}

impl File for DevNull {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, mut _user_buf: UserBuffer) -> usize {
        // do nothing
        0
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        // do nothing
        user_buf.len()
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
    fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = S_IFCHR;
        kstat
    }
}

pub fn open_device_file(_cwd: &str, path: &str, _flags: OpenFlags) -> Option<Arc<dyn File + Send + Sync>> {
    // warning: just a fake implementation
    let pathv = path2vec(path);
    if let Some(&fname) = pathv.last() {
        match fname {
            "zero" => Some(Arc::new(DevZero::new())),
            "null" => Some(Arc::new(DevNull::new())),
            "rtc" => Some(Arc::new(DevRtc::new())),
            _ => None
        }
    } else {
        None
    }
}

impl DevRtc {
    pub fn new() -> Self {
        Self
    }
}

impl File for DevRtc {
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn read(&self, mut user_buf: UserBuffer) -> usize {
        user_buf.clear()
    }
    fn write(&self, user_buf: UserBuffer) -> usize {
        // do nothing
        user_buf.len()
    }
    fn read_blocking(&self) -> bool {
        false
    }
    fn write_blocking(&self) -> bool {
        false
    }
    fn stat(&self) -> Kstat {
        let mut kstat = Kstat::new();
        kstat.st_mode = S_IFCHR;
        kstat
    }
}