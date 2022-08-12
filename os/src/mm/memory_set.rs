use super::mmap::MmapArea;
use super::{frame_alloc, FrameTracker};
use super::{PTEFlags, PageTable, PageTableEntry};
use super::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use super::{StepByOne, VPNRange};
use crate::config::{
    DYNAMIC_LINKER, MEMORY_END, MMIO, PAGE_SIZE, SIGRETURN_TRAMPOLINE, TRAMPOLINE, USER_STACK_BASE,
};
use crate::fs::{open_common_file, OpenFlags};
use crate::gdb_println;
use crate::monitor::{MAPPING_ENABLE, QEMU};
use crate::task::{
    AuxHeader, AT_BASE, AT_CLKTCK, AT_EGID, AT_ENTRY, AT_EUID, AT_FLAGS, AT_GID, AT_HWCAP,
    AT_NOTELF, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_SECURE, AT_UID,
};
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
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

// TODO 此处仅仅在内核地址空间中保存一份lic.so, 可使用更优雅的方式解决
// 还需在动态链接加载其不同时继续进行处理
pub static KERNEL_DL_DATA: Lazy<RwLock<DLLMem>> = Lazy::new(|| RwLock::new(DLLMem::new()));

pub struct DLLMem{
    pub data: Vec<u8>,
    pub name: String,
}

impl DLLMem {
    pub fn new() ->Self {
        Self {
            data: Vec::with_capacity(0x1000),
            name: "NULL".to_string(),
        }
    }
    pub fn readso(&mut self ,cwd: &str, path: &str){
        if let Some(app_vfile) = open_common_file(cwd, path, OpenFlags::RDONLY) {
            self.name = path.to_string();
            self.data = app_vfile.read_all();
        } else {
            error!("[execve load_dl] dynamic load dl {:#x?} false ... cwd is {:#x?}", path, cwd);
            self.name = "NULL".to_string();
            self.data.clear();
        }
    }
}

// fn read_dll(cwd: &str, path: &str) -> DLLMem {
//     if let Some(app_vfile) = open_common_file(cwd, path, OpenFlags::RDONLY) {
//         // let s = "libc.so".to_string();
//         // let data = app_vfile.read_all();
//         return
//             DLLMem::new();
//     } else {
//         error!("[execve load_dl] dynamic load dl false");
//         return DLLMem::new("NULL");
//     }
// }

pub fn load_dll() {
    let dl = KERNEL_DL_DATA.read();
    debug!(
        "load {} to kernel memoryset size:0x{:x} ......",
        dl.name ,dl.data.len()
    );
}

pub static KERNEL_SPACE: Lazy<RwLock<MemorySet>> =
    Lazy::new(|| RwLock::new(MemorySet::new_kernel()));

pub fn kernel_token() -> usize {
    KERNEL_SPACE.read().token()
}

