use core::arch::asm;
use core::sync::atomic::{AtomicUsize, Ordering};

use super::{TaskControlBlock, MAX_SIGNUM};
use super::{add_task, insert_into_tid2task, SigAction};
use crate::config::{is_aligned, FDMAX, MMAP_BASE, PAGE_SIZE, aligned_up};
use crate::fs::{FileClass, Stdin, Stdout};
use crate::mm::{
    translated_refmut, MapPermission, MemorySet, MmapArea, MmapFlags, VirtAddr, KERNEL_SPACE, VirtPageNum,
};
use crate::multicore::get_hartid;
use crate::syscall::CloneFlags;
use crate::task::{AuxHeader, AT_EXECFN, AT_NULL, AT_RANDOM};
use crate::trap::{trap_handler, TrapContext};
use alloc::string::String;
use alloc::sync::{Arc, Weak};
use alloc::vec;
use alloc::vec::Vec;
use spin::{Mutex, MutexGuard};
// use spin::Mutex;

pub struct ProcessControlBlock {
    pub pid: AtomicUsize,
    inner: Arc<Mutex<ProcessControlBlockInner>>,
}

pub struct ProcessControlBlockInner {
    pub is_zombie: bool,
    pub memory_set: MemorySet,
    pub parent: Option<Weak<ProcessControlBlock>>,
    pub children: Vec<Arc<ProcessControlBlock>>,
    pub exit_code: i32,
    pub fd_max: usize,
    pub fd_table: FdTable,
    pub sigactions: [SigAction; MAX_SIGNUM as usize],
    pub tasks: Vec<Option<Arc<TaskControlBlock>>>,
    pub cwd: String,
    pub user_heap_base: usize, // user heap
    pub user_heap_top: usize,
    pub mmap_area_top: usize, // mmap area
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

    pub fn getpid(&self) -> usize {
        self.pid.load(Ordering::Acquire)
    }

    pub fn new(elf_data: &[u8]) -> Arc<Self> {
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, ustack_base, entry_point, uheap_base, _) = MemorySet::from_elf(elf_data);
        // allocate a pid
        let process = Arc::new(Self {
            pid: AtomicUsize::new(0),
            inner: Arc::new(Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                memory_set,
                parent: None,
                children: Vec::with_capacity(10),
                exit_code: 0,
                fd_max: FDMAX,
                fd_table: vec![
                    // 0 -> stdin
                    Some(FileClass::Abs(Arc::new(Stdin))),
                    // 1 -> stdout
                    Some(FileClass::Abs(Arc::new(Stdout))),
                    // 2 -> stderr
                    Some(FileClass::Abs(Arc::new(Stdout))),
                ],
                sigactions: [SigAction::new(); MAX_SIGNUM as usize],
                tasks: Vec::with_capacity(10),
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
            -1,
            true,
        ));
        insert_into_tid2task(task.acquire_inner_lock().gettid(), Arc::clone(&task));

        // prepare trap_cx of main thread
        let task_inner = task.acquire_inner_lock();

        let trap_cx = task_inner.get_trap_cx();
        let ustack_top = task_inner.res.as_ref().unwrap().ustack_top();
        let kstack_top = task.kstack.get_top();

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
        // set pid
        process.pid.store(task_inner.gettid(), Ordering::Release);
        process_inner.tasks.push(Some(Arc::clone(&task)));

        drop(task_inner);
        drop(process_inner);

        // insert_into_pid2process(process.getpid(), Arc::clone(&process));
        // add main thread to scheduler
        add_task(task);
        process
    }

