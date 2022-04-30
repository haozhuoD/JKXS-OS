## exec段偏移问题

见ultraos，已解决。

## 运行busybox，产生错误。

在0xc50cc指令产生缺页错误，stval=0x4bb9bcb08。
用riscv64-unknown-elf-objdump和gdb进行调试，获取错误信息如下：

## 错误信息

```
0x0000000000010144 in ?? ()
(gdb) si
0x0000000000010148 in ?? ()
(gdb) si
0x000000000001014a in ?? ()
(gdb) si
0x000000000001014e in ?? ()
(gdb) si
0x0000000000010152 in ?? ()
(gdb) si
0x0000000000010154 in ?? ()
(gdb) si
0x0000000000010156 in ?? ()
(gdb) si
0x000000000001015a in ?? ()
(gdb) si
0x000000000001015c in ?? ()
(gdb) watch $s0
Watchpoint 5: $s0
(gdb) si
0x0000000000010160 in ?? ()
(gdb) si
0x0000000000010164 in ?? ()
(gdb) si
0x0000000000010168 in ?? ()
(gdb) si
0x000000000001016c in ?? ()
(gdb) si
0x0000000000010170 in ?? ()
(gdb) si
0x0000000000010174 in ?? ()
(gdb) si
0x00000000000c5284 in ?? ()
(gdb) si
0x00000000000c5286 in ?? ()
(gdb) si
0x00000000000c5288 in ?? ()
(gdb) si
0x00000000000c528a in ?? ()
(gdb) si
0x00000000000c528e in ?? ()
(gdb) si
0x00000000000c5290 in ?? ()
(gdb) si
0x00000000000c5292 in ?? ()
(gdb) si
0x00000000000c5294 in ?? ()
(gdb) si
0x00000000000c5296 in ?? ()
(gdb) si
0x00000000000c529a in ?? ()
(gdb) si
0x00000000000c529c in ?? ()
(gdb) si
0x00000000000c529e in ?? ()
(gdb) si

Watchpoint 5: $s0

Old value = (void *) 0x0
New value = (void *) 0xf0001ff0
0x00000000000c52a0 in ?? ()
(gdb) ^CQuit
(gdb) si
0x00000000000c50ae in ?? ()
(gdb) si
0x00000000000c50b0 in ?? ()
(gdb) si
0x00000000000c50b2 in ?? ()
(gdb) si
0x00000000000c50b6 in ?? ()
(gdb) si
0x00000000000c50b8 in ?? ()
(gdb) si
0x00000000000c50ba in ?? ()
(gdb) si

Watchpoint 5: $s0

Old value = (void *) 0xf0001ff0
New value = (void *) 0x4bb9bcb08
0x00000000000c50bc in ?? ()
```

## 问题定位

有问题的寄存器是a0， 其值由读取*($sp)得来。查看栈内存，如下：

```
(gdb) x/20x $sp
0xf0001fe8:     0x79737562      0x00786f62      0xf0001fe8      0x00000000
0xf0001ff8:     0x00000000      0x00000000      Cannot access memory at address 0xf0002000
```

发现栈顶是一个字符串"busybox"，猜测a0读取了错误的值。查阅文档得知栈顶应为argc。修改exec，问题解决。

第一次卡住是因为栈顶不是argc。

## 第二次出现问题

解决第一个问题后又出现了page fault，定位错误为a2没有设置，而a2是环境变量的指针。

由此得知问题出现的原因：环境变量没有设置。

为了一次性解决问题，参考ultraos对exec函数进行大改，将环境变量env、辅助信息aux等入栈。
现在我们的os用户栈初始化后应该与ultraos保持一致。

## 关于进程栈初始化 参考ultraos

```c
    // exec will push following arguments to user stack:
    // STACK TOP
    //      argc
    //      *argv [] (with NULL as the end) 8 bytes each
    //      *envp [] (with NULL as the end) 8 bytes each
    //      auxv[] (with NULL as the end) 16 bytes each: now has PAGESZ(6)
    //      padding (16 bytes-align)
    //      rand bytes: Now set 0x00 ~ 0x0f (not support random) 16bytes
    //      String: platform "RISC-V64"
    //      Argument string(argv[])
    //      Environment String (envp[]): now has SHELL, PWD, LOGNAME, HOME, USER, PATH
    // STACK BOTTOM
    // Due to "push" operations, we will start from the bottom
```

## 很可惜，按照ultraos改了之后，又出现了新的page fault

增加额外入栈信息后，再次出现了一个page fault，出错指令为0xde0cc，stval = 0。这个0就很灵性，我也不清楚是怎么回事。
需要进一步调试！

## 解决

对比ultraos发现缺少了ph_head_addr这个aux。添加之后，busybox成功跑起！

## busybox需要支持的系统调用

发现busybox跑多核时会panic，因为 `processor`的 `index`被设为了一个奇怪的值，推测是执行 `busybox`时 `tp`寄存器被修改了。

另一个要注意的点：`trap.S`中，skip tp(x4), application does not use it这句话不再适用。

`ultraos`已给出解决方法：

