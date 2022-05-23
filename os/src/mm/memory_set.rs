use super::mmap::MmapArea;
use super::{frame_alloc, FrameTracker};
use super::{PTEFlags, PageTable, PageTableEntry};
use super::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use super::{StepByOne, VPNRange};
use crate::config::{MEMORY_END, MMIO, PAGE_SIZE, TRAMPOLINE, USER_STACK_BASE};
use crate::gdb_println;
use crate::monitor::{MAPPING_ENABLE, QEMU};
use crate::task::{
    AuxHeader, FdTable, AT_BASE, AT_CLKTCK, AT_EGID, AT_ENTRY, AT_EUID, AT_FLAGS, AT_GID, AT_HWCAP,
    AT_NOTELF, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_SECURE, AT_UID,
};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::arch::asm;
use riscv::register::satp;
use spin::{Lazy, RwLock};

extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

pub static mut SATP: usize = 0;

pub static KERNEL_SPACE: Lazy<RwLock<MemorySet>> =
    Lazy::new(|| RwLock::new(MemorySet::new_kernel()));

pub fn kernel_token() -> usize {
    KERNEL_SPACE.read().token()
}

pub struct MemorySet {
    page_table: PageTable,
    areas: Vec<MapArea>,
    heap_frames: BTreeMap<VirtPageNum, FrameTracker>,
    mmap_areas: Vec<MmapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        info!("new_bare s");
        Self {
            page_table: PageTable::new(),
            areas: Vec::new(),
            heap_frames: BTreeMap::new(),
            mmap_areas: Vec::new(),
        }
    }
    pub fn token(&self) -> usize {
        self.page_table.token()
    }
    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(start_va, end_va, MapType::Framed, permission),
            None,
            0,
        );
    }
    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        if let Some((idx, area)) = self
            .areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.vpn_range.get_start() == start_vpn)
        {
            area.unmap(&mut self.page_table);
            self.areas.remove(idx);
        }
    }
    /// 添加了offset字段以解决内存不对齐的问题
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>, offset: usize) {
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data, offset);
        }
        self.areas.push(map_area);
    }
    /// Mention that trampoline is not collected by areas.
    fn map_trampoline(&mut self) {
        gdb_println!(
            MAPPING_ENABLE,
            "map_trampoline onepage va[0x{:X}-] -> pa[0x{:X}-]",
            TRAMPOLINE,
            strampoline as usize
        );
        info!("map_trampoline onepage va[0x{:X}-] -> pa[0x{:X}-]",
        TRAMPOLINE,
        strampoline as usize);
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }
    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        info!("new_kernel s");
        let mut memory_set = Self::new_bare();
        // map trampoline
        info!("map trampoline s");
        memory_set.map_trampoline();
        info!("map trampoline e");
        // map kernel sections
        debug!(".text va[{:#x}, {:#x})", stext as usize, etext as usize);
        debug!(
            ".rodata va[{:#x}, {:#x})",
            srodata as usize, erodata as usize
        );
        debug!(".data va[{:#x}, {:#x})", sdata as usize, edata as usize);
        debug!(
            ".bss va[{:#x}, {:#x})",
            sbss_with_stack as usize, ebss as usize
        );
        debug!("mapping .text section Identical");
        memory_set.push(
            MapArea::new(
                (stext as usize).into(),
                (etext as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::X,
            ),
            None,
            0,
        );
        debug!("mapping .rodata section Identical");
        memory_set.push(
            MapArea::new(
                (srodata as usize).into(),
                (erodata as usize).into(),
                MapType::Identical,
                MapPermission::R,
            ),
            None,
            0,
        );
        debug!("mapping .data section Identical");
        memory_set.push(
            MapArea::new(
                (sdata as usize).into(),
                (edata as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
            0,
        );
        debug!("mapping .bss section Identical");
        memory_set.push(
            MapArea::new(
                (sbss_with_stack as usize).into(),
                (ebss as usize).into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
            0,
        );
        debug!("mapping physical memory Identical");
        memory_set.push(
            MapArea::new(
                (ekernel as usize).into(),
                MEMORY_END.into(),
                MapType::Identical,
                MapPermission::R | MapPermission::W,
            ),
            None,
            0,
        );
        debug!("mapping memory-mapped registers Identical");
        for pair in MMIO {
            debug!(
                "MMIO range [ {:#x} ~ {:#x}  ]",
                (*pair).0,
                (*pair).0 + (*pair).1
            );
            memory_set.push(
                MapArea::new(
                    (*pair).0.into(),
                    // (*pair).1.into(),
                    ((*pair).0 + (*pair).1).into(),
                    MapType::Identical,
                    MapPermission::R | MapPermission::W,
                ),
                None,
                0,
            );
        }
        debug!("mapping done");
        debug!("kernel stap {:#x}", memory_set.page_table.token());
        memory_set
    }
    /// Include sections in elf and trampoline,
    /// also returns user_sp_base and entry point.
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize, usize, Vec<AuxHeader>) {
        let mut auxv: Vec<AuxHeader> = Vec::new();
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);

        auxv.push(AuxHeader {
            aux_type: AT_PHENT,
            value: elf.header.pt2.ph_entry_size() as usize,
        }); // ELF64 header 64bytes
        auxv.push(AuxHeader {
            aux_type: AT_PHNUM,
            value: ph_count as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PAGESZ,
            value: PAGE_SIZE as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_BASE,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_FLAGS,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_ENTRY,
            value: elf.header.pt2.entry_point() as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_UID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EUID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_GID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_EGID,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_PLATFORM,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_HWCAP,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_CLKTCK,
            value: 100 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_SECURE,
            value: 0 as usize,
        });
        auxv.push(AuxHeader {
            aux_type: AT_NOTELF,
            value: 0x112d as usize,
        });

        let mut head_va = 0;

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(start_va, end_va, MapType::Framed, map_perm);
                max_end_vpn = map_area.vpn_range.get_end();
                memory_set.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                    start_va.page_offset(),
                );
                gdb_println!(
                    MAPPING_ENABLE,
                    "[user-elfmap] va[0x{:X} - 0x{:X}] Framed",
                    start_va.0,
                    end_va.0
                );

                if start_va.aligned() {
                    head_va = start_va.0;
                }
            }
        }

        let ph_head_addr = head_va + elf.header.pt2.ph_offset() as usize;
        auxv.push(AuxHeader {
            aux_type: AT_PHDR,
            value: ph_head_addr as usize,
        });

        let max_end_va: VirtAddr = max_end_vpn.into();
        let user_heap_base: usize = max_end_va.into();
        (
            memory_set,
            USER_STACK_BASE,
            elf.header.pt2.entry_point() as usize,
            user_heap_base,
            auxv,
        )
    }
    pub fn from_existed_user(user_space: &MemorySet) -> MemorySet {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
        // copy data sections/trap_context/user_stack
        for area in user_space.areas.iter() {
            let new_area = MapArea::from_another(area);
            memory_set.push(new_area, None, 0);
            // copy data from another space
            for vpn in area.vpn_range {
                let src_ppn = user_space.translate(vpn).unwrap().ppn();
                let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(src_ppn.get_bytes_array());
            }
        }
        // 复制mmap区域
        for area in user_space.mmap_areas.iter() {
            // 建立新的mmap_area，有vpn范围和dataframes（没有数据）
            let new_area = MmapArea::from_another(area);
            // 建立页表映射（物理页帧自动分配），同时添加dataframes
            memory_set.push_and_map_mmap_area(new_area);
            // 拷贝数据
            for vpn in area.data_frames.keys() {
                let src_ppn = user_space.translate(*vpn).unwrap().ppn();
                let dst_ppn = memory_set.translate(*vpn).unwrap().ppn();
                dst_ppn
                    .get_bytes_array()
                    .copy_from_slice(src_ppn.get_bytes_array());
            }
        }
        // 复制heap区域
        for &vpn in user_space.heap_frames.keys() {
            let frame = frame_alloc().unwrap();
            let ppn = frame.ppn;
            memory_set.heap_frames.insert(vpn, frame_alloc().unwrap());
            memory_set.page_table.map(vpn, ppn, PTEFlags::U | PTEFlags::R | PTEFlags::W);
            // copy data from another space
            let src_ppn = user_space.translate(vpn).unwrap().ppn();
            ppn
                .get_bytes_array()
                .copy_from_slice(src_ppn.get_bytes_array());
        }
        memory_set
    }
    pub fn activate(&self) {
        info!("activate s");
        let satp = self.page_table.token();
        debug!("activate stap {:#x}", satp);
        info!("activate 1");
        unsafe {
            SATP = satp; //其他核初始化
            satp::write(satp);
            info!("activate 2");
            asm!("sfence.vma");
        }
        info!("activate e");
    }
    pub fn translate(&self, vpn: VirtPageNum) -> Option<PageTableEntry> {
        self.page_table.translate(vpn)
    }
    pub fn recycle_data_pages(&mut self) {
        //*self = Self::new_bare();
        self.areas.clear();
    }

    /// 插入一个mmap区域
    pub fn push_mmap_area(&mut self, mmap_area: MmapArea) {
        self.mmap_areas.push(mmap_area);
    }

    /// 插入一个mmap区域并将mmap_area添加到页表
    pub fn push_and_map_mmap_area(&mut self, mmap_area: MmapArea) {
        // 添加页表和dataframes
        mmap_area.map_all(&mut self.page_table);
        self.mmap_areas.push(mmap_area);
    }

    /// (lazy) 为vpn处的虚拟地址分配一个mmap页面，失败返回-1
    pub fn insert_mmap_dataframe(&mut self, vpn: VirtPageNum, fd_table: FdTable) -> isize {
        for mmap_area in self.mmap_areas.iter_mut() {
            if vpn >= mmap_area.start_vpn && vpn < mmap_area.end_vpn {
                return mmap_area.map_one(&mut self.page_table, fd_table, vpn);
            }
        }
        // if failed
        -1
    }

    pub fn remove_mmap_area_with_start_vpn(&mut self, vpn: VirtPageNum) -> isize {
        if let Some((idx, area)) = self
            .mmap_areas
            .iter_mut()
            .enumerate()
            .find(|(_, area)| area.start_vpn == vpn)
        {
            area.unmap(&mut self.page_table);
            self.mmap_areas.remove(idx);
            return 0;
        }
        // if failed
        -1
    }

    pub fn insert_heap_dataframe(
        &mut self,
        va: usize,
        user_heap_base: usize,
        user_heap_top: usize,
    ) -> isize {
        if va >= user_heap_base && va < user_heap_top {
            // alloc a frame
            let vpn = VirtAddr::from(va).floor();
            let frame = frame_alloc().unwrap();
            self.page_table
                .map(vpn, frame.ppn, PTEFlags::U | PTEFlags::R | PTEFlags::W);
            self.heap_frames.insert(vpn, frame);
            0
        } else {
            -1
        }
    }

    pub fn remove_heap_dataframes(&mut self, prev_top: usize, current_top: usize) {
        // println!("remove_heap_dataframes {:#x?} {:#x}", prev_top, current_top);
        assert!(current_top < prev_top);
        let dropped: Vec<(_, _)> = self
            .heap_frames
            .drain_filter(|vpn, _| {
                let vpn_addr: usize = VirtAddr::from(VirtPageNum::from(*vpn)).into();
                vpn_addr >= current_top && vpn_addr <= prev_top
            })
            .collect();

        // pagetalbe unmapping...
        for (vpn, _) in dropped.iter() {
            self.page_table.unmap(*vpn);
        }

        // Aautomatically drop FrameTrackers here...
    }
}