    /// Only support processes with a single thread.
    pub fn exec(self: &Arc<Self>, elf_data: &[u8], args: &Vec<String>) -> Option<Arc<TaskControlBlock>> {
        let mut inner = self.acquire_inner_lock();
        assert_eq!(inner.thread_count(), 1);
        // memory_set with elf program headers/trampoline/trap context/user stack
        let (memory_set, ustack_base, entry_point, uheap_base, mut auxv) =
            MemorySet::from_elf(elf_data);
        if ustack_base == 0 && entry_point == 0 && uheap_base == 0 {
            return None;
        }
        let new_token = memory_set.token();

        // substitute memory_set
        inner.memory_set = memory_set;

        // ****设置用户堆顶和mmap顶端位置****
        inner.user_heap_base = uheap_base;
        inner.user_heap_top = uheap_base;
        inner.mmap_area_top = MMAP_BASE;
        
        let task = inner.get_task(0);
        drop(inner);

        // then we alloc user resource for main thread again
        // since memory_set has been changed
        let mut task_inner = task.acquire_inner_lock();
        let res = task_inner.res.as_mut().unwrap();
        res.ustack_base = ustack_base;
        res.alloc_user_res();
        let trap_cx_ppn = res.trap_cx_ppn();
        let mut user_sp = res.ustack_top();
        drop(res);
        task_inner.trap_cx_ppn = trap_cx_ppn;

        ////////////// push env strings ///////////////////

        let mut envp: Vec<usize> = Vec::with_capacity(1);
        envp.push(0);
        let mut env: Vec<String> = Vec::with_capacity(30);
        // env.push(String::from("SHELL=/bin/sh"));
        // env.push(String::from("PWD=/"));
        // env.push(String::from("USER=root"));
        // env.push(String::from("MOTD_SHOWN=pam"));
        // env.push(String::from("LANG=C.UTF-8"));
        // env.push(String::from(
        //     "INVOCATION_ID=e9500a871cf044d9886a157f53826684",
        // ));
        // env.push(String::from("TERM=vt220"));
        // env.push(String::from("SHLVL=2"));
        // env.push(String::from("JOURNAL_STREAM=8:9265"));
        // env.push(String::from("OLDPWD=/root"));
        // env.push(String::from("_=busybox"));
        // env.push(String::from("LOGNAME=root"));
        // env.push(String::from("HOME=/"));
        env.push(String::from("PATH=/"));
        // env.push(String::from("LD_LIBRARY_PATH=/lib64"));
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
        ///////////// push argv strings ///////////////////
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
        // p = user_sp;
        auxv.push(AuxHeader {
            aux_type: AT_RANDOM,
            value: user_sp,
        });
        // for i in 0..0xf {
        //     *translated_refmut(new_token, p as *mut u8) = i as u8;
        //     p += 1;
        // }

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
        trap_cx.x[10] = 0;
        trap_cx.x[11] = argv_base;
        trap_cx.x[12] = envp_base;
        trap_cx.x[13] = auxv_base;
        *task_inner.get_trap_cx() = trap_cx;
        drop(task_inner);
        Some(task.clone())
    }

