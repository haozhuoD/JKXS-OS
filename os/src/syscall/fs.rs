use crate::fs::{
    make_pipe, open_file, path2vec, DType, FSDirent, File, FileClass, Kstat, OSFile, OpenFlags,
};
use crate::gdb_println;
use crate::mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer};
use crate::monitor::*;
use crate::task::{current_process, current_user_token};
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::mem::size_of;
use fat32_fs::DIRENT_SZ;

const AT_FDCWD: isize = -100;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let process = current_process();
    let inner = process.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let f: Arc<dyn File + Send + Sync>;
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
        let f: Arc<dyn File + Send + Sync>;
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

    let cwd = if dirfd == AT_FDCWD && !path.starts_with("/") {
        process.inner_exclusive_access().cwd.clone()
    } else {
        String::from("/")
    };

    let ret = {
        if let Some(vfile) = open_file(
            cwd.as_str(),
            path.as_str(),
            OpenFlags::from_bits(flags).unwrap(),
        ) {
            let mut inner = process.inner_exclusive_access();
            let fd = inner.alloc_fd();
            inner.fd_table[fd] = Some(FileClass::File(vfile));
            fd as isize
        } else {
            -1
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_open_at(dirfd: {}, path: {:?}, flags: {:#x?}, mode: {:#x?}) = {}",
        dirfd,
        path,
        flags,
        mode,
        ret
    );
    ret
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

pub fn sys_dup3(old_fd: usize, new_fd: usize) -> isize {
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if old_fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[old_fd].is_none() {
        return -1;
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
    let mut kstat = Kstat::empty();
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
        -1
    } else {
        if let Some(file) = inner.fd_table[fd as usize].clone() {
            match file {
                FileClass::File(f) => fstat_inner(f, &mut userbuf),
                _ => -1,
            }
        } else {
            -1
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

pub fn sys_mkdirat(dirfd: isize, path: *const u8, mode: u32) -> isize {
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
            -1
        }
    };

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_mkdirat(dirfd: {}, path: {:?}, mode: {}) = {}",
        dirfd,
        path,
        mode,
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
            -1
        }
    };

    gdb_println!(SYSCALL_ENABLE, "sys_chdir(path: {:?}) = {}", path, ret);
    ret
}

fn getdents64_inner(f: Arc<OSFile>, userbuf: &mut UserBuffer, len: usize) -> isize {
    let mut offset = 0;
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
        -1
    } else {
        if let Some(file) = inner.fd_table[fd as usize].clone() {
            match file {
                FileClass::File(f) => getdents64_inner(f, &mut userbuf, len),
                _ => -1,
            }
        } else {
            -1
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
    p_special: *const u8,
    p_dir: *const u8,
    p_fstype: *const u8,
    flags: usize,
    data: *const u8,
) -> isize {
    0
}

pub fn sys_umount(p_special: *const u8, flags: usize) -> isize {
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
            return -1;
        }
        if let Some(FileClass::File(osfile)) = &inner.fd_table[dirfd] {
            if let Some(osfile) = osfile.find(path.as_str(), OpenFlags::empty()) {
                osfile.remove();
                return 0;
            }
        }
        return -1;
    }
    if let Some(osfile) = open_file(base_path, path.as_str(), OpenFlags::empty()) {
        osfile.remove();
        return 0;
    }
    return -1;
}

pub fn sys_ioctl() -> isize {
    0
}
