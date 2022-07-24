
debian for qemu编译环境没法编译。无法从设置lmbench 自带的 __DEBUG 更快地定位错误。只能看源码，比较痛苦。 

目前对于第一个测试 lmbench_all lat_syscall -P 1 null 仍无法完全支持

### 已修复BUG与完善的系统调用

#### 信号系统修复与完善

完善：支持默认信号处理 SIG_DFL 和 SIG_IGN 的处理

修复：在初次sigactions的时候 o_act 置为 SIG_DFL

#### pselect实现

支持：
read-fd-set和write-fd-set的立即返回查询，以及对文件描述符是否阻塞的阻塞等待（内核态阻塞调用返回，仅支持PIPE文件）

新增文件trait ：
```
判断当前文件描述符对应的文件是否阻塞
fn read_blocking(&self) -> bool;
fn write_blocking(&self) -> bool;
```

##### todo

当前不支持超时后返回，会一直阻塞等待直到有一个操作符可用

当前对于erro fd set 只是简单的全部清零



### 用户态缺页异常问题--待修复 TODO
（只差最后这一点了）
子进程benchmp_child 
循环运行 benchmp_interval     
在最后case： cooldown的 exit(0) 退出时 用户态运行爆炸。 原因计算错误的寄存器 a4=0x4，jalr a4 导致缺页  （也行是之前某个系统调用返回值不正确？） 
```
相关函数地址信息
0x000000000006f7fa  exit函数入口
0x000000000005c0c8  子进程运行的benchmp_interval函数  call exit 的位置
0x000000000006f704  jalr a4 爆掉的地方
```

```
相关反汇编代码
6f6fe:	0007b823          	sd	zero,16(a5)
6f702:	85da                	mv	a1,s6
6f704:	9702                	jalr	a4                        //debug 第一次：goto 6afda : __libc_csu_fini 返回后继续执行 --- 第二次：a4=0x04 寄
6f706:	100d27af          	lr.w	a5,(s10)   
```

```
GDB信息
(gdb) si
0x000000000006f702 in ?? ()
(gdb) si
0x000000000006f704 in ?? ()
(gdb) si
0x0000000000000004 in ?? ()
(gdb) si
0xfffffffffffff004 in ?? ()
```

trace 比对

仅仅卡在最后子进程exit(0)退出的用户态部分

```
RISCV64 Debian for qemu - trace
247 父进程   248 子进程

[pid   248] pselect6(8, [7], NULL, NULL, {tv_sec=0, tv_nsec=0}, NULL) = 1 (in [7], left {tv_sec=0, tv_nsec=0})
[pid   248] read(7, "\0", 1)            = 1
[pid   248] write(4, "\v\0\0\0\0\0\0\0y\332\27\0\0\0\0\0x\0\0\0\0\0\0\0\2\235\24\0\0\0\0\0"..., 184 <unfinished ...>
[pid   247] <... pselect6 resumed>)     = 1 (in [3], left {tv_sec=0, tv_nsec=955386166})
[pid   248] <... write resumed>)        = 184
[pid   247] read(3,  <unfinished ...>
[pid   248] read(9,  <unfinished ...>
[pid   247] <... read resumed>"\v\0\0\0\0\0\0\0y\332\27\0\0\0\0\0x\0\0\0\0\0\0\0\2\235\24\0\0\0\0\0"..., 184) = 184
[pid   247] rt_sigaction(SIGCHLD, {sa_handler=SIG_DFL, sa_mask=[CHLD], sa_flags=SA_RESTART}, {sa_handler=0x5a09c, sa_mask=[CHLD], sa_flags=SA_RESTART}, 8) = 0
[pid   247] write(10, "\v", 1)          = 1
[pid   248] <... read resumed>"\v", 1)  = 1
[pid   247] close(3 <unfinished ...>
[pid   248] exit_group(0 <unfinished ...>
[pid   247] <... close resumed>)        = 0
[pid   248] <... exit_group resumed>)   = ?
[pid   247] close(6 <unfinished ...>
[pid   248] +++ exited with 0 +++
<... close resumed>)                    = 0
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=248, si_uid=0, si_status=0, si_utime=3, si_stime=114} ---
close(8)                                = 0
close(10)                               = 0
rt_sigaction(SIGCHLD, {sa_handler=SIG_DFL, sa_mask=[CHLD], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[CHLD], sa_flags=SA_RESTART}, 8) = 0
rt_sigaction(SIGALRM, {sa_handler=0x5a136, sa_mask=[ALRM], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
setitimer(ITIMER_REAL, {it_interval={tv_sec=0, tv_usec=0}, it_value={tv_sec=5, tv_usec=0}}, {it_interval={tv_sec=0, tv_usec=0}, it_value={tv_sec=0, tv_usec=0}}) = 0
wait4(248, NULL, 0, NULL)               = 248
setitimer(ITIMER_REAL, {it_interval={tv_sec=0, tv_usec=0}, it_value={tv_sec=0, tv_usec=0}}, {it_interval={tv_sec=0, tv_usec=0}, it_value={tv_sec=4, tv_usec=980740}}) = 0
rt_sigaction(SIGALRM, {sa_handler=SIG_DFL, sa_mask=[ALRM], sa_flags=SA_RESTART}, {sa_handler=0x5a136, sa_mask=[ALRM], sa_flags=SA_RESTART}, 8) = 0
write(2, "Simple syscall: 8131.4488 micros"..., 39Simple syscall: 8131.4488 microseconds
) = 39
exit_group(0)                           = ?
+++ exited with 0 +++
```

```
JKXS-OS - trace
2 父进程   3 子进程

[syscall pid=3] : sys_read(fd: 7, buf: *** , len: 1) = 1
[syscall pid=3] : sys_write(fd: 4, buf: ?, len: 184) = 184
[syscall pid=2] : sys_pselect(nfds: 0x4, rfds = 8, wfds = 0x0, efds = 0xf00178d0, timeout: TimeSpec { tv_sec: 1, tv_nsec: 0 }) = 1
[syscall pid=2] : sys_read(fd: 3, buf: *** , len: 184) = 184
[syscall pid=2] : syscall(134), args = [11, f0017598, f0017628, 8, fffffffffffffea7, 10000000]
[syscall pid=2] : sys_sigaction(signum: 17, sigaction = SigAction {
    handler: 0x0,
    sigaction: 0x10000000,
    mask: 0x10000,
}, old_sigaction = SigAction {
    handler: 0x5a09c,
    sigaction: 0x10000000,
    mask: 0x10000,
} ) = 0
[syscall pid=2] : sys_write(fd: 10, buf: ?, len: 1) = 1
[syscall pid=3] : sys_read(fd: 9, buf: *** , len: 1) = 1
[syscall pid=2] : syscall(57), args = [3, 10fea0, 1, 1, 10ba40, 0]
[syscall pid=2] : sys_close(fd: 3) = 0
[ ERROR ] "src/trap/mod.rs" @ 73 : [pid=3] Exception(InstructionPageFault) in application, bad addr = 0x4, bad instruction = 0x4, kernel killed it.

...... dead loop

```


