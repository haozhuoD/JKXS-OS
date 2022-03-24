use crate::fs::{make_pipe, open_file, File, FileClass, Kstat, OpenFlags};
use crate::gdb_println;
use crate::mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer};
use crate::monitor::*;
use crate::task::{current_process, current_task, current_user_token};
use alloc::sync::Arc;
use core::mem::size_of;

const AT_FDCWD: isize = -100;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let mut f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.writable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);

        let ret = f.write(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        if fd == 2 {
            let str = str::replace(translated_str(token, buf).as_str(), "\n", "\\n");
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_write(fd: {}, buf: \"{}\", len: {}) = {}",
                fd,
                str,
                len,
                ret
            );
        } else if fd > 2 {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_write(fd: {}, buf: ?, len: {}) = {}",
                fd,
                len,
                ret
            );
        }
        ret as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let mut f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        let ret = f.read(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        if fd > 2 {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_read(fd: {}, buf: *** , len: {}) = {}",
                fd,
                len,
                ret
            );
        }
        ret as isize
    } else {
        -1
    }
}

pub fn sys_open_at(dirfd: isize, path: *const u8, flags: u32, mode: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);

    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = process.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(FileClass::File(inode));
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_open_at(dirfd: {}, path: {:?}, flags: {:#x?}, mode: {:#x?}) = {}",
            dirfd,
            path,
            flags,
            mode,
            0
        );
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, -1);
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, -1);
        return -1;
    }
    inner.fd_table[fd].take();
    gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, 0);
    0
}

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let process = current_process();
    let token = current_user_token();
    let mut inner = process.inner_exclusive_access();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(FileClass::Abs(pipe_read));
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(FileClass::Abs(pipe_write));
    *translated_refmut(token, pipe) = read_fd;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd;
    gdb_println!(SYSCALL_ENABLE, "sys_pipe() = [{}, {}]", read_fd, write_fd);
    0
}

pub fn sys_dup(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = inner.fd_table[fd].clone();
    gdb_println!(SYSCALL_ENABLE, "sys_dup(fd: {}) = {}", fd, new_fd);
    new_fd as isize
}

/// 将文件描述符为fd的文件信息填入buf
pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let inner = process.inner_exclusive_access();

    let mut userbuf = UserBuffer::new(buf_vec);
    let mut kstat = Kstat::empty();

    if fd == AT_FDCWD {
        unimplemented!();
    }

    if fd < 0 || fd >= inner.fd_table.len() as isize {
        -1
    } else if let Some(file) = inner.fd_table[fd as usize].clone() {
        match file {
            FileClass::File(f) => {
                kstat.st_size = f.file_size() as i64;
                userbuf.write(kstat.as_bytes());
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_dup(fd: {}, buf = {:x?}) = {}",
                    fd,
                    buf,
                    0
                );
                0
            }
            _ => -1,
        }
    } else {
        -1
    }
}
