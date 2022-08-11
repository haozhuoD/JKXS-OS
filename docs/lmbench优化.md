# lmbench优化

## 需要注意的点

- [ ] trap_from_kernel不能触发

## simple syscall优化

- [X] 系统调用向量化，用查表的方式加速syscall的分发。（优化效果似乎不明显，simple_syscall优化约0.2ms）
- [X] **低效的锁？获取一次spin锁需要40条左右的指令开销！这个问题非常严重。**
- [X] 内联优化：常用的函数，如current_trap_cx()等可改为内联函数；
- [ ] trap陷入时sx是否有必要保存？
- [ ] trap_cx的内容是否可以简化？如core_id等是否需要？

## 内存可做的优化

- [ ] frame_allocator的大锁是否可以去掉？
- [X] frame_alloc时，能否不将页清零？（分配页表的页帧时需要清零）。优化不明显
