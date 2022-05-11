use crate::fs::{
    make_pipe, open_file, path2vec, DType, FSDirent, File, FileClass, IOVec, Kstat, OSFile,
    OpenFlags, S_IFDIR, S_IRWXU, S_IFREG, S_IRWXG, S_IRWXO,
};
use crate::gdb_println;
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};

use crate::monitor::{QEMU, SYSCALL_ENABLE};
use crate::syscall::errorno::{ENXIO, ENOENT};
use crate::task::{current_process, current_user_token};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use fat32_fs::DIRENT_SZ;
use core::mem::size_of;

use super::errorno::EPERM;

const AT_FDCWD: isize = -100;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.writable() {
            return -EPERM;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);

        let ret = f.write(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        if fd == 2 {
            let _str = str::replace(translated_str(token, buf).as_str(), "\n", "\\n");
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_write(fd: {}, buf: \"{}\", len: {}) = {}",
                fd,
                _str,
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
        -EPERM
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.readable() {
            return -EPERM;
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
        -EPERM
    }
}

pub fn sys_open_at(dirfd: isize, path: *const u8, flags: u32, _mode: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags).unwrap();

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        process.inner_exclusive_access().cwd.clone()
    } else {
        String::from("/")
    };

    let ret = {
        if let Some(vfile) = open_file(
            cwd.as_str(),
            path.as_str(),
            flags,
        ) {
            let mut inner = process.inner_exclusive_access();
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileClass::File(vfile));
            fd as isize
        } else {
            -EPERM
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_open_at(dirfd: {}, path: {:?}, flags: {:#x?}, mode: {:#x?}) = {}",
        dirfd,
        path,
        flags,
        _mode,
        ret
    );
    ret
}

pub fn sys_close(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, -EPERM);
        return -EPERM;
    }
    if inner.fd_table[fd].is_none() {
        gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, -EPERM);
        return -EPERM;
    }
    inner.fd_table[fd].take();
    gdb_println!(SYSCALL_ENABLE, "sys_close(fd: {}) = {}", fd, 0);
    0
}

pub fn sys_pipe(pipe: *mut u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let mut inner = process.inner_exclusive_access();
    let (pipe_read, pipe_write) = make_pipe();
    let read_fd = inner.alloc_fd();
    inner.fd_table[read_fd] = Some(FileClass::Abs(pipe_read));
    let write_fd = inner.alloc_fd();
    inner.fd_table[write_fd] = Some(FileClass::Abs(pipe_write));
    *translated_refmut(token, pipe) = read_fd as u32;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd as u32;
    gdb_println!(SYSCALL_ENABLE, "sys_pipe() = [{}, {}]", read_fd, write_fd);
    0
}

pub fn sys_dup(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if inner.fd_table[fd].is_none() {
        return -EPERM;
    }
    let new_fd = inner.alloc_fd();
    inner.fd_table[new_fd] = inner.fd_table[fd].clone();
    gdb_println!(SYSCALL_ENABLE, "sys_dup(fd: {}) = {}", fd, new_fd);
    new_fd as isize
}

pub fn sys_dup3(old_fd: usize, new_fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if old_fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if inner.fd_table[old_fd].is_none() {
        return -EPERM;
    }
    while new_fd >= inner.fd_table.len() {
        inner.fd_table.push(None);
    }
    inner.fd_table[new_fd] = inner.fd_table[old_fd].clone();
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_dup3(old_fd: {}, new_fd: {}) = {}",
        old_fd,
        new_fd,
        new_fd as isize
    );
    new_fd as isize
}

fn fstat_inner(f: Arc<OSFile>, userbuf: &mut UserBuffer) -> isize {
    let mut kstat = Kstat::new();
    kstat.st_mode = {
        if f.is_dir() {
            S_IFDIR | S_IRWXU | S_IRWXG | S_IRWXO
        } else {
            S_IFREG | S_IRWXU | S_IRWXG | S_IRWXO
        }
    };
    kstat.st_ino = f.inode_id() as u64;
    kstat.st_size = f.file_size() as i64;
    userbuf.write(kstat.as_bytes());
    0
}