pub struct MemorySet {
    pub page_table: PageTable,
    areas: Vec<MapArea>,
    heap_frames: BTreeMap<VirtPageNum, FrameTracker>,
    pub mmap_areas: Vec<MmapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::with_capacity(0x100),
            heap_frames: BTreeMap::new(),
            mmap_areas: Vec::with_capacity(0x100),
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
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }
    fn map_sigreturn_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(SIGRETURN_TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X | PTEFlags::U,
        );
    }
    /// Without kernel stacks.
    pub fn new_kernel() -> Self {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_trampoline();
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
        memory_set
    }
    /// load libc.so(elf) to DYNAMIC_LINKER
    /// return value 0: erro,   other: ld入口地址 (&mut self, elf_data: &[u8])
    pub fn load_dl(&mut self, elf_data: &[u8]) -> usize {
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let s = match elf.find_section_by_name(".interp") {
            Some(s) => s,
            None => return 0,
        };
        let s = s.raw_data(&elf).to_vec();
        let mut s = String::from_utf8(s).unwrap();

        // 移除末尾\0
        s = s.strip_suffix("\0").unwrap_or(&s).to_string();

        // info!("load_linker interp: {:?}", s);
        // 手动转发到 libc.so
        if s == "/lib/ld-musl-riscv64-sf.so.1" {
            s = "libc.so".to_string();
        }
        // info!("load_linker interp: {:?}", s);
        let memdll = &mut KERNEL_DL_DATA.write();
        if s != memdll.name {
            // info!("load_linker to mem");
            memdll.readso("/", s.as_str());
        }

        let all_data = &memdll.data;
        if all_data.len() == 0 {
            return 0;
        }
        // println!("[load_dl]  KERNEL_DL_DATA.len():{}", all_data.len());
        let elf_data = all_data.as_slice();
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf-dl !");
        let ph_count = elf_header.pt2.ph_count();

        for i in 0..ph_count {
            let ph = elf.program_header(i).unwrap();
            let start_va: VirtAddr = (DYNAMIC_LINKER + ph.virtual_addr() as usize).into();
            // let start_va: VirtAddr = (ph.virtual_addr() as usize).into(); // virtual_addr 应该是0
            let end_va: VirtAddr =
                (DYNAMIC_LINKER + ph.virtual_addr() as usize + ph.mem_size() as usize).into();

            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // println!("[load_dl] start_va:{:#?},   end_va: {:#?} ", start_va, end_va);
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
                // println!("[load_dl]  elf.input:{}, start:{},   end:{} ", &elf.input.len(), ph.offset() as usize, (ph.offset() + ph.file_size()) as usize);
                self.push(
                    map_area,
                    Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                    start_va.page_offset(),
                );

                // gdb_println!(
                //     SYSCALL_ENABLE,
                //     "[load_dl] va[0x{:X} - 0x{:X}] Framed",
                //     start_va.0,
                //     end_va.0
                // );
            }
        }
        return elf_header.pt2.entry_point() as usize;
    }
    /// Include sections in elf and trampoline,
    /// also returns user_sp_base and entry point.
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize, usize, Vec<AuxHeader>) {
        let mut auxv: Vec<AuxHeader> = Vec::with_capacity(64);
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_sigreturn_trampoline();
        memory_set.map_trampoline();
        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let ph_count = elf_header.pt2.ph_count();
        let mut max_end_vpn = VirtPageNum(0);
        let mut _at_base = 0usize;

        // println!("[from_elf] program_header-2 : type is {:#?} ",ph.get_type().unwrap());
        // run other programs
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
        // todo is_dynamic = 1;
        // .interp: 2 第二个
        // .strtab: ph_count-2 倒数第二个
        // TODO 通过方法寻找？ 可读性更高？
        // let dl_sec = elf.program_header(1).unwrap();
        _at_base = memory_set.load_dl(elf_data);
        if _at_base != 0 {
            // error!("load_dl finish !");
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: DYNAMIC_LINKER as usize,
            });
            // println!("chech_addr : {:X} ",at_base);
            // check
            // if let Some(chech_addr) = memory_set.page_table.translate_va(DYNAMIC_LINKER.into()){
            //     println!("chech_addr : {:?} ",chech_addr);
            // }else {
            //     error!("check no pass");
            // }

            _at_base += DYNAMIC_LINKER;
        }

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
            // println!("[from_elf] program_header-{:#?} : type is {:#?} ", i, ph.get_type().unwrap());
            // println!("[from_elf] virtual_addr : {:X},  mem_size is {:X} ", ph.virtual_addr(), ph.mem_size());
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                // println!(" +++ [from_elf] program_header-{:#?} : type is {:#?} ", i, ph.get_type().unwrap());
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                let mut map_perm = MapPermission::U;
                let ph_flags = ph.flags();
                if head_va == 0 {
                    head_va = start_va.0;
                }
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
            }
        }

        let ph_head_addr = head_va + elf.header.pt2.ph_offset() as usize;
        // println!("[from_elf] AT_PHDR  ph_head_addr is {:X} ", ph_head_addr);
        auxv.push(AuxHeader {
            aux_type: AT_PHDR,
            value: ph_head_addr as usize,
        });

        let entry: usize;
        if _at_base == 0 {
            // 静态链接程序
            entry = elf.header.pt2.entry_point() as usize;
        } else {
            entry = _at_base;
        }

        // println!("[from_elf] elf entry : {:X} ",elf.header.pt2.entry_point() as usize);

        let max_end_va: VirtAddr = max_end_vpn.into();
        let user_heap_base: usize = max_end_va.into();
        (
            memory_set,
            USER_STACK_BASE,
            // elf.header.pt2.entry_point() as usize,
            entry,
            user_heap_base,
            auxv,
        )
    }
    pub fn from_existed_user(user_space: &MemorySet) -> MemorySet {
        let mut memory_set = Self::new_bare();
        // map trampoline
        memory_set.map_sigreturn_trampoline();
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
                    .slice_u64()
                    .copy_from_slice(src_ppn.slice_u64());
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
                // debug!("mmap vpn copy {:#x}", vpn.0);
                let src_ppn = user_space.translate(*vpn).unwrap().ppn();
                let dst_ppn = memory_set.translate(*vpn).unwrap().ppn();
                // debug!("mmap copy: src_ppn = {:#x?},  dst_ppn {:#x}", src_ppn.0, dst_ppn.0);
                dst_ppn
                    .slice_u64()
                    .copy_from_slice(src_ppn.slice_u64());
            }
        }
        // 复制heap区域
        for &vpn in user_space.heap_frames.keys() {
            let frame = frame_alloc().unwrap();
            let ppn = frame.ppn;
            memory_set.heap_frames.insert(vpn, frame);
            memory_set
                .page_table
                .map(vpn, ppn, PTEFlags::U | PTEFlags::R | PTEFlags::W);
            // copy data from another space
            let src_ppn = user_space.translate(vpn).unwrap().ppn();
            ppn.slice_u64()
                .copy_from_slice(src_ppn.slice_u64());
        }
        memory_set
    }
    pub fn activate(&self) {
        let satp = self.page_table.token();
        unsafe {
            SATP = satp; //其他核初始化
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    /// 设置pte标志位，失败返回-1
    pub fn set_pte_flags(&self, vpn: VirtPageNum, flags: PTEFlags) -> isize {
        if self.page_table.set_pte_flags(vpn, flags).is_none() {
            -1
        } else {
            0
        }
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
    pub fn insert_mmap_dataframe(&mut self, vpn: VirtPageNum) -> isize {
        for mmap_area in self.mmap_areas.iter_mut() {
            if vpn >= mmap_area.start_vpn
                && vpn < mmap_area.end_vpn
                && !mmap_area.data_frames.contains_key(&vpn)
            {
                return mmap_area.map_one(&mut self.page_table, vpn);
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
                .slice_u8()[offset..offset + copy_len];
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