    /// Only support processes with a single thread.
    pub fn fork(self: &Arc<Self>, flags: CloneFlags, stack: usize, newtls: usize) -> Arc<Self> {
        let mut parent = self.acquire_inner_lock();
        assert_eq!(parent.thread_count(), 1);
        // clone parent's memory_set completely including trampoline/ustacks/trap_cxs
        // 复制trap_cx和ustack等内存区域均在这里
        // 因此后面不需要再allow_user_res了
        // todo cow
        // let memory_set = MemorySet::from_existed_user(&parent.memory_set);
        let memory_set = MemorySet::cow_from_existed_user(&mut parent.memory_set);
        // copy fd table
        let mut new_fd_table = Vec::with_capacity(1024);
        for fd in parent.fd_table.iter() {
            if let Some(file) = fd {
                new_fd_table.push(Some(file.clone()));
            } else {
                new_fd_table.push(None);
            }
        }

        // create child process pcb
        let child = Arc::new(Self {
            pid: AtomicUsize::new(0),
            inner: Arc::new(Mutex::new(ProcessControlBlockInner {
                is_zombie: false,
                memory_set,
                parent: Some(Arc::downgrade(self)),
                children: Vec::with_capacity(10),
                exit_code: 0,
                fd_max: FDMAX,
                fd_table: new_fd_table,
                sigactions: parent.sigactions.clone(),
                tasks: Vec::with_capacity(10),
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
            -1,
            // here we do not allocate trap_cx or ustack again
            // but mention that we allocate a new kstack here
            false,
        ));
        insert_into_tid2task(task.acquire_inner_lock().gettid(), Arc::clone(&task));

        // attach task to child process
        let mut child_inner = child.acquire_inner_lock();
        let task_inner = task.acquire_inner_lock();
        child.pid.store(task_inner.gettid(), Ordering::Relaxed);
        child_inner.tasks.push(Some(Arc::clone(&task)));
        drop(child_inner);
        // modify kstack_top in trap_cx of this thread
        let trap_cx = task_inner.get_trap_cx();
        trap_cx.kernel_sp = task.kstack.get_top();
        // sys_fork return value ...
        if stack != 0 {
            trap_cx.set_sp(stack);
        }

        trap_cx.x[10] = 0;
        if flags.contains(CloneFlags::CLONE_SETTLS) {
            trap_cx.x[4] = newtls;
        }

        drop(task_inner);
        // insert_into_pid2process(child.getpid(), Arc::clone(&child));
        // add this thread to scheduler
        add_task(task);
        child
    }

    pub fn clone_thread(
        self: &Arc<Self>,
        parent_task: Arc<TaskControlBlock>,
        flags: CloneFlags,
        stack: usize,
        newtls: usize,
    ) -> Arc<TaskControlBlock> {
        let pid = self.getpid();
        // only the main thread can create a sub-thread
        assert_eq!(parent_task.acquire_inner_lock().get_relative_tid(), 0);
        // create main thread of child process
        let task = Arc::new(TaskControlBlock::new(
            Arc::clone(self),
            parent_task
                .acquire_inner_lock()
                .res
                .as_ref()
                .unwrap()
                .ustack_base(),
            pid as isize,
            // mention that we allocate a new kstack / ustack / trap_cx here
            true,
        ));
        insert_into_tid2task(task.acquire_inner_lock().gettid(), Arc::clone(&task));

        // attach task to process
        let mut process_inner = self.acquire_inner_lock();
        let task_inner = task.acquire_inner_lock();
        let task_rel_tid = task_inner.get_relative_tid();
        let tasks = &mut process_inner.tasks;

        while tasks.len() < task_rel_tid + 1 {
            tasks.push(None);
        }
        tasks[task_rel_tid] = Some(Arc::clone(&task));
        let trap_cx = task_inner.get_trap_cx();

        // copy trap_cx from the parent thread
        *trap_cx = *parent_task.acquire_inner_lock().get_trap_cx();

        // modify kstack_top in trap_cx of this thread
        trap_cx.kernel_sp = task.kstack.get_top();
        // sys_fork return value ...
        if stack != 0 {
            trap_cx.set_sp(stack);
        }
        trap_cx.x[10] = 0;
        if flags.contains(CloneFlags::CLONE_SETTLS) {
            trap_cx.x[4] = newtls;
        }

        drop(task_inner);
        // add this thread to scheduler
        add_task(Arc::clone(&task));

        task
        // child
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
        // assert!(is_aligned(start) && is_aligned(len));
        let mut inner = self.acquire_inner_lock();
        let start = if start != 0 {
            aligned_up(start)
        }else {
            inner.mmap_area_top
        };
        let len = aligned_up(len);
        // assert_eq!(start, inner.mmap_area_top);

        let start_vpn = VirtAddr::from(start).floor();
        let end_vpn = VirtAddr::from(start + len).floor();
        let map_perm = MapPermission::from_bits((prot << 1) as u8).unwrap() | MapPermission::U;
        let mmap_flags = MmapFlags::from_bits(flags).unwrap();
        // TODO
        let mmap_fdone: crate::mm::FdOne; // = inner.fd_table[fd as usize].clone();
        if fd == -1 {
            // 转发到fd2, 标准错误输出
            mmap_fdone = inner.fd_table[2].clone();
        } else {
            mmap_fdone = inner.fd_table[fd as usize].clone();
        }
        let fixed = mmap_flags.contains(MmapFlags::MAP_FIXED);
        // println!("mmap_flags: {:#?} , flags: 0x{:x}",mmap_flags,flags);

        if !fixed {
            // 一般情况mmap,注意，此处不判断fd是否有效
            inner.memory_set.push_mmap_area(MmapArea::new(
                start_vpn,
                end_vpn,
                map_perm,
                flags,
                mmap_fdone,
                fd as usize,
                offset,
            ));
        } else {
            // fixed 区域
            //TODO 可能有部分区间重叠情况考虑不到位
            // println!("fixed handle start ...");
            // println!("[new mmap in] start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
            // let mut collision = false;
            let mut old_perm = MapPermission::U;
            let mut old_start = VirtAddr::from(0).floor();
            let mut old_end = VirtAddr::from(0).floor();
            let mut old_flags = 0usize;
            let mut old_fdone = mmap_fdone.clone();
            let mut old_fd = 0usize;
            let mut old_offset = 0usize;
            loop {
                let mut loop_flag = true;
                // let mut index = 0;
                // for (i,mmap_area) in inner.memory_set.mmap_areas.iter().enumerate(){
                for mmap_area in inner.memory_set.mmap_areas.iter() {
                    // 在此处提取old_area相关信息
                    // 1                  1
                    // fix area        |----- - -
                    // old area           |----|
                    // 3                    3
                    // fix area          |-- - - -
                    // old area        |-----|
                    if (start_vpn < mmap_area.vpn_range.get_start() && end_vpn > mmap_area.vpn_range.get_start())
                        || (start_vpn >= mmap_area.vpn_range.get_start() && start_vpn < mmap_area.vpn_range.get_end())
                    {
                        // index = i;
                        old_perm = mmap_area.map_perm;
                        old_start = mmap_area.vpn_range.get_start();
                        old_end = mmap_area.vpn_range.get_end();
                        old_flags = mmap_area.flags;
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
                    old_offset = old_offset
                        + ((len + start - u_old_start + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE;
                    old_start = VirtAddr::from(start + len).ceil();
                    // println!("[part-1]fixed situation 1  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                    // println!("[part-2]fixed situation 1  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                    inner.memory_set.push_mmap_area(MmapArea::new(
                        old_start,
                        old_end,
                        old_perm,
                        old_flags,
                        old_fdone.clone(),
                        old_fd,
                        old_offset,
                    ));
                } else
                // fix area        |----------|
                // old area           |----|
                // 刚好完全覆盖的情况也在此处
                if start_vpn <= old_start && end_vpn >= old_end {
                    // println!("fixed situation 2");
                    // println!("[part-2]fixed situation 2  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                } else
                // fix area          |--|
                // old area        |-----|
                if start_vpn >= old_start && end_vpn <= old_end {
                    // println!("fixed situation 3");
                    if end_vpn != old_end {
                        // 向上取整页
                        let u_old_start: usize = old_start.into();
                        let part3_offset = old_offset
                            + ((len + start - u_old_start + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE;
                        let part3_start = VirtAddr::from(start + len).ceil();
                        let part3_end = old_end;
                        let part3_perm = old_perm;
                        let part3_flags = old_flags;
                        let part3_fd = old_fd;
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
                        old_end = VirtAddr::from(start + PAGE_SIZE - 1).floor();
                        // println!("[part-1]fixed situation 3  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                        inner.memory_set.push_mmap_area(MmapArea::new(
                            old_start,
                            old_end,
                            old_perm,
                            old_flags,
                            old_fdone.clone(),
                            old_fd,
                            old_offset,
                        ));
                    }
                } else
                // fix area          |-------|
                // old area        |-----|
                if start_vpn > old_start && end_vpn > old_end {
                    // println!("fixed situation 4");
                    // 原区域作为第一段
                    old_end = VirtAddr::from(start + PAGE_SIZE - 1).floor();
                    // println!("[part-1]fixed situation 4  start_vpn:{:#?}  end_vpn:{:#?}",old_start,old_end);
                    // println!("[part-2]fixed situation 4  start_vpn:{:#?}  end_vpn:{:#?}",start_vpn,end_vpn);
                    inner.memory_set.push_mmap_area(MmapArea::new(
                        old_start,
                        old_end,
                        old_perm,
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
        if inner.mmap_area_top < VirtAddr::from(end_vpn).0 {
            inner.mmap_area_top = VirtAddr::from(end_vpn).0;
        }

        start as isize
    }

    pub fn munmap(&self, start: usize, _len: usize) -> isize {
        // assert!(is_aligned(start));
        let mut inner = self.acquire_inner_lock();
        let start_vpn = VirtAddr::from(start).floor();
        inner.memory_set.remove_mmap_area_with_start_vpn(start_vpn)
    }
}

impl ProcessControlBlockInner {
    fn lazy_alloc_mmap_page(&mut self, vaddr: usize) -> isize {
        let vpn = VirtAddr::from(vaddr).floor();
        self.memory_set.insert_mmap_dataframe(vpn)
    }

    fn lazy_alloc_heap_page(&mut self, vaddr: usize) -> isize {
        // println!("lazy_alloc_heap_page({:#x?})", vaddr);
        let user_heap_base = self.user_heap_base;
        let user_heap_top = self.user_heap_top;
        self.memory_set
            .insert_heap_dataframe(vaddr, user_heap_base, user_heap_top)
    }
    
    #[inline(always)]
    pub fn check_lazy(&mut self, vaddr: usize, is_load: bool) -> isize {
        if vaddr == 0 {
            error!("Assertion failed in user space");
            return -1;
        }
        let heap_base = self.user_heap_base;
        let heap_top = self.user_heap_top;
        let mmap_top = self.mmap_area_top;
        let mut ret:isize = 0;
        if is_load {
            if vaddr >= heap_base && vaddr < heap_top {
                    // println!("[kernel] lazy_alloc heap memory {:#x?}", vaddr);
                    // println!("is_load? {:#x?}", is_load);
                    ret = self.lazy_alloc_heap_page(vaddr);
                } else if vaddr >= MMAP_BASE && vaddr < mmap_top {
                    // println!("[kernel] lazy_alloc mmap memory {:#x?}", vaddr);
                    // println!("is_load? {:#x?}", is_load);
                    ret = self.lazy_alloc_mmap_page(vaddr);
                } else {
                    ret = -1;
                }
        }else {
            let vaddr_n: VirtAddr = vaddr.into();
            let vpn: VirtPageNum = vaddr_n.floor();
            if let Some(pte) = self.memory_set.translate(vpn) {
                if pte.is_cow() && pte.is_valid() {
                    // cow_alloc(vpn, former_ppn);
                    let former_ppn = pte.ppn();
                    self.memory_set.cow_alloc(vpn, former_ppn, vaddr >= heap_base && vaddr < heap_top);
                    ret = 0;
                }else if !pte.is_valid() {
                    if vaddr >= heap_base && vaddr < heap_top {
                        // println!("[kernel] lazy_alloc heap memory {:#x?}", vaddr);
                        // println!("is_load? {:#x?}", is_load);
                        ret = self.lazy_alloc_heap_page(vaddr);
                    } else if vaddr >= MMAP_BASE && vaddr < mmap_top {
                        // println!("[kernel] lazy_alloc mmap memory {:#x?}", vaddr);
                        // println!("is_load? {:#x?}", is_load);
                        ret = self.lazy_alloc_mmap_page(vaddr);
                    } else {
                        ret = -1;
                    }
                }else {
                    // error!("lazy cow erro , find pte but ...");
                    ret = -1;
                }  
            }else {
                if vaddr >= heap_base && vaddr < heap_top {
                    // println!("[kernel] lazy_alloc heap memory {:#x?}", vaddr);
                    // println!("is_load? {:#x?}", is_load);
                    ret = self.lazy_alloc_heap_page(vaddr);
                } else if vaddr >= MMAP_BASE && vaddr < mmap_top {
                    // println!("[kernel] lazy_alloc mmap memory {:#x?}", vaddr);
                    // println!("is_load? {:#x?}", is_load);
                    ret = self.lazy_alloc_mmap_page(vaddr);
                } else {
                    ret = -1;
                }
            }
        }

        if ret == 0 {
            unsafe {
                asm!("sfence.vma");
                asm!("fence.i");
            }
        }
        ret
    }
}