/// 将文件描述符为fd的文件信息填入buf
pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let inner = process.inner_exclusive_access();
    let cwd = inner.cwd.clone();
    let mut userbuf = UserBuffer::new(buf_vec);

    let ret = if fd == AT_FDCWD {
        fstat_inner(
            open_file(&cwd, "", OpenFlags::RDONLY).unwrap(),
            &mut userbuf,
        )
    } else if fd < 0 || fd >= inner.fd_table.len() as isize {
        -EPERM
    } else {
        if let Some(file) = inner.fd_table[fd as usize].clone() {
            match file {
                FileClass::File(f) => fstat_inner(f, &mut userbuf),
                _ => -EPERM,
            }
        } else {
            -EPERM
        }
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_fstat(fd: {}, buf: {:#x?}) = {}",
        fd,
        buf,
        ret
    );
    ret
}

pub fn sys_fstatat(dirfd: isize, path: *mut u8, buf: *mut u8) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);

    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let mut userbuf = UserBuffer::new(buf_vec);

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        process.inner_exclusive_access().cwd.clone()
    } else {
        String::from("/")
    };

    let ret = if let Some(osfile) = open_file(&cwd, &path, OpenFlags::RDONLY) {
        fstat_inner(osfile, &mut userbuf)
    } else {
        -ENOENT
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_fstatat(dirfd: {}, path: {:#?}, buf: {:#x?}) = {}",
        dirfd,
        path,
        buf,
        ret
    );

    ret
}

pub fn sys_getcwd(buf: *mut u8, size: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, size);
    let inner = process.inner_exclusive_access();

    let mut user_buf = UserBuffer::new(buf_vec);
    let mut cwd = inner.cwd.clone();
    cwd.push('\0');
    let cwd_str = cwd.as_str();

    let ret = unsafe {
        let cwd_buf = core::slice::from_raw_parts(cwd_str.as_ptr(), cwd_str.len());
        user_buf.write(cwd_buf) as isize
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_getcwd(buf: {:#x?}, size = {}) = {}",
        buf,
        size,
        ret
    );
    ret
}

pub fn sys_mkdirat(dirfd: isize, path: *const u8, _mode: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        process.inner_exclusive_access().cwd.clone()
    } else {
        String::from("/")
    };

    let ret = {
        if let Some(_) = open_file(
            cwd.as_str(),
            path.as_str(),
            OpenFlags::DIRECTORY | OpenFlags::RDWR | OpenFlags::CREATE,
        ) {
            0
        } else {
            -EPERM
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_mkdirat(dirfd: {}, path: {:?}, mode: {}) = {}",
        dirfd,
        path,
        _mode,
        ret
    );
    ret
}

pub fn sys_chdir(path: *const u8) -> isize {
    let process = current_process();
    let token = current_user_token();
    let path = translated_str(token, path);
    let mut inner = process.inner_exclusive_access();

    let old_cwd = if !path.starts_with("/") {
        inner.cwd.clone()
    } else {
        String::from("/")
    };

    let ret = {
        if let Some(_) = open_file(old_cwd.as_str(), path.as_str(), OpenFlags::RDONLY) {
            if path.starts_with("/") {
                inner.cwd = path.clone();
            } else {
                assert!(old_cwd.ends_with("/"));
                let pathv = path2vec(&path);
                let mut cwdv: Vec<_> = path2vec(&old_cwd);

                cwdv.pop();
                for &path_element in pathv.iter() {
                    if path_element == "." || path_element == "" {
                        continue;
                    } else if path_element == ".." {
                        cwdv.pop();
                    } else {
                        cwdv.push(path_element);
                    }
                }
                inner.cwd = String::from("/");
                for &cwd_element in cwdv.iter() {
                    if cwd_element != "" {
                        inner.cwd.push_str(cwd_element);
                        inner.cwd.push('/');
                    }
                }
            }
            0
        } else {
            -EPERM
        }
    };

    gdb_println!(SYSCALL_ENABLE, "sys_chdir(path: {:?}) = {}", path, ret);
    ret
}

fn getdents64_inner(f: Arc<OSFile>, userbuf: &mut UserBuffer, len: usize) -> isize {
    let mut offset = f.offset();
    let mut nread = 0;
    let mut dentry_buf = Vec::<u8>::new();
    loop {
        if let Some((mut name, new_offset, first_cluster, attribute)) = f.dirent_info(offset) {
            name.push('\0');
            let reclen = core::mem::size_of::<FSDirent>() + name.len();
            if nread + reclen > len {
                break;
            }
            let fs_dirent = FSDirent::new(
                first_cluster as u64,
                DIRENT_SZ as i64,
                reclen as u16,
                DType::from_attribute(attribute) as u8,
            );
            dentry_buf.extend_from_slice(fs_dirent.as_bytes());
            dentry_buf.extend_from_slice(name.as_bytes());
            nread += reclen;
            offset = new_offset as usize;
        } else {
            break;
        }
        offset += DIRENT_SZ;
    }
    userbuf.write(dentry_buf.as_slice());
    f.set_offset(offset);
    nread as isize
}

pub fn sys_getdents64(fd: isize, buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, len);
    let inner = process.inner_exclusive_access();
    let cwd = inner.cwd.clone();

    let mut userbuf = UserBuffer::new(buf_vec);

    let ret = if fd == AT_FDCWD {
        getdents64_inner(
            open_file(&cwd, "", OpenFlags::RDONLY).unwrap(),
            &mut userbuf,
            len,
        )
    } else if fd < 0 || fd >= inner.fd_table.len() as isize {
        -EPERM
    } else {
        if let Some(file) = inner.fd_table[fd as usize].clone() {
            match file {
                FileClass::File(f) => getdents64_inner(f, &mut userbuf, len),
                _ => -EPERM,
            }
        } else {
            -EPERM
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_getdents64(fd: {}, buf = {:x?}, len: {}) = {}",
        fd,
        buf,
        len,
        ret
    );

    ret
}

pub fn sys_mount(
    _p_special: *const u8,
    _p_dir: *const u8,
    _p_fstype: *const u8,
    _flags: usize,
    _data: *const u8,
) -> isize {
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_mount(...) = {}",
        0
    );
    0
}

pub fn sys_umount(_p_special: *const u8, _flags: usize) -> isize {
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_umount(...) = {}",
        0
    );
    0
}

