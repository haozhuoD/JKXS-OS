use super::id::RecycleAllocator;
use super::manager::insert_into_pid2process;
use super::signal::SigInfo;
use super::add_task;
use super::{pid_alloc, PidHandle};
use super::TaskControlBlock;
use crate::config::{is_aligned, MMAP_BASE, PAGE_SIZE};
use crate::fs::{FileClass, Stdin, Stdout};
use crate::mm::{translated_refmut, MapPermission, MemorySet, MmapArea, VirtAddr, KERNEL_SPACE, MmapFlags};
use crate::multicore::get_hartid;
use crate::task::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM};
use crate::trap::{trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};
// use spin::Mutex;

pub struct ProcessControlBlock {
    // immutable
    pub pid: PidHandle,
    // mutable
    inner: Arc<Mutex<ProcessControlBlockInner>>,
}

pub struct ProcessControlBlockInner {
    pub is_zombie: bool,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: i32,
    pub fd_table: FdTable,
    pub signal_info: SigInfo,
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub task_res_allocator: RecycleAllocator,
    pub cwd: String,
    pub user_heap_base: usize, // user heap
    pub user_heap_top: usize,
    pub mmap_area_top: usize,  // mmap area
}

pub type FdTable = Vec<Option<FileClass>>;
pub type ProcessInnerLock<'a> = MutexGuard<'a, ProcessControlBlockInner>;

impl ProcessControlBlockInner {
    #[allow(unused)]
    pub fn get_user_token(&self) -> usize {
        self.memory_set.token()
    }

    pub fn alloc_fd(&mut self, minfd: usize) -> usize {
        let mut i = minfd;
        loop {
            while i >= self.fd_table.len() {
                self.fd_table.push(None);
            }
            if self.fd_table[i].is_none() {
                return i;
            }
            i += 1;
        }
    }

    pub fn alloc_tid(&mut self) -> usize {
        self.task_res_allocator.alloc()
    }

    pub fn dealloc_tid(&mut self, tid: usize) {
        self.task_res_allocator.dealloc(tid)
    }

    pub fn thread_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn get_task(&self, tid: usize) -> Arc<TaskControlBlock> {
        self.tasks[tid].as_ref().unwrap().clone()
    }
}

