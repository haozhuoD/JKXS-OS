use super::frame_allocator::{frame_enquire_ref, frame_alloc_without_clear};
use super::mmap::MmapArea;
use super::{frame_alloc, FrameTracker};
use super::{PTEFlags, PageTable, PageTableEntry};
use super::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use super::{StepByOne, VPNRange};
use crate::config::{
    aligned_down, is_aligned, DYNAMIC_LINKER, MEMORY_END, MMIO, PAGE_SIZE, SIGRETURN_TRAMPOLINE,
    TRAMPOLINE, USER_STACK_BASE,
};
use crate::fs::{open_common_file, OpenFlags};
use crate::gdb_println;
use crate::monitor::{MAPPING_ENABLE, QEMU};
use crate::task::{
    AuxHeader, AT_BASE, AT_CLKTCK, AT_EGID, AT_ENTRY, AT_EUID, AT_FLAGS, AT_GID, AT_HWCAP,
    AT_NOTELF, AT_PAGESZ, AT_PHDR, AT_PHENT, AT_PHNUM, AT_PLATFORM, AT_SECURE, AT_UID,
};

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;
use core::arch::asm;
use riscv::register::satp;
use spin::{Lazy, RwLock};
use xmas_elf::ElfFile;

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

pub struct DLLMem {
    pub data: Vec<u8>,
    pub name: String,
}

