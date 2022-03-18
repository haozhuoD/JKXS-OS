# OSCOMP-2022

## 更新日志

- 2022-3-16
  + 修改了内核WAIT4系统调用的逻辑（添加阻塞模式），使`riscv64`测例中`wait, waitpid, fork`等系统调用能正常运行。

- 2022-3-17
  + 修改了gettimeofday和sleep系统调用，使其能支持timeval结构体。
  + 添加了monitor模块，并为fs相关的系统调用添加了pin。修复Makefile，并添加了部分不重新编译内核，直接运行的命令。

- 2022-3-18
  + 初步实现`sbrk`系统调用，但`lazy allocation`尚未实现
  + 为支持程序大小的动态调整，用户程序虚拟地址结构有所变动：
    1. 段`.text, .rodata, .data, .bss`没有改变，分布在低地址`[0x1000, elf_end_addr)`。
    2. 可增长的堆内存`user_heap`，分布在低地址`[elf_end_addr, heap_end_addr)`，从低地址向高地址增长。
    3. 用户栈`user_stack`，分布在`[0x80000000, ..)`。
    4. 内核栈`kernel_stack`，分布在`[.., TRAMPOLINE - PAGE_SIZE)`

- 下一步计划实现`mmap`
