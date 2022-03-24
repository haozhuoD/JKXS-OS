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
``` c
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