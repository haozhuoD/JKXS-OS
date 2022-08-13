use super::{File, OpenFlags};
use crate::{mm::UserBuffer, syscall::EPIPE};

use alloc::sync::{Arc, Weak};
use spin::Mutex;

use crate::task::suspend_current_and_run_next;

pub struct Pipe {
    readable: bool,
    writable: bool,
    nonblock: bool,
    buffer: Arc<Mutex<PipeRingBuffer>>,
}

impl Pipe {
    pub fn read_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>, nonblock: bool) -> Self {
        Self {
            readable: true,
            writable: false,
            nonblock,
            buffer,
        }
    }
    pub fn write_end_with_buffer(buffer: Arc<Mutex<PipeRingBuffer>>, nonblock: bool) -> Self {
        Self {
            readable: false,
            writable: true,
            nonblock,
            buffer,
        }
    }
}

const RING_BUFFER_SIZE: usize = 0x20000;

#[derive(Copy, Clone, PartialEq)]
enum RingBufferStatus {
    Full,
    Empty,
    Normal,
}

pub struct PipeRingBuffer {
    arr: [u8; RING_BUFFER_SIZE],
    head: usize,
    tail: usize,
    status: RingBufferStatus,
    read_end: Option<Weak<Pipe>>,
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            status: RingBufferStatus::Empty,
            read_end: None,
            write_end: None,
        }
    }
    pub fn set_read_end(&mut self, read_end: &Arc<Pipe>) {
        self.read_end = Some(Arc::downgrade(read_end));
    }
    pub fn set_write_end(&mut self, write_end: &Arc<Pipe>) {
        self.write_end = Some(Arc::downgrade(write_end));
    }
    pub fn write_byte(&mut self, byte: u8) {
        self.status = RingBufferStatus::Normal;
        self.arr[self.tail] = byte;
        self.tail = (self.tail + 1) % RING_BUFFER_SIZE;
        if self.tail == self.head {
            self.status = RingBufferStatus::Full;
        }
    }
    pub fn read_byte(&mut self) -> u8 {
        self.status = RingBufferStatus::Normal;
        let c = self.arr[self.head];
        self.head = (self.head + 1) % RING_BUFFER_SIZE;
        if self.head == self.tail {
            self.status = RingBufferStatus::Empty;
        }
        c
    }
    pub fn available_read(&self) -> usize {
        if self.status == RingBufferStatus::Empty {
            0
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.tail + RING_BUFFER_SIZE - self.head
        }
    }
    pub fn available_write(&self) -> usize {
        if self.status == RingBufferStatus::Full {
            0
        } else {
            RING_BUFFER_SIZE - self.available_read()
        }
    }
    pub fn all_write_ends_closed(&self) -> bool {
        self.write_end.as_ref().unwrap().upgrade().is_none()
    }
    pub fn all_read_ends_closed(&self) -> bool {
        self.read_end.as_ref().unwrap().upgrade().is_none()
    }
}

/// Return (read_end, write_end)
pub fn make_pipe(flags: OpenFlags) -> (Arc<Pipe>, Arc<Pipe>) {
    let buffer = Arc::new(Mutex::new(PipeRingBuffer::new()));
    let nonblock = flags.contains(OpenFlags::NONBLOCK);
    let read_end = Arc::new(Pipe::read_end_with_buffer(buffer.clone(), nonblock));
    let write_end = Arc::new(Pipe::write_end_with_buffer(buffer.clone(), nonblock));
    buffer.lock().set_read_end(&read_end);
    buffer.lock().set_write_end(&write_end);
    (read_end, write_end)
}

impl File for Pipe {
    fn readable(&self) -> bool {
        self.readable
    }
    fn writable(&self) -> bool {
        self.writable
    }
    fn read(&self, buf: UserBuffer) -> usize {
        assert!(self.readable());
        let l = buf.len();
        let mut buf_iter = buf.into_iter();
        let mut read_size = 0usize;
        loop {
            let mut ring_buffer_lock = self.buffer.lock();
            let available = ring_buffer_lock.available_read();
            if available == 0 {
                if ring_buffer_lock.all_write_ends_closed() || read_size > 0 || self.nonblock {
                    return read_size;
                }
                drop(ring_buffer_lock);
                // debug!("read suspend, buf.len = {}, c = {}", l, read_size);
                suspend_current_and_run_next();
                continue;
            }
            // read at most loop_read bytes
            for _ in 0..available {
                if let Some(byte_ref) = buf_iter.next() {
                    unsafe {
                        *byte_ref = ring_buffer_lock.read_byte();
                    }
                    read_size += 1;
                } else {
                    return read_size;
                }
            }
        }
    }
    fn write(&self, buf: UserBuffer) -> usize {
        assert!(self.writable());
        let l = buf.len();
        if buf.len() == 0 {
            return 0;
        }
        let mut buf_iter = buf.into_iter();
        let mut write_size = 0usize;
        loop {
            let mut ring_buffer_lock = self.buffer.lock();
            if ring_buffer_lock.all_read_ends_closed() {
                return EPIPE as usize;
            }
            let available = ring_buffer_lock.available_write();
            if available == 0 {
                // ring buffer is full
                if self.nonblock {
                    return write_size;
                }
                drop(ring_buffer_lock);
                // debug!("write suspend, buf.len = {}, c = {}", l, write_size);
                suspend_current_and_run_next();
                continue;
            }
            // write at most loop_write bytes
            for _ in 0..available {
                if let Some(byte_ref) = buf_iter.next() {
                    ring_buffer_lock.write_byte(unsafe { *byte_ref });
                    write_size += 1;
                } else {
                    return write_size;
                }
            }
            if write_size == l {
                return write_size;
            }
        }
    }
    fn read_blocking(&self) -> bool {
        if self.readable() {
            if self.nonblock {
                return false;
            } else {
                let ring_buffer = self.buffer.lock();
                if ring_buffer.available_read() == 0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
    fn write_blocking(&self) -> bool {
        if self.writable() {
            if self.nonblock {
                return false;
            } else {
                let ring_buffer = self.buffer.lock();
                if ring_buffer.available_write() == 0 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
}
