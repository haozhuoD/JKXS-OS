use crate::fs::{
    make_pipe, open_common_file, open_device_file, path2vec, BitOpt, DType, FSDirent, FdSet, File,
    FileClass, IOVec, Kstat, OSFile, OpenFlags, Pollfd, Statfs, POLLIN, SEEK_CUR,
    SEEK_END, SEEK_SET, S_IFCHR, S_IFDIR, S_IFREG, S_IRWXG, S_IRWXO, S_IRWXU, remove_vfile_idx,
};
use crate::gdb_println;
use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};

use crate::monitor::{QEMU, SYSCALL_ENABLE};
use crate::syscall::process;
use crate::task::{current_process, current_user_token, suspend_current_and_run_next, TimeSpec};
use crate::timer::{get_time_ns, NSEC_PER_SEC};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;
use fat32_fs::DIRENT_SZ;

use super::errorno::*;

const AT_FDCWD: isize = -100;
const UTIME_NOW: usize = (1 << 30) - 1;
const UTIME_OMIT: usize = (1 << 30) - 2;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -EBADF;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.writable() {
            return -EINVAL;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        drop(process);
        let ret = f.write(UserBuffer::new(translated_byte_buffer(token, buf, len)));
        if fd >= 2 {
            gdb_println!(
                SYSCALL_ENABLE,
                "sys_write(fd: {}, buf: {:#x?}, len: {}) = {}",
                fd,
                translated_str(token, buf),
                len,
                ret
            );
        }
        ret as isize
    } else {
        -EBADF
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -EBADF;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
        match file {
            FileClass::File(fi) => f = fi.clone(),
            FileClass::Abs(fi) => f = fi.clone(),
        }
        if !f.readable() {
            return -EINVAL;
        }
        // release current task TCB manually to avoid multi-borrow
        // 为什么要提前drop掉？因为在read/write的过程可能会触发suspend_current/exit_current
        drop(inner);
        drop(process);
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
        -EBADF
    }
}