pub struct MapArea {
    vpn_range: VPNRange,
    data_frames: BTreeMap<VirtPageNum, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
}

impl MapArea {
    pub fn new(
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: BTreeMap::new(),
            map_type,
            map_perm,
        }
    }
    pub fn from_another(another: &MapArea) -> Self {
        Self {
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            data_frames: BTreeMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
        }
    }
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn);
        }
        page_table.unmap(vpn);
    }
    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.map_one(page_table, vpn);
        }
    }
    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpn_range {
            self.unmap_one(page_table, vpn);
        }
    }
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8], mut offset: usize) {
        // println!("copy_data offset = {:#x?}", offset);
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpn_range.get_start();
        let len = data.len();
        loop {
            let copy_len = PAGE_SIZE.min(len - start).min(PAGE_SIZE - offset);
            let src = &data[start..start + copy_len];
            let dst = &mut page_table
                .translate(current_vpn)
                .unwrap()
                .ppn()
                .get_bytes_array()[offset..offset + copy_len];
            // println!("offset = {:#x?}, copy_len = {:#x?}, start = {:#x?}", offset, copy_len, start);
            dst.copy_from_slice(src);
            start += copy_len;
            offset = 0;
            if start >= len {
                break;
            }
            current_vpn.step();
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

bitflags! {
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

#[allow(unused)]
pub fn remap_test() {
    debug!("remap test start...");
    let mut kernel_space = KERNEL_SPACE.read();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert!(!kernel_space
        .page_table
        .translate(mid_text.floor())
        .unwrap()
        .writable(),);

    assert!(!kernel_space
        .page_table
        .translate(mid_rodata.floor())
        .unwrap()
        .writable(),);

    assert!(!kernel_space
        .page_table
        .translate(mid_data.floor())
        .unwrap()
        .executable(),);
    debug!("remap_test passed!");
}
