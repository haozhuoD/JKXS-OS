# Busybox调试记录

## 地址0xc50cc处产生缺页错误

在0xc50cc指令产生缺页错误，stval=0x4bb9bcb08。
用riscv64-unknown-elf-objdump和gdb进行调试，获取错误信息如下：

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

有问题的寄存器是a0， 其值由读取*($sp)得来。查看栈内存，如下：

```
(gdb) x/20x $sp
0xf0001fe8:     0x79737562      0x00786f62      0xf0001fe8      0x00000000
0xf0001ff8:     0x00000000      0x00000000      Cannot access memory at address 0xf0002000
```

发现栈顶是一个字符串"busybox"，猜测a0读取了错误的值。查阅文档得知栈顶应为argc。修改exec，在栈中压入正确的参数，问题解决。

解决该问题后又出现了pagefault，定位错误为a2没有设置，而a2是环境变量的指针。

由此得知问题出现的原因：环境变量没有设置。

为了一次性解决问题，对exec函数进行大改，将环境变量env、辅助信息aux等入栈。

## 进程栈的初始化

```c
exec will push following arguments to user stack:
    STACK TOP
         argc
         *argv [] (with NULL as the end) 8 bytes each
         *envp [] (with NULL as the end) 8 bytes each
         auxv[] (with NULL as the end) 16 bytes each: now has PAGESZ(6)
         padding (16 bytes-align)
         rand bytes: Now set 0x00 ~ 0x0f (not support random) 16bytes
         String: platform "RISC-V64"
         Argument string(argv[])
         Environment String (envp[]): now has SHELL, PWD, LOGNAME, HOME, USER, PATH
    STACK BOTTOM
    Due to "push" operations, we will start from the bottom
```

## 0xde0cc处出现pagefault

增加额外入栈信息后，再次出现了一个page fault，出错指令为0xde0cc，stval = 0，对比ultraos发现缺少了ph_head_addr这个aux。添加之后，busybox成功跑起！

## busybox多核运行崩溃

发现busybox跑多核时会panic，因为 `processor`的 `index`被设为了一个奇怪的值，推测是执行 `busybox`时 `tp`寄存器被修改了。`tp`寄存器在内核态和用户态的含义不同，内核态下我们用 `tp`标识当前 `CPU`核的序号(`core_id`)，而用户态下 `tp`一般是用来保存当前线程信息结构体的地址。因此，我们需要在陷入时正确保存 `tp`寄存器的值，然后将其设为 `core_id`。`trap return`时恢复`tp`的值。

## “非法”的浮点指令

运行 `busybox sleep 3`时出现非法指令错误，非法指令如下：

```
a0b62:	f2000453          	fmv.d.x	fs0,zero
```

检查得知这是一条将整型数转换为浮点数的指令。由于 `rustsbi`未开启浮点指令，故该指令为非法。

浮点实现参考：华科xv6-k210文档[https://gitlab.eduxiji.net/retrhelo/xv6-k210/-/blob/scene/doc/%E6%9E%84%E5%BB%BA%E8%B0%83%E8%AF%95-%E6%B5%AE%E7%82%B9%E6%93%8D%E4%BD%9C.md]

具体实现方案：设置 `sstatus`的 `fs`位，这种方案可以不使用 `opensbi`。

```rust
pub fn init() {
    unsafe {
        sstatus::set_fs(FS::Clean);
    }
}
```
