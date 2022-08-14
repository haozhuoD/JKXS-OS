use crate::config::{aligned_up, aligned_down, PAGE_SIZE};
use crate::task::current_process;

use super::{frame_alloc, FrameTracker, PhysAddr, PhysPageNum, StepByOne, VirtAddr, VirtPageNum};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use bitflags::*;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits as usize,
        }
    }
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
    pub fn readable(&self) -> bool {
        (self.flags() & PTEFlags::R) != PTEFlags::empty()
    }
    pub fn writable(&self) -> bool {
        (self.flags() & PTEFlags::W) != PTEFlags::empty()
    }
    pub fn executable(&self) -> bool {
        (self.flags() & PTEFlags::X) != PTEFlags::empty()
    }
    pub fn set_cow(&mut self) {
        (*self).bits = self.bits | (1 << 9);
        // let _ = self.flags() & (!PTEFlags::W);
    }
    pub fn set_flag(&mut self, flags: PTEFlags ) {
        let new_flags: u8 = flags.bits().clone();
        self.bits = (self.bits & 0xFFFF_FFFF_FFFF_FF00) | (new_flags as usize);
    }
    pub fn reset_cow(&mut self) {
        (*self).bits = self.bits & !(1 << 9);
    }
    pub fn is_cow(&self) -> bool {
        self.bits & (1 << 9) != 0
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Option<Vec<FrameTracker>>,
}

/// Assume that it won't oom when creating/mapping.
impl PageTable {
    pub fn new() -> Self {
        let frame = frame_alloc().unwrap();
        PageTable {
            root_ppn: frame.ppn,
            frames: Some(vec![frame]),
        }
    }
    /// Temporarily used to get arguments from user space.
    pub fn from_token(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: None,
        }
    }
    fn find_pte_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                let frame = frame_alloc().unwrap();
                // 只有第三级页表可置A D 标志位  | PTEFlags::A | PTEFlags::D
                // *pte = PageTableEntry::new(frame.ppn, PTEFlags::V );
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V | PTEFlags::A | PTEFlags::D);
                self.frames.as_mut().unwrap().push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }
    fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.indexes();
        let mut ppn = self.root_ppn;
        let mut result: Option<&mut PageTableEntry> = None;
        for (i, idx) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_pte_array()[*idx];
            if i == 2 {
                result = Some(pte);
                break;
            }
            if !pte.is_valid() {
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }
    pub fn set_pte_flags(&self, vpn: VirtPageNum, flags: PTEFlags) -> Option<&mut PageTableEntry> {
        if let Some(pte) = self.find_pte(vpn) {
            if !pte.is_valid() {
                return None;
            }
            pte.bits = usize::from(pte.ppn()) << 10
                | (flags | PTEFlags::U | PTEFlags::V | PTEFlags::A | PTEFlags::D).bits() as usize;
            Some(pte)
        } else {
            None
        }
    }
    #[allow(unused)]
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping", vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V | PTEFlags::A | PTEFlags::D);
    }
    #[allow(unused)]
    pub fn unmap(&mut self, vpn: VirtPageNum) {
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }
    #[allow(unused)]
    pub fn cow_remap(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, former_ppn: PhysPageNum) {
        let pte = self.find_pte_create(vpn).unwrap();
        *pte = PageTableEntry::new(ppn, pte.flags() | PTEFlags::W);
        // pte.reset_cow();
        pte.set_cow();
        ppn.slice_u64().copy_from_slice(former_ppn.slice_u64());
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.find_pte(vpn).map(|pte| *pte)
    }
    /// Todo: multi-borrowing problem?
    pub fn translate_vpn_with_lazycheck(&self, vpn: VirtPageNum) -> Option<PhysPageNum> {
        let re_check: _ = |vpn: VirtPageNum| match current_process()
            .acquire_inner_lock()
            .check_lazy(VirtAddr::from(vpn).into())
        {
            0 => Some(self.translate(vpn).unwrap().ppn()),
            _ => None,
        };
        match self.translate(vpn) {
            Some(pte) => match pte.is_valid() {
                true => Some(pte.ppn()),
                false => re_check(vpn),
            },
            None => re_check(vpn),
        }
    }
    pub fn translate_va_with_lazycheck(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.translate_vpn_with_lazycheck(va.floor()).map(|ppn| {
            let aligned_pa: PhysAddr = ppn.into();
            let offset = va.page_offset();
            let aligned_pa_usize: usize = aligned_pa.into();
            (aligned_pa_usize + offset).into()
        })
    }
    pub fn token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
    pub fn set_flag(&mut self, vpn: VirtPageNum, flags: PTEFlags)  {
        self.find_pte_create(vpn).unwrap().set_flag(flags);
    }
    pub fn set_cow(&mut self, vpn: VirtPageNum)  {
        self.find_pte_create(vpn).unwrap().set_cow();
    }
    pub fn reset_cow(&mut self, vpn: VirtPageNum) {
        self.find_pte_create(vpn).unwrap().reset_cow();
    }
}

