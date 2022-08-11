# lmbench优化

## simple syscall优化

1. 系统调用向量化，用查表的方式加速syscall的分发。（优化效果似乎不明显，simple_syscall优化约0.2ms）
2. **低效的锁？获取一次spin锁需要40条左右的指令开销！这个问题非常严重。**
3. 内联优化：常用的函数，如current_trap_cx()等可改为内联函数；
4. current_trap_cx()也有些慢了。是否可以利用gp？
5. trap陷入时sx是否有必要保存？
6. trap_cx的内容是否可以简化？如core_id等是否需要？
