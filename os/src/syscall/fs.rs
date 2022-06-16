use crate::fs::{
    make_pipe, open_file, path2vec, DType, FSDirent, File, FileClass, IOVec, Kstat, OSFile,
    OpenFlags, S_IFDIR, S_IFREG, S_IRWXG, S_IRWXO, S_IRWXU
};
use crate::gdb_println;
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};

use crate::monitor::{QEMU, SYSCALL_ENABLE};
use crate::task::{current_process, current_user_token};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;
use fat32_fs::DIRENT_SZ;

use super::errorno::{EPERM, ENOENT, EBADF, ENOTDIR, EINVAL};

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
        if let Some(vfile) = open_file(cwd.as_str(), path.as_str(), flags) {
            let mut inner = process.inner_exclusive_access();
            let fd = inner.alloc_fd(0);
            inner.fd_table[fd] = Some(FileClass::File(vfile));
            fd as isize
        } else {
            -ENOENT
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
    let read_fd = inner.alloc_fd(0);
    inner.fd_table[read_fd] = Some(FileClass::Abs(pipe_read));
    let write_fd = inner.alloc_fd(0);
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
    let new_fd = inner.alloc_fd(0);
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
    let mut path = translated_str(token, path);
    let mut inner = process.inner_exclusive_access();
    let old_cwd = if !path.starts_with("/") {
        inner.cwd.clone()
    } else {
        String::from("/")
    };
    let ret = {
        if let Some(osfile) = open_file(old_cwd.as_str(), path.as_str(), OpenFlags::RDONLY) {
            if osfile.is_dir() {
                if path.starts_with("/") {
                    if !path.ends_with("/") {
                        path.push('/');
                    }
                    inner.cwd = path.clone();
                } else {
                    assert!(old_cwd.ends_with("/"));
                    let pathv = path2vec(&path);
                    let mut cwdv: Vec<_> = path2vec(&old_cwd);

                    // cwdv.pop();
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
                -ENOTDIR
            }
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
    f.seek(offset);
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
    gdb_println!(SYSCALL_ENABLE, "sys_mount(...) = {}", 0);
    0
}

pub fn sys_umount(_p_special: *const u8, _flags: usize) -> isize {
    gdb_println!(SYSCALL_ENABLE, "sys_umount(...) = {}", 0);
    0
}

pub fn sys_unlinkat(dirfd: isize, path: *const u8, _: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.inner_exclusive_access();
    let path = translated_str(token, path);
    let mut base_path = inner.cwd.as_str();
    // 如果path是绝对路径，则dirfd被忽略
    if path.starts_with("/") {
        base_path = "/";
    } else if dirfd != AT_FDCWD {
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
                    dirfd,
                    path,
                    0
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
            dirfd,
            path,
            0
        );
        return 0;
    }
    return -ENOENT;
}

pub fn sys_ioctl() -> isize {
    gdb_println!(SYSCALL_ENABLE, "sys_ioctl(...) = 0");
    0
}

pub const F_DUPFD: u32 = 0;
pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_DUPFD_CLOEXEC: u32 = 1030;

pub fn sys_fcntl(fd: usize, cmd: u32, arg: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();

    if fd > inner.fd_table.len() {
        return -1;
    }

    let ret = {
        if let Some(file) = &mut inner.fd_table[fd] {
            match cmd {
                F_DUPFD_CLOEXEC | F_DUPFD => {
                    let new_fd = inner.alloc_fd(arg);
                    inner.fd_table[new_fd] = inner.fd_table[fd].clone();
                    new_fd as isize
                }
                F_GETFD | F_SETFD => 0,
                _ => 0, // WARNING!!!
            }
        } else {
            -1
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_fcntl(fd = {}, cmd = {}, arg = {}) = {}",
        fd,
        cmd,
        arg,
        ret
    );

    ret
}

pub fn sys_readv(fd: usize, iov: *mut IOVec, iocnt: usize) -> isize {
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
        if !f.readable() {
            return -EPERM;
        }
        for i in 0..iocnt {
            let iovec = translated_ref(token, unsafe { iov.add(i) });
            let buf = translated_byte_buffer(token, iovec.iov_base, iovec.iov_len);
            ret += f.read(UserBuffer::new(buf)) as isize;
        }
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_readv(fd: {}, iov = {:x?}, iocnt: {}) = {}",
        fd,
        iov,
        iocnt,
        ret
    );

    ret
}

pub fn sys_writev(fd: usize, iov: *const IOVec, iocnt: usize) -> isize {
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

pub fn sys_sendfile(out_fd: usize, in_fd: usize, offset: *mut usize, count: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();

    let fout = inner.fd_table.get(out_fd).unwrap_or(&None);
    let fin = inner.fd_table.get(in_fd).unwrap_or(&None);

    if fin.is_none() || fout.is_none() {
        return -EPERM;
    } else {
        let fin = fin.clone().unwrap();
        let fin_inner: Arc<dyn File + Send + Sync>;
        match fin {
            FileClass::File(fi) => {
                if offset as usize != 0 {
                    fi.seek(*translated_ref(token, offset));
                };

                fin_inner = fi.clone();
            }
            _ => return -EPERM,
        }
        if !fin_inner.readable() {
            return -EPERM;
        }

        let fout = fout.clone().unwrap();
        let fout_inner: Arc<dyn File + Send + Sync>;
        match fout {
            FileClass::File(fi) => fout_inner = fi.clone(),
            FileClass::Abs(fi) => fout_inner = fi.clone(),
        }
        if !fout_inner.writable() {
            return -EPERM;
        }

        // sendfile
        let mut buf = vec![0u8; count];
        let userbuf_read = UserBuffer::new(vec![unsafe {
            core::slice::from_raw_parts_mut(buf.as_mut_slice().as_mut_ptr(), count)
        }]);
        let read_cnt = fin_inner.read(userbuf_read);

        let userbuf_write = UserBuffer::new(vec![unsafe {
            core::slice::from_raw_parts_mut(buf.as_mut_slice().as_mut_ptr(), read_cnt)
        }]);
        let ret = fout_inner.write(userbuf_write) as isize;
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_sendfile(out_fd = {}, in_fd = {}, offset = {:#x?}, count = {}) = {}",
            out_fd,
            in_fd,
            offset,
            count,
            ret
        );
        ret
    }
}

pub fn sys_utimensat(dirfd: isize, path: *const u8, _times: usize, _flags: isize) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.inner_exclusive_access();
    let path = translated_str(token, path);
    let mut base_path = inner.cwd.as_str();
    // 如果path是绝对路径，则dirfd被忽略
    if path.starts_with("/") {
        base_path = "/";
    } else if dirfd != AT_FDCWD {
        let dirfd = dirfd as usize;
        if dirfd >= inner.fd_table.len() {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
                dirfd, path, -EBADF
            );
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[dirfd] {
            if let Some(_) = osfile.find(path.as_str(), OpenFlags::empty()) {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
                    dirfd, path, 0
                );
                return 0;
            }
        }
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
            dirfd, path, -ENOENT
        );
        return -ENOENT;
    }
    if let Some(_) = open_file(base_path, path.as_str(), OpenFlags::empty()) {
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
            dirfd, path, 0
        );
        return 0;
    }
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
        dirfd, path, -ENOENT
    );
    return -ENOENT;
}