pub fn sys_unlinkat(dirfd: i32, path: *const u8, _: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.inner_exclusive_access();
    let path = translated_str(token, path);
    let mut base_path = inner.cwd.as_str();
    // 如果path是绝对路径，则dirfd被忽略
    if path.starts_with("/") {
        base_path = "/";
    } else if dirfd != AT_FDCWD as i32 {
        let dirfd = dirfd as usize;
        if dirfd >= inner.fd_table.len() {
            return -EPERM;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[dirfd] {
            if let Some(osfile) = osfile.find(path.as_str(), OpenFlags::empty()) {
                osfile.remove();
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_unlinkat(dirfd = {}, path = {:#?}) = {}",
                    dirfd, path, 0
                );
                return 0;
            }
        }
        return -EPERM;
    }
    if let Some(osfile) = open_file(base_path, path.as_str(), OpenFlags::empty()) {
        osfile.remove();
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_unlinkat(dirfd = {}, path = {:#?}) = {}",
            dirfd, path, 0
        );
        return 0;
    }
    return -ENOENT;
}

pub fn sys_ioctl() -> isize {
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_ioctl(...) = 0"
    );
    0
}

pub fn sys_fcntl() -> isize {
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_fcntl(...) = 0"
    );
    0
}

pub fn sys_writev(fd: usize, iov: *mut IOVec, iocnt: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();

    let mut ret = 0isize;

    if fd >= inner.fd_table.len() {
        return -EPERM;
    }

    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.writable() {
            return -EPERM;
        }

        for i in 0..iocnt {
            let iovec = translated_ref(token, unsafe { iov.add(i) });
            let buf = translated_byte_buffer(token, iovec.iov_base, iovec.iov_len);
            ret += f.write(UserBuffer::new(buf)) as isize;
        }
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_writev(fd: {}, iov = {:x?}, iocnt: {}) = {}",
        fd,
        iov,
        iocnt,
        ret
    );

    ret
}
