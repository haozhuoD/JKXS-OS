mod finfo;
mod pipe;
mod stdio;
mod vfile;

use crate::mm::UserBuffer;
use alloc::sync::Arc;

/// 枚举类型，分为普通文件和抽象文件
/// 普通文件File，特点是支持更多类型的操作，包含seek, set_offset等
/// 抽象文件Abs，抽象文件，只支持File trait的一些操作
#[derive(Clone)]    
pub enum FileClass {
    File(Arc<OSFile>),
    Abs(Arc<dyn File + Send + Sync>),
}

pub trait File: Send + Sync {
    fn readable(&self) -> bool;
    fn writable(&self) -> bool;
    fn read(&self, buf: UserBuffer) -> usize;
    fn write(&self, buf: UserBuffer) -> usize;
    fn read_blocking(&self) -> bool;
    fn write_blocking(&self) -> bool;
}

pub use finfo::*;
pub use pipe::{make_pipe, Pipe,PipeRingBuffer};
pub use stdio::{Stdin, Stdout};
pub use vfile::{init_rootfs, list_apps, open_common_file, path2vec, OSFile, OpenFlags};
