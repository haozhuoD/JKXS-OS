# multicore&并发控制

## QEMU启动多核
```shell
# 启动双核
make run CPUS=2
```

## 如何用gdb调试多核

https://stackoverflow.com/questions/42800801/how-to-use-gdb-to-debug-qemu-with-smp-symmetric-multiple-processors

```
i th 查看线程信息
thr x 切换到x号线程
```

## 初步尝试

将`UPSafeCell`全部替换为`Mutex`，并为每个processor设置自己的全局变量.观察到现象：cpu0正常启动，轮到cpu1运行任务时会报错：no tasks available in run_tasks。
之后内核疯狂`panic`，输出字母顺序是乱的。

## console加锁

结果是cpu0直接卡死（卡在console那里），cpu1由于cpu0未初始化完成也卡死。
用`Mutex<ConsoleInner>`代替`Arc<Mutex<ConsoleInner>>`，问题解决。我也不知道为啥（现在知道了，原因是堆内存未初始化）

## 三核（四核）出现跑飞

现象:程序跑飞，程序运行到奇怪位置。随后内核进入死锁状态，

**注意: `b trap_from_kernel` 是一个非常好的调试方法!!**
> 程序跑飞而导致Instruction Page Fault（已解决）

> 这个死锁的直接结果就是在内核因为ra异常，程序运行在了没有设定的地址范围内。在排查之后，发现间接原因是task_cx被修改了。然后，使用gdb抓取出现Instruction Page Fault的时刻，也就是进入trap_from_kernel的时候，发现两个核使用的都是一个内核栈，而这并不符合内核的设定。正常来说，内核运行的栈严格和当前处理器核运行进程的pid绑定，或者转入idle_cx也就是初始栈，但这仍然不可能出现两个核处在相同的栈。结合task_cx的问题，我们推测是栈的切换过程之中出现了空隙，使得这种异常现象得以发生。

> 最后，我们发现，是切换的时候锁的控制出现了空隙。在内核进行进程切换的时候，先将当前进程放入空闲进程队列，再从队列中寻找下一个进程，这使得该核在使用该进程栈的同时，却有可能将该进程放手给了另一个核。

> 因此我们必须强制使得每个核必须手握至少一个进程的锁，也就是先找到下一个可以切换的进程，再将当前进程放入队列中。但是显然，这会造成死锁，所以我们采用了非常巧妙的方法，那就是：如果当前没有下一个进程，就不进行进程切换，返回原来的进程。这样的让出争抢锁的方式避免了死锁的发生可能。

> 具体操作我们还遇到了一些小问题：
> - cpu运行第一个程序的时候核自身没有程序，这个时候不能“返回原来的进程”，要特殊判断，持续争抢下一个进程。
> - 注意手动释放锁
>- loop块的作用域等等细节实现问题

## 多核退出竞争问题

开三核测试，发现执行用户程序时经常会产生死锁，下面利用gdb调试工具分析某一次死锁出现的情况（此时usershell刚执行完mmap用户程序，mmap准备退出时发生死锁）。先打印堆栈信息，如下：

**核心1：**
```shell
(gdb) bac
#0  0x0000000080218162 in core::sync::atomic::AtomicUsize::load (self=0x806bb218 <os::mm::heap_allocator::HEAP_SPACE+4182552>,
    order=core::sync::atomic::Ordering::Acquire)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/sync/atomic.rs:1503
#1  0x000000008023e936 in spin::mutex::ticket::TicketMutex<T>::lock (self=0x806bb210 <os::mm::heap_allocator::HEAP_SPACE+4182544>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex/ticket.rs:161
#2  spin::mutex::Mutex<T>::lock (self=0x806bb210 <os::mm::heap_allocator::HEAP_SPACE+4182544>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex.rs:174
#3  os::task::process::ProcessControlBlock::inner_exclusive_access (self=0x806ba630 <os::mm::heap_allocator::HEAP_SPACE+4179504>)
    at src/task/process.rs:82
#4  0x0000000080240f08 in os::syscall::process::sys_waitpid::{{closure}} () at src/syscall/process.rs:108
#5  0x0000000080235a0a in core::iter::traits::iterator::Iterator::find::check::{{closure}} (x=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:2455
#6  0x00000000802141c8 in <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold::enumerate::{{closure}} (acc=(), item=0x806ba840 <os::mm::heap_allocator::HEAP_SPACE+4180032>)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/adapters/enumerate.rs:85
#7  0x0000000080214cc4 in core::iter::traits::iterator::Iterator::try_fold (self=0xffffffffffff9be0, init=(), f=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:1991
#8  0x0000000080213fb8 in <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold (
    self=0xffffffffffff9be0, init=(), fold=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/adapters/enumerate.rs:91
#9  0x0000000080214438 in core::iter::traits::iterator::Iterator::find (self=0xffffffffffff9be0, predicate=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:2459
#10 0x000000008021a9f6 in os::syscall::process::sys_waitpid (pid=2, wstatus=0xf0003e88, options=1) at src/syscall/process.rs:106
#11 0x000000008023d534 in os::syscall::syscall (syscall_id=260, args=...) at src/syscall/mod.rs:118
#12 0x00000000802345a8 in os::trap::trap_handler () at src/trap/mod.rs:53
#13 0x0000000000010cc8 in ?? ()
```