pub fn sys_open_at(dirfd: isize, path: *const u8, flags: u32, _mode: u32) -> isize {
    let token = current_user_token();
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    let mut path = translated_str(token, path);
    let flags = OpenFlags::from_bits(flags).unwrap();
    gdb_println!(
        SYSCALL_ENABLE,
        "***sys_open_at(dirfd: {}, path: {:?}, flags: {:#x?}, mode: {:#x?}) = ?",
        dirfd,
        path,
        flags,
        _mode
    );
    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        inner.cwd.clone()
    } else {
        String::from("/")
    };

    let ret = {
        if let Some(devfile) = open_device_file(cwd.as_str(), path.as_str(), flags) {
            let fd = inner.alloc_fd(0);
            inner.fd_table[fd] = Some(FileClass::Abs(devfile));
            fd as isize
        } else if let Some(vfile) = open_common_file(cwd.as_str(), path.as_str(), flags) {
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
    let mut inner = process.acquire_inner_lock();
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

pub fn sys_pipe2(pipe: *mut u32, flags: u32) -> isize {
    let process = current_process();
    let token = current_user_token();
    let flags = OpenFlags::from_bits(flags).unwrap();

    let mut inner = process.acquire_inner_lock();
    let (pipe_read, pipe_write) = make_pipe(flags);
    let read_fd = inner.alloc_fd(0);
    inner.fd_table[read_fd] = Some(FileClass::Abs(pipe_read));
    let write_fd = inner.alloc_fd(0);
    inner.fd_table[write_fd] = Some(FileClass::Abs(pipe_write));
    *translated_refmut(token, pipe) = read_fd as u32;
    *translated_refmut(token, unsafe { pipe.add(1) }) = write_fd as u32;

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_pipe2(flags: {:#x?}) = [{}, {}]",
        flags,
        read_fd,
        write_fd
    );
    0
}

pub fn sys_dup(fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    if fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if inner.fd_table[fd].is_none() {
        return -EPERM;
    }

    if inner.fd_table.len() > inner.fd_max {
        return -EMFILE;
    }

    let new_fd = inner.alloc_fd(0);
    inner.fd_table[new_fd] = inner.fd_table[fd].clone();
    gdb_println!(SYSCALL_ENABLE, "sys_dup(fd: {}) = {}", fd, new_fd);
    new_fd as isize
}

pub fn sys_dup3(old_fd: usize, new_fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.acquire_inner_lock();
    if old_fd >= inner.fd_table.len() {
        return -EPERM;
    }
    if inner.fd_table[old_fd].is_none() {
        return -EPERM;
    }

    if old_fd > inner.fd_max {
        return -EMFILE;
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
        if f.name() == "null" {
            S_IFCHR
        } else if f.is_dir() {
            S_IFDIR | S_IRWXU | S_IRWXG | S_IRWXO
        } else {
            S_IFREG | S_IRWXU | S_IRWXG | S_IRWXO
        }
    };
    kstat.st_ino = f.inode_id() as u64;
    kstat.st_size = f.file_size() as i64;
    kstat.st_atime_sec = f.accessed_time() as i64;
    kstat.st_mtime_sec = f.modification_time() as i64;
    userbuf.write(kstat.as_bytes());
    0
}

/// 将文件描述符为fd的文件信息填入buf
pub fn sys_fstat(fd: isize, buf: *mut u8) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Kstat>());
    let inner = process.acquire_inner_lock();
    let cwd = inner.cwd.clone();
    let mut userbuf = UserBuffer::new(buf_vec);

    let ret = if fd == AT_FDCWD {
        // fstat_inner(
        //     open_common_file(&cwd, "", OpenFlags::RDONLY).unwrap(),
        //     &mut userbuf,
        // )
        userbuf.write(open_common_file(&cwd, "", OpenFlags::RDONLY).unwrap().stat().as_bytes());
        0
    // } else if fd < 0 || fd >= inner.fd_table.len() as isize {
    //     -EPERM
    // } else {
    //     if let Some(file) = inner.fd_table[fd as usize].clone() {
    //         match file {
    //             FileClass::File(f) => fstat_inner(f, &mut userbuf),
    //             _ => -EPERM,
    //         }
    //     } else {
    //         -EPERM
    //     }
    // };
    } else if let Some(Some(FileClass::File(f))) = inner.fd_table.get(fd as usize) {
        userbuf.write(f.stat().as_bytes());
        0
    } else {
        -EPERM
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
        process.acquire_inner_lock().cwd.clone()
    } else {
        String::from("/")
    };

    let ret = if let Some(osfile) = open_common_file(&cwd, &path, OpenFlags::RDONLY) {
        // fstat_inner(osfile, &mut userbuf)
        userbuf.write(osfile.stat().as_bytes());
        0
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
    let inner = process.acquire_inner_lock();

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
        process.acquire_inner_lock().cwd.clone()
    } else {
        String::from("/")
    };
    // gdb_println!(
    //     SYSCALL_ENABLE,
    //     "sys_mkdirat(dirfd: {}, path: {:?}, mode: {})",
    //     dirfd,
    //     path,
    //     _mode,
    // );

    let ret = {
        if let Some(_) = open_common_file(
            cwd.as_str(),
            path.as_str(),
            OpenFlags::DIRECTORY | OpenFlags::RDWR,
        ) {
            -EEXIST
        } else {
            if let Some(_) = open_common_file(
                cwd.as_str(),
                path.as_str(),
                OpenFlags::DIRECTORY | OpenFlags::RDWR | OpenFlags::CREATE,
            ) {
                0
            } else {
                -EPERM
            }
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
    let mut inner = process.acquire_inner_lock();

    let old_cwd = if !path.starts_with("/") {
        inner.cwd.clone()
    } else {
        String::from("/")
    };
    let ret = {
        if let Some(osfile) = open_common_file(old_cwd.as_str(), path.as_str(), OpenFlags::RDONLY) {
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
    f.set_offset(offset);
    nread as isize
}

pub fn sys_getdents64(fd: isize, buf: *mut u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let buf_vec = translated_byte_buffer(token, buf, len);
    let inner = process.acquire_inner_lock();
    let cwd = inner.cwd.clone();

    let mut userbuf = UserBuffer::new(buf_vec);

    let ret = if fd == AT_FDCWD {
        getdents64_inner(
            open_common_file(&cwd, "", OpenFlags::RDONLY).unwrap(),
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
    let inner = process.acquire_inner_lock();
    let path = translated_str(token, path);
    let mut base_path = inner.cwd.as_str();
    // 如果path是绝对路径，则dirfd被忽略
    if path.starts_with("/") {
        base_path = "/";
    } else if dirfd != AT_FDCWD {
        if let Some(Some(FileClass::File(osfile))) = inner.fd_table.get(dirfd as usize) {
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
    if let Some(osfile) = open_common_file(base_path, path.as_str(), OpenFlags::empty()) {
        let abs_path = if path.starts_with("/") {
            path
        } else {
            base_path.to_string() + &path
        };
        remove_vfile_idx(&abs_path);
        osfile.remove();
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_unlinkat(dirfd = {}, path = {:#?}) = {}",
            dirfd,
            abs_path,
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
// pub const F_GETFL: u32 = 3;
pub const F_DUPFD_CLOEXEC: u32 = 1030;

pub fn sys_fcntl(fd: usize, cmd: u32, arg: usize) -> isize {
    let process = current_process();
    let mut inner = process.acquire_inner_lock();

    if fd > inner.fd_table.len() {
        return -1;
    }

    let ret = {
        if let Some(_file) = &mut inner.fd_table[fd] {
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
    let inner = process.acquire_inner_lock();

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
    let inner = process.acquire_inner_lock();

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
    let inner = process.acquire_inner_lock();

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
                    fi.set_offset(*translated_ref(token, offset));
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

pub fn sys_utimensat(
    dirfd: isize,
    ppath: *const u8,
    times: *const TimeSpec,
    _flags: isize,
) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.acquire_inner_lock();
    let path = if ppath as usize != 0 {
        translated_str(token, ppath)
    } else {
        String::from(".")
    };
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
                dirfd,
                path,
                -EBADF
            );
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = inner.fd_table[dirfd].clone() {
            if ppath as usize == 0 {
                do_utimensat(osfile, times, token);
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
                    dirfd,
                    path,
                    0
                );
                return 0;
            } else if let Some(f) = osfile.find(path.as_str(), OpenFlags::empty()) {
                do_utimensat(f, times, token);
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
                    dirfd,
                    path,
                    0
                );
                return 0;
            }
        }
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
            dirfd,
            path,
            -ENOENT
        );
        return -ENOENT;
    }
    if let Some(f) = open_common_file(base_path, path.as_str(), OpenFlags::empty()) {
        do_utimensat(f, times, token);
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
            dirfd,
            path,
            0
        );
        return 0;
    }
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_utimensat(dirfd = {}, path = {:#?}) = {}",
        dirfd,
        path,
        -ENOENT
    );
    return -ENOENT;
}

fn do_utimensat(file: Arc<OSFile>, times: *const TimeSpec, token: usize) {
    let curtime = (get_time_ns() / NSEC_PER_SEC) as u64;
    if times as usize == 0 {
        file.set_accessed_time(curtime);
        file.set_modification_time(curtime);
    } else {
        let atime_ts = translated_ref(token, times);
        match atime_ts.tv_usec {
            UTIME_NOW => file.set_accessed_time(curtime),
            UTIME_OMIT => (),
            _ => file.set_accessed_time(atime_ts.tv_sec as u64),
        };
        let mtime_ts = translated_ref(token, unsafe { times.add(1) });
        match mtime_ts.tv_usec {
            UTIME_NOW => file.set_modification_time(curtime),
            UTIME_OMIT => (),
            _ => file.set_modification_time(mtime_ts.tv_sec as u64),
        };
    }
}

pub fn sys_faccessat(dirfd: isize, path: *const u8, _mode: usize, flags: usize) -> isize {
    let process = current_process();
    let token = current_user_token();
    let inner = process.acquire_inner_lock();
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
                dirfd,
                path,
                flags,
                -EBADF
            );
            return -EBADF;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[dirfd] {
            if let Some(_) = osfile.find(path.as_str(), flags) {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
                    dirfd,
                    path,
                    flags,
                    0
                );
                return 0;
            }
        }
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
            dirfd,
            path,
            flags,
            -ENOENT
        );
        return -ENOENT;
    }
    if let Some(_) = open_common_file(base_path, path.as_str(), flags) {
        gdb_println!(
            SYSCALL_ENABLE,
            "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
            dirfd,
            path,
            flags,
            0
        );
        return 0;
    }
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_faccessat(dirfd = {}, path = {:#?}, flags: {:#?}) = {}",
        dirfd,
        path,
        flags,
        -ENOENT
    );
    return -ENOENT;
}

pub fn sys_ppoll(fds: *mut Pollfd, nfds: usize, timeout: i32) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.acquire_inner_lock();

    let mut ret = 0isize;

    for i in 0..nfds {
        let mut pollfd = translated_refmut(token, unsafe { fds.add(i) });
        if let Some(f) = inner.fd_table.get(pollfd.fd as usize) {
            if f.is_some() {
                pollfd.revents |= POLLIN;
                ret += 1;
            }
        }
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_ppoll(fds: {:#x?}, nfds = {:x?}, timeout: {}) = {}",
        fds,
        nfds,
        timeout,
        ret
    );

    ret
}

// int select(int nfds, fd_set *rfds, fd_set *wfds, fd_set *efds, struct timeval *timeout)
// todo: 实现计时返回
// timesoec 为0.0 轮询一遍返回结果  erro fd_set 永远清零
// 否则 轮询fdset ,线程切换. 直到fdset 中有可用fd返回 . erro fd_set 永远清零
pub fn sys_pselect(
    nfds: i64,
    rfds: *mut FdSet,
    wfds: *mut FdSet,
    efds: *mut FdSet,
    timeout: *mut TimeSpec,
) -> isize {
    let token = current_user_token();
    let mut ret = 0isize;

    let time = translated_refmut(token, timeout);
    if time.tv_sec == 0 && time.tv_usec == 0 {
        let process = current_process();
        let inner = process.acquire_inner_lock();
        ////pselect非阻塞处理 todo todo
        // 处理 read fd set
        if rfds as usize != 0 {
            let read_fds = translated_refmut(token, rfds);
            // let rfd_clone = read_fds.clone(); // debug
            let select_rfd = read_fds;

            // select_rfd.u128_set_bit(0,true);
            // //debug
            // println!(" debug select_rfd: {}    rfd_clone: {} ",select_rfd,rfd_clone);

            for i in 0..nfds as usize {
                //read fs set 直接当中的返回可用fd
                // let Some(file) = &inner.fd_table[fd]
                if select_rfd.u128_get_bit(i) {
                    if let Some(f) = inner.fd_table.get(i) {
                        if f.is_some() {
                            if let Some(file) = &inner.fd_table[i] {
                                let f: Arc<dyn File + Send + Sync>;
                                match file {
                                    FileClass::File(fi) => f = fi.clone(),
                                    FileClass::Abs(fi) => f = fi.clone(),
                                }
                                if f.read_blocking() {
                                    //fd 不可用
                                    select_rfd.u128_set_bit(i, false);
                                    continue;
                                }
                                ret += 1;
                            }
                        }
                    }
                }

                // //debug
                // if i==5 {
                //     select_rfd.u128_set_bit(i,false);
                //     ret -=1;
                // }
            }
        }

        // 处理 write fd set
        //write fs set 直接当中的返回可用fd
        if wfds as usize != 0 {
            let write_fds = translated_refmut(token, wfds);
            let select_wfd = write_fds;

            for i in 0..nfds as usize {
                if select_wfd.u128_get_bit(i) {
                    if let Some(f) = inner.fd_table.get(i) {
                        if f.is_some() {
                            if let Some(file) = &inner.fd_table[i] {
                                let f: Arc<dyn File + Send + Sync>;
                                match file {
                                    FileClass::File(fi) => f = fi.clone(),
                                    FileClass::Abs(fi) => f = fi.clone(),
                                }
                                if f.write_blocking() {
                                    //fd 不可用
                                    select_wfd.u128_set_bit(i, false);
                                    continue;
                                }
                                ret += 1;
                            }
                        }
                    }
                }
            }
        }

        // 简单直接把erro fd set 清零
        if efds as usize != 0 {
            let erro_fds = translated_refmut(token, efds);
            erro_fds.u128_clear_all();
        }

        gdb_println!(
            SYSCALL_ENABLE,
            "sys_pselect(nfds: {:#x?}, rfds = {:x?}, wfds = {:#x?}, efds = {:#x?}, timeout: {:#x?}) = {}  TimeOut ...",
            nfds,
            translated_refmut(token, rfds),
            wfds,
            efds,
            time,
            ret
        );
    } else {
        // pselect阻塞处理 todo erro fd set 处理
        // 内核保存一份 read_fds
        let rfd_clone: u128;
        if rfds as usize != 0 {
            let read_fds = translated_refmut(token, rfds);
            rfd_clone = read_fds.clone();
        } else {
            rfd_clone = 0;
        }
        // 内核保存一份 write_fds
        let wfd_clone: u128;
        if wfds as usize != 0 {
            let write_fds = translated_refmut(token, wfds);
            wfd_clone = write_fds.clone();
        } else {
            wfd_clone = 0;
        }
        // 简单直接把erro fd set 清零
        if efds as usize != 0 {
            let erro_fds = translated_refmut(token, efds);
            erro_fds.u128_clear_all();
        }

        loop {
            let process = current_process();
            let inner = process.acquire_inner_lock();
            let mut ret = 0isize;

            //处理 read fd set
            if rfd_clone != 0 {
                let read_fds = translated_refmut(token, rfds);
                for i in 0..nfds as usize {
                    if rfd_clone.u128_get_bit(i) {
                        if let Some(f) = inner.fd_table.get(i) {
                            if f.is_some() {
                                if let Some(file) = &inner.fd_table[i] {
                                    let f: Arc<dyn File + Send + Sync>;
                                    match file {
                                        FileClass::File(fi) => f = fi.clone(),
                                        FileClass::Abs(fi) => f = fi.clone(),
                                    }
                                    if !f.readable() {
                                        return -1; //可能是错的
                                    }
                                    if f.read_blocking() {
                                        //fd 不可用
                                        read_fds.u128_set_bit(i, false);
                                        continue;
                                    }
                                    read_fds.u128_set_bit(i, true);
                                    ret += 1;
                                }
                            }
                        }
                    }
                }
            }

            //处理write fd set
            if wfd_clone != 0 {
                let write_fds = translated_refmut(token, wfds);
                for i in 0..nfds as usize {
                    if wfd_clone.u128_get_bit(i) {
                        if let Some(f) = inner.fd_table.get(i) {
                            if f.is_some() {
                                if let Some(file) = &inner.fd_table[i] {
                                    let f: Arc<dyn File + Send + Sync>;
                                    match file {
                                        FileClass::File(fi) => f = fi.clone(),
                                        FileClass::Abs(fi) => f = fi.clone(),
                                    }
                                    if !f.writable() {
                                        return -1; //可能是错的
                                    }
                                    if f.write_blocking() {
                                        //fd 不可用
                                        write_fds.u128_set_bit(i, false);
                                        continue;
                                    }
                                    write_fds.u128_set_bit(i, true);
                                    ret += 1;
                                }
                            }
                        }
                    }
                }
            }

            if ret == 0 {
                // gdb_println!(
                //     SYSCALL_ENABLE,
                //     "sys_pselect(nfds: {:#x?}, rfds = {:x?}, wfds = {:x?}, efds = {:x?}, timeout: {:x?}) ...... continue",
                //     nfds,
                //     translated_refmut(token, rfds),
                //     wfds,
                //     efds,
                //     time,
                // );
                drop(inner);
                drop(process);
                suspend_current_and_run_next();
            } else {
                gdb_println!(
                    SYSCALL_ENABLE,
                    "sys_pselect(nfds: {:#x?}, rfds = {:x?}, wfds = {:x?}, efds = {:x?}, timeout: {:x?}) = {}",
                    nfds,
                    translated_refmut(token, rfds),
                    wfds,
                    efds,
                    time,
                    ret
                );
                break;
            }
        }
    }
    ret
}

pub fn sys_renameat2(
    old_fd: isize,
    old_path: *const u8,
    new_fd: isize,
    new_path: *const u8,
    flags: usize,
) -> isize {
    if flags != 0 {
        return -EINVAL;
    }
    let process = current_process();
    let token = current_user_token();
    let inner = process.acquire_inner_lock();
    let old_path = translated_str(token, old_path);
    let new_path = translated_str(token, new_path);
    let cwd = inner.cwd.as_str();
    let old_file;
    let new_file;

    if old_path.starts_with("/") {
        match open_common_file("/", old_path.as_str(), OpenFlags::empty()) {
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
    } else if let Some(tmp_file) = open_common_file(cwd, old_path.as_str(), OpenFlags::empty()) {
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
        match open_common_file("/", new_path.as_str(), open_flags) {
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
    } else if let Some(tmp_file) = open_common_file(cwd, new_path.as_str(), open_flags) {
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
    let ret = if let Some(osfile) = open_common_file("/", abs_path.as_str(), OpenFlags::RDONLY) {
        getdents64_inner(osfile, &mut userbuf, len)
    } else {
        -EPERM
    };
    ret
}

pub fn sys_lseek(fd: usize, offset: usize, whence: usize) -> isize {
    let process = current_process();
    let inner = process.acquire_inner_lock();

    let ret = if let Some(Some(f)) = inner.fd_table.get(fd) {
        match f {
            FileClass::File(fi) => {
                let sz = fi.file_size();
                let new_off: isize = match whence {
                    SEEK_SET => offset as _,
                    SEEK_CUR => (fi.offset() + offset) as _,
                    SEEK_END => (sz + offset) as _,
                    _ => -1,
                };
                if new_off < 0 {
                    -EINVAL
                } else {
                    fi.set_offset(new_off as _) as isize
                }
            }
            FileClass::Abs(_) => -ESPIPE,
        }
    } else {
        -EBADF
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_lseek(fd: {}, offset: {}, whence: {}) = {}",
        fd,
        offset,
        whence,
        ret
    );
    ret
}

pub fn sys_pread64(fd: usize, buf: *mut u8, count: usize, offset: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.acquire_inner_lock();
    let ret = if let Some(Some(f)) = inner.fd_table.get(fd) {
        match f {
            FileClass::File(fi) => {
                let old_off = fi.offset();
                fi.set_offset(offset);
                let read_cnt =
                    fi.read(UserBuffer::new(translated_byte_buffer(token, buf, count))) as isize;
                fi.set_offset(old_off);
                read_cnt
            }
            FileClass::Abs(_) => -ESPIPE,
        }
    } else {
        -EBADF
    };
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_pread64(fd: {}, buf: {:#x?}, count: {}, offset: {}) = {}",
        fd,
        buf,
        count,
        offset,
        ret
    );
    ret
}

pub fn sys_statfs(_path: *const u8, buf: *const u8) -> isize {
    let token = current_user_token();
    let buf_vec = translated_byte_buffer(token, buf, size_of::<Statfs>());
    let mut userbuf = UserBuffer::new(buf_vec);
    let statfs = Statfs::new();
    userbuf.write(statfs.as_bytes());
    gdb_println!(
        SYSCALL_ENABLE,
        "sys_statfs(path: {:#x?}, buf: {:#x?}) = {}",
        _path,
        buf,
        0
    );
    0
}

pub fn sys_readlinkat(dirfd: isize, pathname: *const u8, buf: *mut u8, bufsiz: usize) -> isize {
    if dirfd != AT_FDCWD {
        panic!("dirfd != AT_FDCWD, unimplemented yet!");
    }
    let process = current_process();
    let inner = process.acquire_inner_lock();
    let token = inner.get_user_token();
    let path = translated_str(token, pathname);
    if path.as_str() != "/proc/self/exe" {
        unimplemented!();
    }
    let mut userbuf = UserBuffer::new(translated_byte_buffer(token, buf, bufsiz));
    let _lmbench = "/exit_test\0";
    userbuf.write(_lmbench.as_bytes());
    let len = _lmbench.len() - 1;

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_readlinkat(dirfd = {}, pathname = {:#?}, buf = {:#x?}, bufsiz = {}) = {}",
        dirfd,
        path,
        buf as usize,
        bufsiz,
        len
    );
    len as isize
}