impl DLLMem {
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(0x1000),
            name: "NULL".to_string(),
        }
    }
    pub fn readso(&mut self, cwd: &str, path: &str) {
        if let Some(app_vfile) = open_common_file(cwd, path, OpenFlags::RDONLY) {
            self.name = path.to_string();
            unsafe {
                self.data = app_vfile.read_as_elf().to_vec();
            }
        } else {
            error!(
                "[execve load_dl] dynamic load dl {:#x?} false ... cwd is {:#x?}",
                path, cwd
            );
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
        dl.name,
        dl.data.len()
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
    heap_frames: HashMap<usize, FrameTracker>,
    pub mmap_areas: Vec<MmapArea>,
}

impl MemorySet {
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: Vec::with_capacity(0x100),
            heap_frames: HashMap::new(),
            mmap_areas: Vec::with_capacity(0x100),
        }
    }
    pub fn token(&self) -> usize {
        self.page_table.token()
    }
    /// Assume that no conflicts.
    pub fn insert_framed_area(
        &mut self,
        area_type: MapAreaType,
        start_va: VirtAddr,
        end_va: VirtAddr,
        permission: MapPermission,
    ) {
        self.push(
            MapArea::new(area_type, start_va, end_va, MapType::Framed, permission),
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
            self.areas.swap_remove(idx);
        }
    }
    /// 在 MemorySet.page_table 中为 MapArea 创建页表项 , 页属性为MapArea 对应的属性( R W X U )
    /// 可选是否向相应MapArea页表指向区域写入数据
    /// 添加了offset字段以解决内存不对齐的问题
    fn push(&mut self, mut map_area: MapArea, data: Option<&[u8]>, offset: usize) {
        // 为 MapArea 建立页表,分配页框并分配物理内存
        map_area.map(&mut self.page_table);
        if let Some(data) = data {
            map_area.copy_data(&mut self.page_table, data, offset);
        }
        // 将MapArea 加入对应MemorySet
        self.areas.push(map_area);
    }

    /// 在 MemorySet.page_table 中为 MapArea 创建页表项
    /// 不同于push，该方法不进行复制，也不为Maparea分配页帧，而是建立physaddr -> data的直接映射
    fn push_with_direct_mapping(&mut self, map_area: MapArea) {
        let data = map_area.direct_mapping_slice.unwrap();
        let mut ppn = PhysAddr::from(data.as_ptr() as usize).floor();

        // let end_ppn = PhysAddr::from(data.as_ptr() as usize + data.len()).floor();
        let flags = PTEFlags::from_bits(map_area.map_perm.bits).unwrap();
        for vpn in map_area.vpn_range {
            // info!("push_with_direct_mapping {:#x?} -> {:#x?}", vpn, ppn);
            self.page_table.map(vpn, ppn, flags);
            ppn.step();
        }
        // error!("in fact, end_ppn = {:#x?}", end_ppn);
        self.areas.push(map_area);
    }

    /// 将MapArea 加入对应MemorySet
    fn push_mapped(&mut self, map_area: MapArea) {
        self.areas.push(map_area);
    }
    // fn push_mmap_mapped_areas(&mut self, mmap_area: MmapArea){
    //     // 将MapArea 加入对应MemorySet
    //     self.mmap_areas.push(mmap_area);
    // }
    /// Mention that trampoline is not collected by areas.
    fn map_trampoline(&mut self) {
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
                MapAreaType::KernelSpaceArea,
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
                MapAreaType::KernelSpaceArea,
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
                MapAreaType::KernelSpaceArea,
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
                MapAreaType::KernelSpaceArea,
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
                MapAreaType::KernelSpaceArea,
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
                    MapAreaType::KernelSpaceArea,
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
    pub fn load_dl(&mut self, elf: &ElfFile) -> usize {
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
                let mut area_type = MapAreaType::ElfReadOnlyArea;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                    area_type = MapAreaType::ElfReadWriteArea;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }
                let map_area = MapArea::new(area_type, start_va, end_va, MapType::Framed, map_perm);
                // println!("[load_dl]  elf.input:{}, start:{},   end:{} ", &elf.input.len(), ph.offset() as usize, (ph.offset() + ph.file_size()) as usize);
                if area_type == MapAreaType::ElfReadWriteArea {
                    self.push(
                        map_area,
                        Some(
                            &elf.input
                                [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                        ),
                        start_va.page_offset(),
                    );
                } else {
                    self.push(
                        map_area,
                        Some(
                            &elf.input
                                [ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                        ),
                        start_va.page_offset(),
                    );
                }
            }
        }
        return elf_header.pt2.entry_point() as usize;
    }
    /// Include sections in elf and trampoline,
    /// also returns user_sp_base and entry point.
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize, usize, Vec<AuxHeader>) {
        assert!(is_aligned(elf_data.as_ptr() as usize));
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
        let mut auxv: Vec<AuxHeader> = Vec::with_capacity(64);

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

        _at_base = memory_set.load_dl(&elf);

        if _at_base != 0 {
            auxv.push(AuxHeader {
                aux_type: AT_BASE,
                value: DYNAMIC_LINKER as usize,
            });
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
                let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                
                let mut map_perm = MapPermission::U;
                if head_va == 0 {
                    head_va = start_va.0;
                }
                let mut area_type = MapAreaType::ElfReadOnlyArea;
                let ph_flags = ph.flags();
                if ph_flags.is_read() {
                    map_perm |= MapPermission::R;
                }
                if ph_flags.is_write() {
                    map_perm |= MapPermission::W;
                    area_type = MapAreaType::ElfReadWriteArea;
                }
                if ph_flags.is_execute() {
                    map_perm |= MapPermission::X;
                }

                let mut map_area =
                    MapArea::new(area_type, start_va, end_va, MapType::Framed, map_perm);

                max_end_vpn = map_area.vpn_range.get_end();

                let ph_offset = ph.offset() as usize;
                let ph_file_size = ph.file_size() as usize;
                let data = &elf_data[ph_offset..(ph_offset + ph_file_size)];
                // println!("[load_dl]  elf.input:{}, start:{},   end:{} ", &elf.input.len(), ph.offset() as usize, (ph.offset() + ph.file_size()) as usize);
                if area_type == MapAreaType::ElfReadWriteArea {
                    memory_set.push(map_area, Some(data), start_va.page_offset());
                } else {
                    map_area.add_direct_mapping_slice(data);
                    memory_set.push_with_direct_mapping(map_area);
                }
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
        (memory_set, USER_STACK_BASE, entry, user_heap_base, auxv)
    }

    pub fn cow_from_existed_user(user_space: &mut MemorySet) -> MemorySet {
        let mut memory_set = Self::new_bare();
        // map trampoline & strampoline
        memory_set.map_sigreturn_trampoline();
        memory_set.map_trampoline();
        
        for area in user_space.areas.iter() {
            let mut new_area = MapArea::from_another(area);
            match area.area_type {
                MapAreaType::TrapContext => {
                    // we copy trap_context/user_stack directly
                    memory_set.push(new_area, None, 0);
                    // copy data from another space
                    for vpn in area.vpn_range {
                        let src_ppn = user_space.translate(vpn).unwrap().ppn();
                        let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                        dst_ppn.slice_u64().copy_from_slice(src_ppn.slice_u64());
                    }
                }
                MapAreaType::UserStack | MapAreaType::ElfReadWriteArea => {
                    // we apply COW for ElfReadWriteArea
                    // error!("cow-elf area_start:{:?} , area_end:{:?}",area.vpn_range.get_start(),area.vpn_range.get_end());
                    // cow 处理elf逻辑段
                    let parent_page_table = &mut user_space.page_table;
                    // 复制页表项并插入
                    for vpn in area.vpn_range {
                        // 设置相同页表（指向同一块内存）
                        // 同时修改父子进程的页属性为只读
                        // 获取父进程 页表项
                        // 获取父进程 页表项
                        // 获取父进程 页表项
                        let pte = parent_page_table.translate(vpn).unwrap();
                        let pte_flags = pte.flags() & !PTEFlags::W;
                        let ppn = pte.ppn();
                        // 并设置为只读属性
                        parent_page_table.set_flag(vpn, pte_flags);
                        parent_page_table.set_cow(vpn);
                        // 设置子进程页表项目
                        memory_set.page_table.map(vpn, ppn, pte_flags);
                        memory_set.page_table.set_cow(vpn);
                        new_area.insert_tracker(vpn, ppn);
                    }
                    memory_set.push_mapped(new_area);
                }
                MapAreaType::ElfReadOnlyArea => memory_set.push_with_direct_mapping(new_area),
                _ => unreachable!(),
            }
        }

        let parent_page_table = &mut user_space.page_table;

        // we copy mmap areas directly
        for area in user_space.mmap_areas.iter() {
            // error!("cp mmap");
            // 建立新的mmap_area，有vpn范围和dataframes（没有数据）
            let new_area = MmapArea::from_another(area);
            // 建立页表映射（物理页帧自动分配），同时添加dataframes
            memory_set.push_and_map_mmap_area(new_area);
            // 拷贝数据
            for vpn in area.data_frames.keys() {
                // debug!("mmap vpn copy {:#x}", vpn.0);
                let vpn = VirtPageNum::from(*vpn);
                let src_ppn = parent_page_table.translate(vpn).unwrap().ppn();
                let dst_ppn = memory_set.translate(vpn).unwrap().ppn();
                // debug!("mmap copy: src_ppn = {:#x?},  dst_ppn {:#x}", src_ppn.0, dst_ppn.0);
                dst_ppn.slice_u64().copy_from_slice(src_ppn.slice_u64());
            }
        }
        // we apply COW for heap areas
        for &vpn in user_space.heap_frames.keys() {
            let vpn = VirtPageNum::from(vpn);
            let pte = parent_page_table.translate(vpn).unwrap();
            let pte_flags = pte.flags() & !PTEFlags::W;
            let ppn = pte.ppn();
            // 并设置为只读属性
            parent_page_table.set_flag(vpn, pte_flags);
            parent_page_table.set_cow(vpn);
            // 设置子进程页表项目
            memory_set.page_table.map(vpn, ppn, pte_flags);
            memory_set.page_table.set_cow(vpn);
            memory_set
                .heap_frames
                .insert(vpn.0, FrameTracker::from_ppn(ppn));
        }
        memory_set
    }
    pub fn cow_alloc(&mut self, vpn: VirtPageNum, former_ppn: PhysPageNum, is_heap: bool) {
        if frame_enquire_ref(former_ppn) == 1 {
            // info!("cow_alloc ref only 1 , vpn:{:?}, former_ppn:{:?}",vpn, former_ppn);
            // 引用计数为1 无需复制, 清除cow flag 添加 W flag
            self.page_table.reset_cow(vpn);
            self.page_table.set_flag(
                vpn,
                self.page_table.translate(vpn).unwrap().flags() | PTEFlags::W,
            );
            return;
        }
        // info!("cow_alloc ref = 2");
        let frame = frame_alloc_without_clear().unwrap();
        let ppn = frame.ppn;
        self.page_table.cow_remap(vpn, ppn, former_ppn);
        // info!("cow_remapping  vpn:{:?}, former_ppn:{:?}, ppn:{:?}",vpn, former_ppn, ppn);
        if is_heap {
            self.heap_frames.insert(vpn.0, frame);
            return;
        }
        for area in self.areas.iter_mut() {
            let head_vpn = area.vpn_range.get_start();
            let tail_vpn = area.vpn_range.get_end();
            if vpn < tail_vpn && vpn >= head_vpn {
                area.data_frames.insert(vpn.0, frame);
                break;
            }
        }
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
        self.areas.clear();
        self.mmap_areas.clear();
        self.heap_frames.clear();
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
            if vpn >= mmap_area.vpn_range.get_start()
                && vpn < mmap_area.vpn_range.get_end()
                && !mmap_area.data_frames.contains_key(&vpn.0)
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
            .find(|(_, area)| area.vpn_range.get_start() == vpn)
        {
            area.unmap(&mut self.page_table);
            self.mmap_areas.swap_remove(idx);
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
            self.heap_frames.insert(vpn.0, frame);
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
            self.page_table.unmap(VirtPageNum::from(*vpn));
        }

        // Aautomatically drop FrameTrackers here...
    }
}