**核心2：**
```shell
#0  0x0000000080217cc8 in core::hint::spin_loop () at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/hint.rs:137
#1  0x00000000802180fc in core::sync::atomic::spin_loop_hint ()
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/sync/atomic.rs:2807
#2  0x000000008023e948 in spin::mutex::ticket::TicketMutex<T>::lock (self=0x806ba410 <os::mm::heap_allocator::HEAP_SPACE+4178960>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex/ticket.rs:162
#3  spin::mutex::Mutex<T>::lock (self=0x806ba410 <os::mm::heap_allocator::HEAP_SPACE+4178960>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex.rs:174
#4  os::task::process::ProcessControlBlock::inner_exclusive_access (self=0x806bc650 <os::mm::heap_allocator::HEAP_SPACE+4187728>)
    at src/task/process.rs:82
#5  0x0000000080240f08 in os::syscall::process::sys_waitpid::{{closure}} () at src/syscall/process.rs:108
#6  0x0000000080235a0a in core::iter::traits::iterator::Iterator::find::check::{{closure}} (x=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:2455
#7  0x00000000802141c8 in <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold::enumerate::{{closure}} (acc=(), item=0x806b81e0 <os::mm::heap_allocator::HEAP_SPACE+4170208>)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/adapters/enumerate.rs:85
#8  0x0000000080214cc4 in core::iter::traits::iterator::Iterator::try_fold (self=0xffffffffffffebe0, init=(), f=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:1991
#9  0x0000000080213fb8 in <core::iter::adapters::enumerate::Enumerate<I> as core::iter::traits::iterator::Iterator>::try_fold (
    self=0xffffffffffffebe0, init=(), fold=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/adapters/enumerate.rs:91
#10 0x0000000080214438 in core::iter::traits::iterator::Iterator::find (self=0xffffffffffffebe0, predicate=...)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/iter/traits/iterator.rs:2459
#11 0x000000008021a9f6 in os::syscall::process::sys_waitpid (pid=-1, wstatus=0xf0003f6c, options=1) at src/syscall/process.rs:106
#12 0x000000008023d534 in os::syscall::syscall (syscall_id=260, args=...) at src/syscall/mod.rs:118
#13 0x00000000802345a8 in os::trap::trap_handler () at src/trap/mod.rs:53
#14 0x0000000000010156 in ?? ()
```

**核心3：**·
```shell
(gdb) bac
#0  core::sync::atomic::atomic_load (dst=0x806b8218 <os::mm::heap_allocator::HEAP_SPACE+4170264>,
    order=core::sync::atomic::Ordering::Relaxed)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/sync/atomic.rs:2360
#1  0x0000000080218162 in core::sync::atomic::AtomicUsize::load (self=0x806b8218 <os::mm::heap_allocator::HEAP_SPACE+4170264>,
    order=core::sync::atomic::Ordering::Acquire)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/sync/atomic.rs:1503
#2  0x000000008023e936 in spin::mutex::ticket::TicketMutex<T>::lock (self=0x806b8210 <os::mm::heap_allocator::HEAP_SPACE+4170256>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex/ticket.rs:161
#3  spin::mutex::Mutex<T>::lock (self=0x806b8210 <os::mm::heap_allocator::HEAP_SPACE+4170256>)
    at /home/user/.cargo/registry/src/mirrors.tuna.tsinghua.edu.cn-df7c3c540f42cdbd/spin-0.7.1/src/mutex.rs:174
#4  os::task::process::ProcessControlBlock::inner_exclusive_access (self=0x806b8130 <os::mm::heap_allocator::HEAP_SPACE+4170032>)
    at src/task/process.rs:82
#5  0x000000008022e2b8 in os::task::exit_current_and_run_next (exit_code=0) at src/task/mod.rs:91
#6  0x000000008021a1a0 in os::syscall::process::sys_exit (exit_code=0) at src/syscall/process.rs:15
#7  0x000000008023d458 in os::syscall::syscall (syscall_id=93, args=...) at src/syscall/mod.rs:100
#8  0x00000000802345a8 in os::trap::trap_handler () at src/trap/mod.rs:53
#9  0x000000000000110e in ?? ()
```
分析得知出现循环等待现象：

``waitpid``获取锁的顺序是 **自己->孩子**
``exit``获取锁的顺序是 **自己->孩子->INITPROC**

**cpu_1**为`initproc`，它先获取自己的锁。此时它正在进行`waitpid`系统调用，试图获取其子进程`usershell`的锁。
**cpu_2**为`user_shell`，它先获取自己的锁。此时它正在进行`waitpid`系统调用，试图获取其子进程`mmap`的锁。
**cpu_3**为`mmap`，它先获取自己和孩子（虽然没有）的锁。此时它退出，正想获取`initproc`的锁，从而把它的所有子进程（虽然没有）挂到`initproc`下面。

但是这三个进程的锁都被自己占有，谁也无法获取另一进程的锁，因而发生死锁。

这一情况在`ultraos`中有类似的情况出现。参考学长的解决方案，可以想到一种简单的策略：改变资源获取的顺序，将`exit_current_and_next`（对应上文中的cpu_3）中获取锁的顺序改变一下，即先获取`initproc`的锁，再获取自己和孩子的锁，变为**INITPROC->自己->孩子**。这种方法破坏了死锁的循环等待条件，避免了死锁的出现。

这种策略也可以完美解决`ultraos`之前遇到的父子进程同时退出的死锁问题。

## 成果

多核机制初步实现，目前可以稳定跑4核，且经过数次测试暂未发现问题。

可用`multicore_test`应用程序测试多核性能。

## 一些优化的想法

仿照`xv6`，设计一些方法用于减少多核时的竞争。