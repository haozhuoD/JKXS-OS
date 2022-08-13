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
    pub sz: usize,
    read_end: Option<Weak<Pipe>>,
    write_end: Option<Weak<Pipe>>,
}

impl PipeRingBuffer {
    pub fn new() -> Self {
        Self {
            arr: [0; RING_BUFFER_SIZE],
            head: 0,
            tail: 0,
            sz: 0,
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
        let mut read_size = 0usize;
        loop {
            let mut ring = self.buffer.lock();
            if ring.sz == 0 {
                if ring.all_write_ends_closed() || self.nonblock {
                    return read_size;
                }
                drop(ring);
                // debug!("read suspend, buf.len = {}, c = {}", l, read_size);
                suspend_current_and_run_next();
                continue;
            }
            for pa in buf.into_iter() {
                if ring.sz == 0 {
                    break;
                }
                unsafe { *pa = ring.arr[ring.head]; }
                ring.head += 1;
                if ring.head == RING_BUFFER_SIZE {
                    ring.head = 0;
                }
                ring.sz -= 1;
                read_size += 1;
            }
            return read_size;
        }
    }
    fn write(&self, buf: UserBuffer) -> usize {
        assert!(self.writable());
        let l = buf.len();
        if l == 0 {
            return 0;
        }
        let mut write_size = 0usize;
        loop {
            let mut ring = self.buffer.lock();
            if ring.all_read_ends_closed() {
                return write_size;
            }
            if ring.sz == RING_BUFFER_SIZE {
                // ring buffer is full
                if self.nonblock {
                    return write_size;
                }
                drop(ring);
                // debug!("write suspend, buf.len = {}, c = {}", l, write_size);
                suspend_current_and_run_next();
                continue;
            }
            for pa in buf.into_iter() {
                if ring.sz == RING_BUFFER_SIZE {
                    break;
                }
                let tail = ring.tail;
                unsafe { ring.arr[tail] = *pa; }
                ring.tail += 1;
                if ring.tail == RING_BUFFER_SIZE {
                    ring.tail = 0;
                }
                ring.sz += 1;
                write_size += 1;
            }
            return write_size;
        }
    }
    fn read_blocking(&self) -> bool {
        if self.readable() {
            if self.nonblock {
                return false;
            } else {
                let ring_buffer = self.buffer.lock();
                if ring_buffer.sz == 0 {
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
                if ring_buffer.sz == RING_BUFFER_SIZE {
                    return true;
                } else {
                    return false;
                }
            }
        }
        false
    }
}