```c
// strace ./busybox sleep 3
execve("./busybox", ["./busybox", "sleep", "3"], 0x3fffab8d70 /* 9 vars */) = 0
set_tid_address(0x122d08)               = 73
getuid()                                = 0
nanosleep({tv_sec=3, tv_nsec=0}, 0x3fffae6b70) = 0
exit_group(0)                           = ?
```

```c
root@oscomp:/mnt# strace ./busybox ls
execve("./busybox", ["./busybox", "ls"], 0x3fffc29d78 /* 9 vars */) = 0
set_tid_address(0x122d08)               = 78
getuid()                                = 0
clock_gettime(CLOCK_REALTIME, {tv_sec=4209, tv_nsec=486776700}) = 0
ioctl(0, TIOCGWINSZ, {ws_row=0, ws_col=0, ws_xpixel=0, ws_ypixel=0}) = 0
ioctl(1, TIOCGWINSZ, {ws_row=0, ws_col=0, ws_xpixel=0, ws_ypixel=0}) = 0
ioctl(1, TIOCGWINSZ, {ws_row=0, ws_col=0, ws_xpixel=0, ws_ypixel=0}) = 0
brk(NULL)                               = 0x123000
brk(0x125000)                           = 0x125000
mmap(0x123000, 4096, PROT_NONE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x123000
mmap(NULL, 4096, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3fd3be5000
newfstatat(AT_FDCWD, ".", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
openat(AT_FDCWD, ".", O_RDONLY|O_LARGEFILE|O_CLOEXEC|O_DIRECTORY) = 3
fcntl(3, F_SETFD, FD_CLOEXEC)           = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3fd3be3000
getdents64(3, 0x3fd3be3048 /* 6 entries */, 2048) = 184
newfstatat(AT_FDCWD, "./riscv64-syscalls.tgz", {st_mode=S_IFREG|0755, st_size=649227, ...}, AT_SYMLINK_NOFOLLOW) = 0
newfstatat(AT_FDCWD, "./riscv64", {st_mode=S_IFDIR|0755, st_size=4096, ...}, AT_SYMLINK_NOFOLLOW) = 0
newfstatat(AT_FDCWD, "./gtd___", {st_mode=S_IFREG|0755, st_size=62920, ...}, AT_SYMLINK_NOFOLLOW) = 0
newfstatat(AT_FDCWD, "./busybox", {st_mode=S_IFREG|0755, st_size=1116184, ...}, AT_SYMLINK_NOFOLLOW) = 0
getdents64(3, 0x3fd3be3048 /* 0 entries */, 2048) = 0
close(3)                                = 0
munmap(0x3fd3be3000, 8192)              = 0
ioctl(1, TIOCGWINSZ, {ws_row=0, ws_col=0, ws_xpixel=0, ws_ypixel=0}) = 0
writev(1, [{iov_base="\33[1;32mbusybox\33[m               "..., iov_len=49}, {iov_base="\n", iov_len=1}], 2busybox               riscv64
) = 50
writev(1, [{iov_base="\33[1;32mgtd___\33[m                "..., iov_len=62}, {iov_base="\n", iov_len=1}], 2gtd___                riscv64-syscalls.tgz
) = 63
exit_group(0)                           = ?
+++ exited with 0 +++
```