pub struct MapArea {
    area_type: MapAreaType,
    vpn_range: VPNRange,
    data_frames: HashMap<usize, FrameTracker>,
    map_type: MapType,
    map_perm: MapPermission,
    direct_mapping_slice: Option<&'static [u8]>,
}

impl MapArea {
    pub fn new(
        area_type: MapAreaType,
        start_va: VirtAddr,
        end_va: VirtAddr,
        map_type: MapType,
        map_perm: MapPermission,
    ) -> Self {
        let start_vpn: VirtPageNum = start_va.floor();
        let end_vpn: VirtPageNum = end_va.ceil();
        Self {
            area_type,
            vpn_range: VPNRange::new(start_vpn, end_vpn),
            data_frames: HashMap::new(),
            map_type,
            map_perm,
            direct_mapping_slice: None,
        }
    }
    pub fn from_another(another: &MapArea) -> Self {
        Self {
            area_type: another.area_type,
            vpn_range: VPNRange::new(another.vpn_range.get_start(), another.vpn_range.get_end()),
            data_frames: HashMap::new(),
            map_type: another.map_type,
            map_perm: another.map_perm,
            direct_mapping_slice: another.direct_mapping_slice,
        }
    }
    /// 仅在Btree中插入 映射
    pub fn insert_tracker(&mut self, vpn: VirtPageNum, ppn: PhysPageNum) {
        self.data_frames.insert(vpn.0, FrameTracker::from_ppn(ppn));
    }
    // 建立页框并分配物理内存, 不清空
    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Identical => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                // let frame = frame_alloc_without_clear().unwrap();
                let frame = frame_alloc().unwrap();
                ppn = frame.ppn;
                self.data_frames.insert(vpn.0, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.map_perm.bits).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }
    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        if self.map_type == MapType::Framed {
            self.data_frames.remove(&vpn.0);
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
            let dst = &mut page_table.translate(current_vpn).unwrap().ppn().slice_u8()
                [offset..offset + copy_len];
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

    fn add_direct_mapping_slice(&mut self, data: &[u8]) {
        assert_eq!(self.area_type, MapAreaType::ElfReadOnlyArea);
        let _data = unsafe { core::slice::from_raw_parts(data.as_ptr(), data.len()) };
        self.direct_mapping_slice = Some(_data);
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapAreaType {
    UserStack,
    KernelStack,
    ElfReadOnlyArea,
    ElfReadWriteArea,
    TrapContext,
    KernelSpaceArea,
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
