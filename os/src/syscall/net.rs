use crate::mm::{
    translated_byte_buffer, translated_ref, translated_refmut, translated_str, UserBuffer,
};
use crate::task::{current_user_token};
use spin::{Lazy, Mutex};
use crate::fs::PipeRingBuffer;

use crate::gdb_println;
use crate::monitor::{QEMU, SYSCALL_ENABLE};


pub static SOCKET_BUF: Lazy<Mutex<PipeRingBuffer>> =
    Lazy::new(|| Mutex::new(PipeRingBuffer::new()));

// ssize_t sendto(int sockfd, const void *buf, size_t len, int flags,
//     const struct sockaddr *dest_addr, socklen_t addrlen)
pub fn sys_sendto(sockfd : isize, buf: *const u8, len:usize , flags:isize , dest_addr:usize, addrlen:usize) -> isize {
    let token = current_user_token();
    let userbuf = UserBuffer::new(translated_byte_buffer(token, buf, len));
    // gdb_println!(
    //     SYSCALL_ENABLE,
    //     "userbuf  [{:#x?}] ",
    //     userbuf
    // );
    let mut buf_iter = userbuf.into_iter();
    let mut write_size = 0isize;

    let mut ring_buffer = SOCKET_BUF.lock();
    let loop_write = ring_buffer.available_write();
    if loop_write == 0 {
        return -1;
    }
    // write at most loop_write bytes
    for _ in 0..loop_write {
        if let Some(byte_ref) = buf_iter.next() {
            ring_buffer.write_byte(unsafe { *byte_ref });
            write_size += 1;
        } else {
            return write_size;
        }
    }

    gdb_println!(
        SYSCALL_ENABLE,
        "sys_sendto(sockfd: {:#x?}, buf = ..., len:{}, flags={}, dest_addr={}, addrlen={} = 1 ---- fake",
        sockfd,
        len,
        flags,
        dest_addr,
        addrlen
    );
    write_size
}

// ssize_t recvfrom(int sockfd, void *buf, size_t len, int flags,
//     struct sockaddr *src_addr, socklen_t *addrlen);
pub fn sys_recvfrom(sockfd : isize, buf: *const u8, len:usize , flags:isize , src_addr:usize, addrlen:usize) -> isize{
    let token = current_user_token();
    let userbuf = UserBuffer::new(translated_byte_buffer(token, buf, len));
    let mut buf_iter = userbuf.into_iter();
    let mut read_size = 0isize;

    let mut ring_buffer = SOCKET_BUF.lock();
    let loop_read = ring_buffer.available_read();
    if loop_read == 0 {
        return -1;
    }
    // read at most loop_read bytes
    for _ in 0..loop_read {
        if let Some(byte_ref) = buf_iter.next() {
            unsafe {
                *byte_ref = ring_buffer.read_byte();
            }
            read_size += 1;
        } else {
            return read_size;
        }
    }
    read_size
}