```c
root@oscomp:/mnt# strace ./busybox sh
execve("/bin/busybox", ["busybox", "sh"], 0x3fffc63d68 /* 9 vars */) = 0
brk(NULL)                               = 0xf8000
faccessat(AT_FDCWD, "/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib64/lp64d/tls/libm.so.6", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
newfstatat(AT_FDCWD, "/lib64/lp64d/tls", 0x3fff9c8e50, 0) = -1 ENOENT (No such file or directory)
openat(AT_FDCWD, "/lib64/lp64d/libm.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\363\0\1\0\0\0\260\261\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=2957368, ...}) = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3fd69e9000
mmap(NULL, 528536, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x3fd6967000
mmap(0x3fd69e7000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0x7f000) = 0x3fd69e7000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/lp64d/libresolv.so.2", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\0\0\0\0\0\0\0\0\0\3\0\363\0\1\0\0\0\3201\0\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=460088, ...}) = 0
mmap(NULL, 59128, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x3fd6958000
mmap(0x3fd6965000, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xc000) = 0x3fd6965000
close(3)                                = 0
openat(AT_FDCWD, "/lib64/lp64d/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0\363\0\1\0\0\0\300\1\2\0\0\0\0\0"..., 832) = 832
fstat(3, {st_mode=S_IFREG|0755, st_size=17493888, ...}) = 0
mmap(NULL, 1065496, PROT_READ|PROT_EXEC, MAP_PRIVATE|MAP_DENYWRITE, 3, 0) = 0x3fd6853000
mmap(0x3fd694e000, 24576, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_DENYWRITE, 3, 0xfa000) = 0x3fd694e000
mmap(0x3fd6954000, 12824, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_FIXED|MAP_ANONYMOUS, -1, 0) = 0x3fd6954000
close(3)                                = 0
mmap(NULL, 8192, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3fd6851000
mprotect(0x3fd694e000, 16384, PROT_READ) = 0
mprotect(0x3fd6965000, 4096, PROT_READ) = 0
mprotect(0x3fd69e7000, 4096, PROT_READ) = 0
mprotect(0xf5000, 4096, PROT_READ)      = 0
mprotect(0x3fd6a04000, 4096, PROT_READ) = 0
getuid()                                = 0
brk(NULL)                               = 0xf8000
brk(0x119000)                           = 0x119000
getpid()                                = 76
rt_sigaction(SIGCHLD, {sa_handler=0x6685c, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
getppid()                               = 73
uname({sysname="Linux", nodename="oscomp", ...}) = 0
newfstatat(AT_FDCWD, "/mnt", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
newfstatat(AT_FDCWD, ".", {st_mode=S_IFDIR|0755, st_size=4096, ...}, 0) = 0
ioctl(0, TCGETS, {B115200 opost isig icanon echo ...}) = 0
ioctl(1, TCGETS, {B115200 opost isig icanon echo ...}) = 0
rt_sigaction(SIGINT, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGINT, {sa_handler=0x6685c, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
rt_sigaction(SIGQUIT, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGQUIT, {sa_handler=SIG_IGN, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
rt_sigaction(SIGTERM, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGTERM, {sa_handler=SIG_IGN, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
openat(AT_FDCWD, "/dev/tty", O_RDWR)    = 3
fcntl(3, F_DUPFD_CLOEXEC, 10)           = 10
close(3)                                = 0
ioctl(10, TIOCGPGRP, [73])              = 0
getpgid(0)                              = 73
rt_sigaction(SIGTSTP, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGTSTP, {sa_handler=SIG_IGN, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
rt_sigaction(SIGTTOU, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
rt_sigaction(SIGTTOU, {sa_handler=SIG_IGN, sa_mask=~[RTMIN RT_1], sa_flags=0}, NULL, 8) = 0
rt_sigaction(SIGTTIN, NULL, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
setpgid(0, 76)                          = 0
ioctl(10, TIOCSPGRP, [76])              = 0
rt_sigaction(SIGHUP, {sa_handler=SIG_DFL, sa_mask=[HUP], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
ioctl(0, TCGETS, {B115200 opost isig icanon echo ...}) = 0
openat(AT_FDCWD, "/root/.ash_history", O_RDONLY) = 3
fstat(3, {st_mode=S_IFREG|0600, st_size=3357, ...}) = 0
read(3, "fdisk  -l \ndf -h\nmount \nmount -v"..., 1024) = 1024
read(3, ".bin \nscp os@192.168.100.1:/tmp/"..., 1024) = 1024
read(3, "busybox \nclear \nstrace busybox\nc"..., 1024) = 1024
read(3, "/\nls\nstrace ./busybox shell\nstra"..., 1024) = 285
read(3, "", 1024)                       = 0
close(3)                                = 0
ioctl(0, SNDCTL_TMR_START or TCSETS, {B115200 opost -isig -icanon -echo ...}) = 0
geteuid()                               = 0
openat(AT_FDCWD, "/etc/passwd", O_RDONLY) = 3
fstat(3, {st_mode=S_IFREG|0664, st_size=30, ...}) = 0
read(3, "root:x:0:0:root:/root:/bin/sh\n", 1024) = 30
close(3)                                = 0
ioctl(0, TIOCGWINSZ, {ws_row=0, ws_col=0, ws_xpixel=0, ws_ypixel=0}) = 0
getcwd("/mnt", 192)                     = 5
geteuid()                               = 0
fstat(1, {st_mode=S_IFCHR|0600, st_rdev=makedev(0x4, 0x40), ...}) = 0
ioctl(1, TCGETS, {B115200 opost -isig -icanon -echo ...}) = 0
rt_sigaction(SIGWINCH, {sa_handler=0xbe544, sa_mask=[], sa_flags=SA_RESTART}, {sa_handler=SIG_DFL, sa_mask=[], sa_flags=0}, 8) = 0
write(1, "/mnt # ", 7/mnt # )                  = 7
ppoll([{fd=0, events=POLLIN}], 1, NULL, NULL, 0) = 1 ([{fd=0, revents=POLLIN}])
```

## busybox新问题：

运行 `busybox sleep 3`时出现非法指令错误，问题正在排查。

非法指令如下：

```
a0b62:	f2000453          	fmv.d.x	fs0,zero
```

检查得知这是一条将整型数转换为浮点数的指令。由于 `rustsbi`未开启浮点指令，故该指令为非法。

## 浮点实现参考

华科xv6-k210文档[https://gitlab.eduxiji.net/retrhelo/xv6-k210/-/blob/scene/doc/%E6%9E%84%E5%BB%BA%E8%B0%83%E8%AF%95-%E6%B5%AE%E7%82%B9%E6%93%8D%E4%BD%9C.md]

具体实现方案：设置 `sstatus`的 `fs`位，这种方案可以不使用 `opensbi`。

```rust
pub fn init() {
    unsafe {
        sstatus::set_fs(FS::Clean);
    }
}
```

## 浮点上下文保存

// TODO...
