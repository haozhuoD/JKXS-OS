mod finfo;
mod pipe;
mod stdio;
mod vfile;
mod devfs;
mod fsidx;

use crate::mm::UserBuffer;
use alloc::{sync::Arc, string::String, vec::Vec};

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
pub use vfile::*;
pub use devfs::open_device_file;
pub use fsidx::*;

pub fn path2abs<'a>(cwdv: &mut Vec<&'a str>, pathv: &Vec<&'a str>) -> String {
    for &path_element in pathv.iter() {
        if path_element.is_empty() || path_element == "." {
            continue;
        } else if path_element == ".." {
            cwdv.pop();
        } else {
            cwdv.push(path_element);
        }
    }
    let mut abs_path = String::from("/");
    abs_path.push_str(&cwdv.join("/"));
    abs_path
}