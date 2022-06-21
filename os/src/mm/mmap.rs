use alloc::collections::BTreeMap;

use crate::{
    config::PAGE_SIZE,
    fs::{File, FileClass},
    task::FdTable,
};

use super::{
    frame_alloc, page_table::PTEFlags, translated_byte_buffer, FrameTracker, MapPermission,
    PageTable, PhysPageNum, UserBuffer, VirtAddr, VirtPageNum,
};

pub struct MmapArea {
    pub start_vpn: VirtPageNum,
    pub end_vpn: VirtPageNum,
    pub map_perm: MapPermission,
    pub flags: usize,
    pub fd: usize,
    pub offset: usize,
    pub data_frames: BTreeMap<VirtPageNum, FrameTracker>,
}

impl MmapArea {
    pub fn new(
        start_vpn: VirtPageNum,
        end_vpn: VirtPageNum,
        map_perm: MapPermission,
        flags: usize,
        fd: usize,
        offset: usize,
    ) -> Self {
        Self {
            start_vpn,
            end_vpn,
            map_perm,
            flags,
            fd,
            offset,
            data_frames: BTreeMap::new(),
        }
    }

    pub fn from_another(another: &MmapArea) -> Self {
        let mut new_area = Self {
            start_vpn: another.start_vpn,
            end_vpn: another.end_vpn,
            map_perm: another.map_perm,
            flags: another.flags,
            fd: another.fd,
            offset: another.offset,
            data_frames: BTreeMap::new(),
        };
        for (vpn, _) in (&another.data_frames).into_iter() {
            let frame = frame_alloc().unwrap();
            new_area.data_frames.insert(*vpn, frame);
        }
        new_area
    }

    pub fn map_all(&self, page_table: &mut PageTable) {
        for (vpn, frame) in (&self.data_frames).into_iter() {
            let ppn = frame.ppn;
            let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
            page_table.map(*vpn, ppn, pte_flags);
        }
    }

    /// (lazy)分配一个物理页帧并建立vpn到它的mmap映射，同时从fd中读取对应文件，失败返回-1
    pub fn map_one(
        &mut self,
        page_table: &mut PageTable,
        fd_table: FdTable,
        vpn: VirtPageNum,
    ) -> isize {
        let ppn: PhysPageNum;
        let frame = frame_alloc().unwrap();
        ppn = frame.ppn;
        self.data_frames.insert(vpn, frame);

        let pte_flags = PTEFlags::from_bits(self.map_perm.bits()).unwrap();
        page_table.map(vpn, ppn, pte_flags);

        let token = page_table.token();

        if self.fd as isize == -1 {
            return 0;
        }

        if let Some(file) = &fd_table[self.fd] {
            match file {
                FileClass::File(f) => {
                    let vaddr = VirtAddr::from(vpn).0;
                    f.set_offset(self.offset + vaddr - VirtAddr::from(self.start_vpn).0);
                    if !f.readable() {
                        return -1;
                    }
                    //println!{"The va_start is 0x{:X}, offset of file is {}", va_start.0, offset};
                    f.read(UserBuffer::new(translated_byte_buffer(
                        token,
                        vaddr as *const u8,
                        PAGE_SIZE,
                    )));
                    //println!{"read {} bytes", read_len};
                }
                _ => {
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
            page_table.unmap(*vpn);
        }
    }
}