pub fn sys_faccessat(dirfd: isize, path: *const u8, _mode: usize, flags: usize) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.inner_exclusive_access();
    let path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags as u32).unwrap();
    let mut base_path = inner.cwd.as_str();
    if path.starts_with("/") {
        base_path = "/";
    } else if dirfd != AT_FDCWD {
        let dirfd = dirfd as usize;
        if dirfd >= inner.fd_table.len() {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
                dirfd, path, flags, -EBADF
            );
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[dirfd] {
            if let Some(_) = osfile.find(path.as_str(), flags) {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
                    dirfd, path, flags, 0
                );
                return 0;
            }
        }
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
            dirfd, path, flags, -ENOENT
        );
        return -ENOENT;
    }
    if let Some(_) = open_file(base_path, path.as_str(), flags) {
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
            dirfd, path, flags, 0
        );
        return 0;
    }
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
        dirfd, path, flags, -ENOENT
    );
    return -ENOENT;
}

// todo,存在bug
pub fn sys_renameat2(old_fd: isize, old_path: *const u8, new_fd: isize, new_path: *const u8, flags: usize) -> isize {
    if flags != 0 {
        return -EINVAL;
    }   
    let process = current_process();
    let token = current_user_token();
    let inner = process.inner_exclusive_access();
    let old_path = translated_str(token, old_path);
    let new_path = translated_str(token, new_path);
    let cwd = inner.cwd.as_str();
    let old_file;
    let new_file;

    if old_path.starts_with("/") {
        match open_file("/", old_path.as_str(), OpenFlags::empty()) {
            Some(tmp_file) => old_file = tmp_file.clone(),
            None => return -ENOENT,
        }
    } else if old_fd != AT_FDCWD {
        let old_fd = old_fd as usize;
        if old_fd >= inner.fd_table.len() {
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[old_fd] {
            match osfile.find(old_path.as_str(), OpenFlags::empty()) {
                Some(tmp_file) => old_file = tmp_file.clone(),
                None => return -ENOENT,
            }
        } else {
            return -ENOENT;
        }
    } else if let Some(tmp_file) = open_file(cwd, old_path.as_str(), OpenFlags::empty()) {
        old_file = tmp_file.clone();
    } else {
        return -ENOENT;  
    }  

    let open_flags = {
        if old_file.is_dir() {
            OpenFlags::CREATE | OpenFlags::RDWR | OpenFlags::DIRECTORY
        } else {
            OpenFlags::CREATE | OpenFlags::RDWR
        }
    };

    if new_path.starts_with("/") {
        match open_file("/", new_path.as_str(), open_flags) {
            Some(tmp_file) => new_file = tmp_file.clone(),
            None => return -ENOENT,
        }
    } else if new_fd != AT_FDCWD {
        let new_fd = new_fd as usize;
        if new_fd >= inner.fd_table.len() {
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[new_fd] {
            match osfile.find(new_path.as_str(), open_flags) {
                Some(tmp_file) => new_file = tmp_file.clone(),
                None => return -ENOENT,
            }
        } else {
            return -ENOENT;
        }
    } else if let Some(tmp_file) = open_file(cwd, new_path.as_str(), open_flags) {
        new_file = tmp_file.clone();
    } else {
        return -ENOENT;  
    }
    let old_ino = old_file.inode_id();
    let new_ino = new_file.inode_id();
    if old_ino == new_ino {
        return -EPERM;
    }
    new_file.set_file_size(old_file.file_size() as u32);
    new_file.set_inode_id(old_ino);
    old_file.delete();
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_renameat2(old_fd = {}, old_path = {:#?}, new_fd = {}, new_path = {}, flags: {:#?}) = {}",
        old_fd, old_path, new_fd, new_path, flags, 0
    );
    return 0;
}

pub fn sys_readdir(abs_path: *const u8, buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, len);
    let abs_path = translated_str(token, abs_path);
    let mut userbuf = UserBuffer::new(buf_vec);
    let ret = if let Some(osfile) = open_file("/", abs_path.as_str(), OpenFlags::RDONLY) {
        getdents64_inner(osfile, &mut userbuf, len)
    } else {
        -EPERM
    };
    ret
}
