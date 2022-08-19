use hashbrown::HashMap;

use crate::{
    config::PAGE_SIZE,
    fs::{File, FileClass},
};

use super::{
    address::VPNRange, frame_alloc, frame_allocator::frame_alloc_without_clear,
    page_table::PTEFlags, translated_byte_buffer, FrameTracker, MapPermission, PageTable, PhysAddr,
    PhysPageNum, UserBuffer, VirtAddr, VirtPageNum,
};

bitflags! {
    #[derive(Default)]
    pub struct MmapFlags: usize {
        const MAP_32BIT = 0;
        const MAP_SHARED = 1 << 0;
        const MAP_PRIVATE = 1 << 1;
        const _X2 = 1 << 2;
        const _X3 = 1 << 3;
        const MAP_FIXED = 1 << 4;
        const MAP_ANONYMOUS = 1 << 5;
        const _X6 = 1 << 6;
        const _X7 = 1 << 7;
        const _X8 = 1 << 8;
        const _X9 = 1 << 9;
        const _X10 = 1 << 10;
        const _X11 = 1 << 11;
    }
}

pub type FdOne = Option<FileClass>;

pub struct MmapArea {
    // pub start_vpn: VirtPageNum,
    // pub end_vpn: VirtPageNum,
    pub vpn_range: VPNRange,
    pub map_perm: MapPermission,
    pub flags: usize,
    pub fd_one: FdOne,
    pub fd: usize,
    pub offset: usize,
    pub data_frames: HashMap<usize, Option<FrameTracker>>,
}

impl MmapArea {
    pub fn new(
        start_vpn: VirtPageNum,
        end_vpn: VirtPageNum,
        map_perm: MapPermission,
        flags: usize,
        fd_one: FdOne,
        fd: usize,
        offset: usize,
    ) -> Self {
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            map_perm,
            flags,
            fd_one,
            fd,
            offset,
            data_frames: HashMap::new(),
        }
    }

    pub fn from_another(another: &MmapArea) -> Self {
        let mut new_area = Self {
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            map_perm: another.map_perm,
            flags: another.flags,
            fd_one: another.fd_one.clone(),
            fd: another.fd,
            offset: another.offset,
            data_frames: HashMap::new(),
        };
        for (vpn, _) in (&another.data_frames).into_iter() {
            let frame = frame_alloc_without_clear().unwrap();
            // let frame = frame_alloc().unwrap();
            new_area.data_frames.insert(*vpn, Some(frame));
        }
        new_area
    }

    pub fn cow_from_another(another: &MmapArea) -> Self {
        let mut new_area = Self {
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            map_perm: another.map_perm,
            flags: another.flags,
            fd_one: another.fd_one.clone(),
            fd: another.fd,
            offset: another.offset,
            data_frames: HashMap::new(),
        };
        new_area
    }

    /// 这里有问题：pte_flags可能被sys_mprotect修改，导致其与self.map_perm不一致.
    /// fake solution here.
    pub fn map_all(&self, page_table: &mut PageTable) {
        for (vpn, frame_unwrapped) in (&self.data_frames).into_iter() {
            if let Some(frame) = frame_unwrapped {
                let ppn = frame.ppn;
                // let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
                let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap()
                    | PTEFlags::U
                    | PTEFlags::R
                    | PTEFlags::W;
                page_table.map((*vpn).into(), ppn, pte_flags);
            }
        }
    }

    /// (lazy)分配一个物理页帧并建立vpn到它的mmap映射，同时从fd中读取对应文件，失败返回-1
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) -> isize {
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();

        // println!{"The translate_va 0x800001f0 is 0x{:#?}", page_table.translate_va((0x800001f0 as usize).into())};
        let token = page_table.token();

        if self.fd as isize == -1 {
            let frame = frame_alloc().unwrap();
            let ppn = frame.ppn;
            self.data_frames.insert(vpn.0, Some(frame));
            page_table.map(vpn, ppn, pte_flags);
            return 0;
        }

        if let Some(file) = &self.fd_one {
            match file {
                FileClass::File(f) => {
                    let vaddr = VirtAddr::from(vpn).0;
                    let file_off =
                        self.offset + vaddr - VirtAddr::from(self.vpn_range.get_start()).0;
                    f.set_offset(file_off);
                    if !f.readable() {
                        return -1;
                    }
                    let pa = unsafe { f.get_data_cache_physaddr(file_off) };
                    let ppn = PhysAddr::from(pa).floor();
                    self.data_frames.insert(vpn.0, None);
                    page_table.map(vpn, ppn, pte_flags);

                    // println! { "The va_start is 0x{:X}, vpn is {:#x?}, offset of file is {}, pa = {:#x?}", vaddr, vpn ,file_off, pa };
                    // f.read(UserBuffer::new(translated_byte_buffer(
                    //     token,
                    //     vaddr as *const u8,
                    //     PAGE_SIZE,
                    // )));
                    // println! {"read fd:{} bytes", self.fd};
                }
                _ => {
                    // println!{"not a OS_file"};
                    return -1;
                }
            }
        } else {
            return -1;
        }
        0
    }

    pub fn unmap(&self, page_table: &mut PageTable) {
        for vpn in self.data_frames.keys() {
            page_table.unmap((*vpn).into());
        }
    }

    /// 仅在mmaparea中插入映射
    pub fn insert_tracker(&mut self, vpn: VirtPageNum, ppn: PhysPageNum) {
        self.data_frames
            .insert(vpn.0, Some(FrameTracker::from_ppn(ppn)));
    }
}