pub fn translated_byte_buffer(token: usize, ptr: *const u8, len: usize) -> UserBuffVec {
    let page_table = PageTable::from_token(token);
    let mut start = ptr as usize;
    let end = start + len;
    let mut v = [(0, 0); USERBUF_MAX_SIZE];

    let mut i = 0;
    while start < end {
        let start_va = VirtAddr::from(start);
        let mut vpn = start_va.floor();
        let ppn = page_table.translate_vpn_with_lazycheck(vpn).unwrap();
        vpn.step();
        let mut end_va: VirtAddr = vpn.into();
        end_va = end_va.min(VirtAddr::from(end));
        // info!(
        //     "start_va = {:#x?}, end_va = {:#x?}, vpn = {:#x?}, ppn = {:#x?}",
        //     start_va, end_va, vpn, ppn
        // );
        let pa_slice = if end_va.page_offset() == 0 {
            &ppn.slice_u8()[start_va.page_offset()..]
        } else {
            &ppn.slice_u8()[start_va.page_offset()..end_va.page_offset()]
        };
        v[i] = (
            pa_slice.as_ptr() as usize,
            pa_slice.as_ptr() as usize + pa_slice.len(),
        );
        i += 1;
        start = end_va.into();
    }
    UserBuffVec { bufs: v, sz: i }
}

/// Load a string from other address spaces into kernel space without an end `\0`.
pub fn translated_str(token: usize, ptr: *const u8) -> String {
    let page_table = PageTable::from_token(token);
    let mut string = String::with_capacity(128);
    let mut start_va = ptr as usize;
    let mut done = false;
    loop {
        let start_pa: usize = page_table
            .translate_va_with_lazycheck(VirtAddr::from(start_va))
            .unwrap()
            .into();
        for pa in start_pa..aligned_down(start_pa) + PAGE_SIZE {
            let ch = unsafe {*(pa as *mut u8)};
            if ch == 0 {
                done = true;
                break;
            }
            string.push(ch as char);
        }
        if done {
            break;
        }
        start_va = aligned_down(start_va) + PAGE_SIZE;
    }
    string
}

/// 不支持跨页读写
pub fn translated_ref<T>(token: usize, ptr: *const T) -> &'static T {
    let page_table = PageTable::from_token(token);
    page_table
        .translate_va_with_lazycheck(VirtAddr::from(ptr as usize))
        .unwrap()
        .get_ref()
}

pub fn translated_refmut<T>(token: usize, ptr: *mut T) -> &'static mut T {
    let page_table = PageTable::from_token(token);
    let va = ptr as usize;
    page_table
        .translate_va_with_lazycheck(VirtAddr::from(va))
        .unwrap()
        .get_mut()
}

const USERBUF_MAX_SIZE: usize = 18;

/// 保存物理地址范围
pub struct UserBuffVec {
    pub bufs: [(usize, usize); USERBUF_MAX_SIZE],
    pub sz: usize,
}

impl UserBuffVec {
    pub fn from_single_slice(buf: &'static mut [u8]) -> Self {
        let mut bufs = [(0, 0); USERBUF_MAX_SIZE];
        bufs[0] = (buf.as_ptr() as usize, buf.as_ptr() as usize + buf.len());
        Self { bufs, sz: 1 }
    }
}

pub struct UserBuffer {
    pub bufvec: UserBuffVec,
    len: usize,
    id: usize,
    offset: usize
}

impl UserBuffer {
    pub fn new(bufvec: UserBuffVec) -> Self {
        let _j = bufvec.bufs[0].0;
        let mut len: usize = 0;
        for b in bufvec.bufs[0..bufvec.sz].iter() {
            len += b.1 - b.0;
        }
        Self { bufvec, len, id: 0, offset: _j }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn copy_from_user(&mut self, buf: &mut [u8]) -> usize {
        let end = buf.len();
        let mut start = 0;
        
        while self.id < self.bufvec.sz {
            let sub_buf = self.bufvec.bufs[self.id];
            if start == end {
                return start;
            }
            let slen = (sub_buf.1 - self.offset).min(end - start);
            unsafe{
                &buf[start..start+slen].copy_from_slice(
                core::slice::from_raw_parts(self.offset as *mut u8, slen));
            }
            start += slen;
            if slen == sub_buf.1 - self.offset {
                self.id += 1;
                self.offset = self.bufvec.bufs[self.id].0;
            } else {
                self.offset += slen;
            }
        }
        start
    }

    pub fn copy_to_user(&mut self, buf: &[u8]) -> usize {
        let end = buf.len();
        let mut start = 0;
        
        while self.id < self.bufvec.sz {
            let sub_buf = self.bufvec.bufs[self.id];
            if start == end {
                return start;
            }
            let slen = (sub_buf.1 - self.offset).min(end - start);
            unsafe{
                core::slice::from_raw_parts_mut(self.offset as *mut u8, slen)
                .copy_from_slice(&buf[start..start+slen]);
            }
            start += slen;
            if slen == sub_buf.1 - self.offset {
                self.id += 1;
                self.offset = self.bufvec.bufs[self.id].0;
            } else {
                self.offset += slen;
            }
        }
        start
    }

    pub fn clear(&mut self) -> usize {
        self.bufvec.bufs[0..(self.bufvec.sz)]
            .iter_mut()
            .for_each(|buf| unsafe {
                core::slice::from_raw_parts_mut(buf.0 as *mut u8, buf.1 - buf.0).fill(0)
            });
        self.len()
    }
}

impl IntoIterator for UserBuffer {
    type Item = *mut u8;
    type IntoIter = UserBufferIterator;
    fn into_iter(self) -> Self::IntoIter {
        UserBufferIterator {
            bufvec: self.bufvec,
            i: 0,
            j: 0,
        }
    }
}

pub struct UserBufferIterator {
    bufvec: UserBuffVec,
    i: usize,
    j: usize,
}

impl Iterator for UserBufferIterator {
    type Item = *mut u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.bufvec.sz {
            None
        } else {
            let r = self.bufvec.bufs[self.i].0 + self.j;
            if r + 1 == self.bufvec.bufs[self.i].1 {
                self.j = 0;
                self.i += 1;
            } else {
                self.j += 1;
            }
            Some(r as *mut _)
        }
    }
}
