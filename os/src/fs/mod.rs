mod finfo;
mod inode;
mod pipe;
mod stdio;

use crate::mm::UserBuffer;
use alloc::sync::Arc;

/// 枚举类型，分为普通文件和抽象文件
#[derive(Clone)]
pub enum FileClass {
    File(Arc<OSInode>),
    Abs(Arc<dyn File + Send + Sync>),
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
}

pub use finfo::*;
pub use inode::{list_apps, open_file, OSInode, OpenFlags};
pub use pipe::{make_pipe, Pipe};
pub use stdio::{Stdin, Stdout};