impl ProcessControlBlock {
    pub fn acquire_inner_lock(&self) -> ProcessInnerLock {
        self.inner.lock()
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, ustack_base, entry_point, uheap_base, _) = MemorySet::from_elf(elf_data);
        // allocate a pid
        let pid_handle = pid_alloc();
        let process = Arc::new(Self {
            pid: pid_handle,
            inner: Arc::new(Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                memory_set,
                parent: None,
                children: Vec::new(),
                exit_code: 0,
                fd_table: vec![
                    // 0 -> stdin
                    Some(FileClass::Abs(Arc::new(Stdin))),
                    // 1 -> stdout
                    Some(FileClass::Abs(Arc::new(Stdout))),
                    // 2 -> stderr
                    Some(FileClass::Abs(Arc::new(Stdout))),
                ],
                signal_info: SigInfo::new(),
                tasks: Vec::new(),
                task_res_allocator: RecycleAllocator::new(),
                cwd: String::from("/"),
                user_heap_base: uheap_base,
                user_heap_top: uheap_base,
                mmap_area_top: MMAP_BASE,
            })),
        });
        // create a main thread, we should allocate ustack and trap_cx here
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&process),
            ustack_base,
            true,
        ));
        // prepare trap_cx of main thread
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        let ustack_top = task_inner.res.as_ref().unwrap().ustack_top();
        let kstack_top = task.kstack.get_top();
        drop(task_inner);
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            ustack_top,
            KERNEL_SPACE.read().token(),
            kstack_top,
            trap_handler as usize,
            get_hartid(),
        );
        // add main thread to the process
        let mut process_inner = process.acquire_inner_lock();
        process_inner.tasks.push(Some(Arc::clone(&task)));
        drop(process_inner);
        insert_into_pid2process(process.getpid(), Arc::clone(&process));
        // add main thread to scheduler
        add_task(task);
        process
    }

    /// Only support processes with a single thread.
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: Vec<String>) -> isize {
        assert_eq!(self.acquire_inner_lock().thread_count(), 1);
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, ustack_base, entry_point, uheap_base, mut auxv) =
            MemorySet::from_elf(elf_data);
        if (ustack_base==0) && (entry_point==0) && (uheap_base==0) {
            return -1;
        }
        let new_token = memory_set.token();
        // substitute memory_set
        self.acquire_inner_lock().memory_set = memory_set;

        // ****设置用户堆顶和mmap顶端位置****
        self.acquire_inner_lock().user_heap_base = uheap_base;
        self.acquire_inner_lock().user_heap_top = uheap_base;
        self.acquire_inner_lock().mmap_area_top = MMAP_BASE;
        // then we alloc user resource for main thread again
        // since memory_set has been changed
        let task = self.acquire_inner_lock().get_task(0);
        let mut task_inner = task.acquire_inner_lock();
        task_inner.res.as_mut().unwrap().ustack_base = ustack_base;
        task_inner.res.as_mut().unwrap().alloc_user_res();
        task_inner.trap_cx_ppn = task_inner.res.as_mut().unwrap().trap_cx_ppn();

        let mut user_sp = task_inner.res.as_mut().unwrap().ustack_top();

        ////////////// envp[] ///////////////////
        let mut env: Vec<String> = Vec::new();
        env.push(String::from("SHELL=/user_shell"));
        env.push(String::from("PWD=/"));
        env.push(String::from("USER=root"));
        env.push(String::from("MOTD_SHOWN=pam"));
        env.push(String::from("LANG=C.UTF-8"));
        env.push(String::from(
            "INVOCATION_ID=e9500a871cf044d9886a157f53826684",
        ));
        env.push(String::from("TERM=vt220"));
        env.push(String::from("SHLVL=2"));
        env.push(String::from("JOURNAL_STREAM=8:9265"));
        env.push(String::from("OLDPWD=/root"));
        env.push(String::from("_=busybox"));
        env.push(String::from("LOGNAME=root"));
        env.push(String::from("HOME=/"));
        env.push(String::from("PATH=/"));
        env.push(String::from("LD_LIBRARY_PATH=/"));
        let mut envp: Vec<usize> = (0..=env.len()).collect();
        envp[env.len()] = 0;

        for i in 0..env.len() {
            user_sp -= env[i].len() + 1;
            envp[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in env[i].as_bytes() {
                *translated_refmut(new_token, p as *mut u8) = *c;
                p += 1;
            }
            *translated_refmut(new_token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// argv[] ///////////////////
        let mut argv: Vec<usize> = (0..=args.len()).collect();
        argv[args.len()] = 0;
        for i in 0..args.len() {
            user_sp -= args[i].len() + 1;
            // println!("user_sp {:X}", user_sp);
            argv[i] = user_sp;
            let mut p = user_sp;
            // write chars to [user_sp, user_sp + len]
            for c in args[i].as_bytes() {
                *translated_refmut(new_token, p as *mut u8) = *c;
                // print!("({})",*c as char);
                p += 1;
            }
            *translated_refmut(new_token, p as *mut u8) = 0;
        }
        // make the user_sp aligned to 8B for k210 platform
        user_sp -= user_sp % core::mem::size_of::<usize>();

        ////////////// platform String ///////////////////
        let platform = "RISC-V64";
        user_sp -= platform.len() + 1;
        user_sp -= user_sp % core::mem::size_of::<usize>();
        let mut p = user_sp;
        for c in platform.as_bytes() {
            *translated_refmut(new_token, p as *mut u8) = *c;
            p += 1;
        }
        *translated_refmut(new_token, p as *mut u8) = 0;

        ////////////// rand bytes ///////////////////
        user_sp -= 16;
        p = user_sp;
        auxv.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });
        for i in 0..0xf {
            *translated_refmut(new_token, p as *mut u8) = i as u8;
            p += 1;
        }

        ////////////// padding //////////////////////
        user_sp -= user_sp % 16;

        ////////////// auxv[] //////////////////////
        auxv.push(AuxHeader {
            aux_type: AT_EXECFN,
            value: argv[0],
        }); // file name
        auxv.push(AuxHeader {
            aux_type: AT_NULL,
            value: 0,
        }); // end
        user_sp -= auxv.len() * core::mem::size_of::<AuxHeader>();
        let auxv_base = user_sp;
        // println!("[auxv]: base 0x{:X}", auxv_base);
        for i in 0..auxv.len() {
            // println!("[auxv]: {:?}", auxv[i]);
            let addr = user_sp + core::mem::size_of::<AuxHeader>() * i;
            *translated_refmut(new_token, addr as *mut usize) = auxv[i].aux_type;
            *translated_refmut(
                new_token,
                (addr + core::mem::size_of::<usize>()) as *mut usize,
            ) = auxv[i].value;
        }

        ////////////// *envp [] //////////////////////
        user_sp -= (env.len() + 1) * core::mem::size_of::<usize>();
        let envp_base = user_sp;
        *translated_refmut(
            new_token,
            (user_sp + core::mem::size_of::<usize>() * (env.len())) as *mut usize,
        ) = 0;
        for i in 0..env.len() {
            *translated_refmut(
                new_token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = envp[i];
        }

        ////////////// *argv [] //////////////////////
        user_sp -= (args.len() + 1) * core::mem::size_of::<usize>();
        let argv_base = user_sp;
        *translated_refmut(
            new_token,
            (user_sp + core::mem::size_of::<usize>() * (args.len())) as *mut usize,
        ) = 0;
        for i in 0..args.len() {
            *translated_refmut(
                new_token,
                (user_sp + core::mem::size_of::<usize>() * i) as *mut usize,
            ) = argv[i];
        }

        ////////////// argc //////////////////////
        user_sp -= core::mem::size_of::<usize>();
        *translated_refmut(new_token, user_sp as *mut usize) = args.len();

        // initialize trap_cx
        let mut trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.read().token(),
            task.kstack.get_top(),
            trap_handler as usize,
            get_hartid(),
        );
        trap_cx.x[10] = args.len();
        trap_cx.x[11] = argv_base;
        trap_cx.x[12] = envp_base;
        trap_cx.x[13] = auxv_base;
        *task_inner.get_trap_cx() = trap_cx;
        0
    }

    /// Only support processes with a single thread.
    pub fn fork(self: &Arc<Self>, _flags: u32, stack: usize) -> Arc<Self> {
        let mut parent = self.acquire_inner_lock();
        assert_eq!(parent.thread_count(), 1);
        // clone parent's memory_set completely including trampoline/ustacks/trap_cxs
        // 复制trapframe等内存区域均在这里
        let memory_set = MemorySet::from_existed_user(&parent.memory_set);
        // alloc a pid
        let pid = pid_alloc();
        // copy fd table
        let mut new_fd_table = Vec::new();
        for fd in parent.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }
        // copy signal-info
        let mut new_signal_info = parent.signal_info.clone();
        new_signal_info.pending_signals.clear();

        // create child process pcb
        let child = Arc::new(Self {
            pid,
            inner: Arc::new(Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                memory_set,
                parent: Some(Arc::downgrade(self)),
                children: Vec::new(),
                exit_code: 0,
                fd_table: new_fd_table,
                signal_info: new_signal_info,
                tasks: Vec::new(),
                task_res_allocator: RecycleAllocator::new(),
                cwd: parent.cwd.clone(),
                user_heap_base: parent.user_heap_base,
                user_heap_top: parent.user_heap_top,
                mmap_area_top: parent.mmap_area_top,
            })),
        });
        // add child
        parent.children.push(Arc::clone(&child));

        // create main thread of child process
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(&child),
            parent
                .get_task(0)
                .acquire_inner_lock()
                .res
                .as_ref()
                .unwrap()
                .ustack_base(),
            // here we do not allocate trap_cx or ustack again
            // but mention that we allocate a new kstack here
            false,
        ));
        // attach task to child process
        let mut child_inner = child.acquire_inner_lock();
        child_inner.tasks.push(Some(Arc::clone(&task)));
        drop(child_inner);
        // modify kstack_top in trap_cx of this thread
        let task_inner = task.acquire_inner_lock();
        let trap_cx = task_inner.get_trap_cx();
        trap_cx.kernel_sp = task.kstack.get_top();
        // sys_fork return value ...
        if stack != 0 {
            trap_cx.set_sp(stack);
        }
        trap_cx.x[10] = 0;
        drop(task_inner);
        insert_into_pid2process(child.getpid(), Arc::clone(&child));
        // add this thread to scheduler
        add_task(task);
        child
    }

    /// 插入一个mmap区域（此时尚未实际分配数据页），并更新进程mmap顶部位置
    pub fn mmap(
        &self,
        start: usize,
        len: usize,
        prot: usize,
        flags: usize,
        fd: isize,
        offset: usize,
    ) -> isize {
        // `flags` field unimplemented
        // 目前mmap区域只能不断向上增长，无回收重整内存
        // 目前不检查fd是否合法
        assert!(is_aligned(start) && is_aligned(len));
        let mut inner = self.acquire_inner_lock();
        // assert_eq!(start, inner.mmap_area_top);

        let start_vpn = VirtAddr::from(start).floor();
        let end_vpn = VirtAddr::from(start + len).floor();
        let map_perm = MapPermission::from_bits((prot << 1) as u8).unwrap() | MapPermission::U;
        let mmap_flags = MmapFlags::from_bits(flags).unwrap();
        let mmap_fdone = inner.fd_table[fd as usize].clone();
        let fixed = mmap_flags.contains(MmapFlags::MAP_FIXED);
        // println!("mmap_flags: {:#?} , flags: 0x{:x}",mmap_flags,flags);

        if !fixed { // 一般情况mmap,注意，此处不判断fd是否有效
            inner.memory_set.push_mmap_area(MmapArea::new(
                start_vpn,
                end_vpn,
                map_perm,
                flags,
                mmap_fdone,
                fd as usize,
                offset,
            ));
        } else {   // fixed 区域
            //TODO 可能有部分区间重叠情况考虑不到位
            // println!("fixed handle start ...");
            // println!("[new mmap in] start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
            // let mut collision = false;
            let mut old_perm = MapPermission::U;
            let mut old_start = VirtAddr::from(0).floor();
            let mut old_end = VirtAddr::from(0).floor();
            let mut old_flags= 0usize;
            let mut old_fdone =mmap_fdone.clone();
            let mut old_fd= 0usize;
            let mut old_offset = 0usize;
            loop {
                let mut loop_flag = true;
                // let mut index = 0;
                // for (i,mmap_area) in inner.memory_set.mmap_areas.iter().enumerate(){
                for mmap_area in inner.memory_set.mmap_areas.iter(){
                    // 在此处提取old_area相关信息
                    // 1                  1          
                    // fix area        |----- - -    
                    // old area           |----|     
                    // 3                    3               
                    // fix area          |-- - - -        
                    // old area        |-----|  
                    if (start_vpn < mmap_area.start_vpn && end_vpn > mmap_area.start_vpn) 
                            ||(start_vpn >= mmap_area.start_vpn && start_vpn < mmap_area.end_vpn) {
                        // index = i;
                        old_perm = mmap_area.map_perm;
                        old_start= mmap_area.start_vpn;
                        old_end = mmap_area.end_vpn;
                        old_flags= mmap_area.flags;
                        old_offset = mmap_area.offset;
                        old_fdone = mmap_area.fd_one.clone();
                        old_fd = mmap_area.fd;
                        loop_flag = false;
                        // collision = true;
                    }            
                }

                if loop_flag {
                    // println!("break ...");
                    break;
                }
                // println!("fixed handle real start ...");
                inner.memory_set.remove_mmap_area_with_start_vpn(old_start);
                // fix area        |-----|   
                // old area           |----| 
                if start_vpn <= old_start && end_vpn > old_start && end_vpn < old_end {
                    // println!("fixed situation 1");
                    let u_old_start: usize = old_start.into();
                    // 向上取整页
                    old_offset = old_offset + ( (len+start- u_old_start +PAGE_SIZE -1) / PAGE_SIZE )*PAGE_SIZE; 
                    old_start = VirtAddr::from(start + len).ceil();
                    // println!("[part-1]fixed situation 1  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                    // println!("[part-2]fixed situation 1  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                    inner.memory_set.push_mmap_area(MmapArea::new(
                        old_start,
                        old_end ,
                        old_perm  ,
                        old_flags,
                        old_fdone.clone(),
                        old_fd,
                        old_offset,
                    ));
                }else 
                // fix area        |----------|   
                // old area           |----| 
                // 刚好完全覆盖的情况也在此处
                if start_vpn <= old_start && end_vpn >= old_end {
                    // println!("fixed situation 2");
                    // println!("[part-2]fixed situation 2  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                    
                }else
                // fix area          |--|        
                // old area        |-----|   
                if start_vpn >= old_start && end_vpn <= old_end {
                    // println!("fixed situation 3");
                    if end_vpn != old_end{
                        // 向上取整页
                        let u_old_start: usize = old_start.into();
                        let part3_offset = old_offset+( (len+start-u_old_start +PAGE_SIZE -1) / PAGE_SIZE )*PAGE_SIZE; 
                        let part3_start = VirtAddr::from(start + len).ceil();
                        let part3_end= old_end;
                        let part3_perm = old_perm;
                        let part3_flags= old_flags;
                        let part3_fd= old_fd;
                        // println!("[part-3]fixed situation 3  start_vpn:{:#?}  end_vpn:{:#?}",part3_start,part3_end);
                        inner.memory_set.push_mmap_area(MmapArea::new(
                            part3_start,
                            part3_end,
                            part3_perm,
                            part3_flags,
                            old_fdone.clone(),
                            part3_fd,
                            part3_offset,
                        ));
                    } 
                
                    // println!("[part-2]fixed situation 3  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);      
                    if start_vpn != old_start {
                        // 原区域作为第一段
                        old_end = VirtAddr::from(start+PAGE_SIZE-1).floor();
                        // println!("[part-1]fixed situation 3  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                        inner.memory_set.push_mmap_area(MmapArea::new(
                            old_start,
                            old_end ,
                            old_perm  ,
                            old_flags,
                            old_fdone.clone(),
                            old_fd,
                            old_offset,
                        ));
                    }                    
                }else
                // fix area          |-------|        
                // old area        |-----|  
                if start_vpn > old_start && end_vpn > old_end {
                    // println!("fixed situation 4");
                    // 原区域作为第一段
                    old_end = VirtAddr::from(start+PAGE_SIZE-1).floor();
                    // println!("[part-1]fixed situation 4  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                    // println!("[part-2]fixed situation 4  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                    inner.memory_set.push_mmap_area(MmapArea::new(
                        old_start,
                        old_end ,
                        old_perm  ,
                        old_flags,
                        old_fdone.clone(),
                        old_fd,
                        old_offset,
                    ));   
                }
            }
            inner.memory_set.push_mmap_area(MmapArea::new(
                start_vpn,
                end_vpn,
                map_perm,
                flags,
                mmap_fdone,
                fd as usize,
                offset,
            ));
            // if !collision {
            //     println!("[mmap-fixed handle] no collision start_vpn:{:#?}  end_vpn:{:#?}  perm :{:#?}",start_vpn,end_vpn,map_perm);
            // }
        }
        // 维护最高mmap区域地址值
        if inner.mmap_area_top < VirtAddr::from(end_vpn).0{
            inner.mmap_area_top = VirtAddr::from(end_vpn).0;
        }
        
        start as isize
    }

    pub fn munmap(&self, start: usize, _len: usize) -> isize {
        assert!(is_aligned(start));
        let mut inner = self.acquire_inner_lock();
        let start_vpn = VirtAddr::from(start).floor();
        inner.memory_set.remove_mmap_area_with_start_vpn(start_vpn)
    }

    pub fn getpid(&self) -> usize {
        self.pid.0
    }
}
