# Lmbench性能优化策略概述

## Simple Syscall优化

### 系统调用向量化

原先的实现中，系统调用的分发是通过 `match`语句实现的，相当于线性查找。当系统调用数量增加时，`match`将会变得低效。因此，我们采取系统调用向量化的策略，用查表的方式直接获取系统调用号对应的 `syscall`处理函数，从而加速 `syscall`的分发。相关代码如下：

```rust
    SYSCALL_TABLE.iter_mut().for_each(|x| *x = sys_unknown as usize);
    SYSCALL_TABLE[SYSCALL_GETCWD] = sys_getcwd as usize;
    SYSCALL_TABLE[SYSCALL_DUP] = sys_dup as usize;
    SYSCALL_TABLE[SYSCALL_DUP3] = sys_dup3 as usize;
    SYSCALL_TABLE[SYSCALL_FCNTL] = sys_fcntl as usize;
    SYSCALL_TABLE[SYSCALL_IOCTL] = sys_ioctl as usize;
    SYSCALL_TABLE[SYSCALL_MKDIRAT] = sys_mkdirat as usize;
    SYSCALL_TABLE[SYSCALL_UNLINKAT] = sys_unlinkat as usize;
    SYSCALL_TABLE[SYSCALL_UMOUNT2] = sys_umount as usize;
    SYSCALL_TABLE[SYSCALL_MOUNT] = sys_mount as usize;
    SYSCALL_TABLE[SYSCALL_STATFS] = sys_statfs as usize;
    SYSCALL_TABLE[SYSCALL_FACCESSAT] = sys_faccessat as usize;
    SYSCALL_TABLE[SYSCALL_CHDIR] = sys_chdir as usize;
```

### 当前进程信息的快速查找 - FastAccess

我们发现每次 `trap`都会调用 `current_user_token()`等获取当前线程/进程信息的函数。这些函数都需要获取TCB或PCB内部的锁，通过gdb单步执行追踪 `trap`过程，发现获取锁的操作会引入40个指令左右的开销（即使是在没有竞争的情况下）。那我们有没有办法减少这一开销呢？为此我们采取的优化策略是：设置一个全局的无锁结构体 `__FA[]` `(Fast Access)`，它保存当前核上正在运行任务的一些信息，包括 `tid`、`trap_cx`的位置、页表地址，这样我们就可以直接访问 `__FA[]`内的数据，而不需获取锁。注意 `tid`、`trap_cx`的位置、页表地址等信息在线程/进程创建后一般不会修改，我们只需要在切换任务和 `exec`时更新 `__FA[]`的信息即可。

## Syscall read/write优化

### Userbuffer：去除Vec

我们对用户地址缓冲 `UserBuffer`进行了重构，将其中较慢的 `Vec`更换为定长数组，从而减少堆内存 `alloc / dealloc`的开销。这一优化对 `simple read/write`的性能提升约为 `1us`。

### Userbuffer切片复制：copy_slice

`UserBuffer`的复制最好采用 `copy_slice`方法，编译器似乎会对这个方法进行优化（比如一个指令填充8B数据）。如果采用原来的逐字节填充 `(*mut u8)`就显得低效了。

## Syscall stat / open优化

这两个测试的共同点在于：它们都需要在文件系统查找并打开文件。如何快速地在文件系统中查询文件，是针对该测试优化的重点。

### 路径解析加速

解析用户态传来的字符串时，不需要对字符串的每个字节都模拟查询一次页表，事实上，一个页只要查询一次即可。

### 磁盘文件索引 (FSIDX, FS Index)

见 `FAT32`优化文档“文件和目录的查找和创建优化”部分。建立磁盘文件索引能够尽可能减少访问文件系统的次数，大大提高查找文件的效率。

## Syscall fork 优化

### 写时复制策略(Copy On Write)

见 `CoW`实现文档。在许多情况下，`fork`系统调用产生的子进程会很快进行 `exec`，进程复制时就为子进程分配全部物理页面将是一种低效的策略（因为这些页面在 `exec`时又会被回收）。`CoW`机制能够节省 `fork`系统调用时复制大量页面所带来的开销，在时间和空间上均有极好的优化效果。

For fork + exit (on fu740)，
before CoW: 5010.0 us
after Cow :  657.1 us

### 最大程度精简内核栈

内核栈是在内核页表中进行映射的，而 `CoW`只对用户页表生效，这使得内核栈必须在 `fork`时创建。原先我们的内核栈大小为260KB，事实上，经测试只需8KB即可支持正常运行。精简内核栈后性能大幅提高。

For fork + exit (on fu740)，
before : 657.1 us
after  : 243.5 us

## Syscall exec 优化

### 利用文件缓存(VFile Cache) 减少一次拷贝

在 `VFS`层，我们为磁盘文件实现了文件缓存 `(VFile Cache)`，该机制类似于 `Linux`的页缓存 `(Page Cache)`。该机制实现后，内核读写文件时将不再需要频繁读写磁盘，而是直接与文件缓存交互，从而提高读写文件的速率。

与页缓存不同的是，我们采取了一种更加粗粒度的实现策略：为文件分配一片连续的按页对齐的内存空间。这样做的好处是，我们可以将这个连续区域的引用（切片）直接传给 `Elf`解析函数，从而减少一次拷贝（原先是要把数据复制一份再传过去）。由于我们需要 `exec`很大的文件（如 `1.5MB`的 `busybox`），故减少一次拷贝带来的优化是极为明显的。

For fork + execve (on fu740)，
before : 15097.0 us
after  :  6545.0 us

### 创建用户页表映射：再减少一次拷贝，接近零拷贝！！

在我们最初的实现中，`exec`创建新的用户页表时，首先需要为进程分配物理页，然后将 `elf`数据拷贝进去，最后在页表中建立映射。其实仔细分析不难发现，对于 `elf`的只读逻辑段，我们可以直接在新页表中把虚拟地址映射到 `elf`文件缓存的对应位置，而不需要拷贝数据。

自此之后，我们唯一需要拷贝的就是 `elf`的可读写数据段（这部分不做直接映射，主要是因为 `CoW`处理不方便）。经过测试，只读数据段的页数约为可读写数据段的 `20`倍，减少这一次拷贝的优化效果非常显著。

For fork + execve (on fu740)，
before : 6545.0 us
after  : 1248.3 us

### 精简内核栈

详见上一部分。由于 `exec`时也需要分配内核栈，故精简内核栈对 `exec`也有一定程度的优化。

For fork + execve (on fu740)，
before : 1248.3 us
after  :  577.3 us

