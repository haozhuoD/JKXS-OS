# lmbench优化

## 需要注意的点

- [X] trap_from_kernel不能触发
- [ ] 单核可以不用锁

## simple syscall优化

- [X] 系统调用向量化，用查表的方式加速syscall的分发。（优化效果似乎不明显，simple_syscall优化约0.2ms）
- [X] **低效的锁？获取一次spin锁需要40条左右的指令开销！这个问题非常严重。**
- [X] 内联优化：常用的函数，如current_trap_cx()等可改为内联函数；
- [ ] trap陷入时sx是否有必要保存？
- [ ] trap_cx的内容是否可以简化？如core_id等是否需要？

## 内存可做的优化

- [ ] frame_allocator的大锁是否可以去掉？
- [ ] heap_allocator太慢
- [X] 减少不必要的Vec使用？（需要alloc和dealloc）
- [X] Pagetable::from_token，是否还需要frames这个vec？
- [X] translated_str不需要逐字节翻译页表
- [X] Vec初始化时预留容量
  内存可做的优化
- [ ] 进程创建时栈空间可否lazy分配？
- [ ] 控制alloc时是否需要清空？

## fs可做的优化

- [ ] 缓存读写锁数组可以换成Atomic？
- [ ] 路径解析加速
- [ ] read_all返回切片？减少一次拷贝

## 进程调度相关

- [ ] 阻塞式wait
- [ ] task回收、tcb重构
