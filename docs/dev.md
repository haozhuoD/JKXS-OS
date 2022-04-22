# OSCOMP-2022

## 更新日志

- 2022-3-16
  + 修改了内核WAIT4系统调用的逻辑（添加阻塞模式），使`riscv64`测例中`wait, waitpid, fork`等系统调用能正常运行。

- 2022-3-17
  + 修改了gettimeofday和sleep系统调用，使其能支持timeval结构体。
  + 添加了monitor模块[monitor文档](./monitor.md)
    
    并为fs相关的系统调用添加了pin。修复Makefile, 并添加了部分不重新编译内核, 直接运行的命令。

- 2022-3-18
  + 初步实现`sbrk`系统调用，但`lazy allocation`尚未实现
  + 为支持程序大小的动态调整，用户程序虚拟地址结构有所变动：
    1. 段`.text, .rodata, .data, .bss`没有改变，分布在低地址`[0x1000, elf_end_addr)`。
    2. 可增长的堆内存`user_heap`，分布在低地址`[elf_end_addr, heap_end_addr)`，从低地址向高地址增长。
    3. 用户栈`user_stack`，分布在`[0xf000_0000, ..)`。
    4. 内核栈`kernel_stack`，分布在`[.., TRAMPOLINE - PAGE_SIZE)`
  + [当前内核、用户程序地址空间分配图](https://gitee.com/chen_lin_k/oscomp-2022/tree/doc/memory_set.md)

- 2022-3-24
  + 初步实现`sbrk`和`mmap`, 并支持`lazy allocation`
    [mmap文档](./mmap.md)
  + 为虚拟空间创建过程添加pin, 可观察虚拟空间mapping信息

- 2022-3-27
  + 实现了`getppid, uname, times`系统调用。

- 2022-3-30
  + 多核初步实现，目前可以稳定跑4核，暂未发现问题。（或许将来会有呢？）
    [multicore文档](./multicore.md)

- 2022-4-22
  + busybox ls 简单实现
  + busybox sh 需要实现信号机制，将来有机会实现之。