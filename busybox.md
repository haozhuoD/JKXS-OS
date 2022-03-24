# 错误信息

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

###### 问题定位
(gdb) x/20x $sp
0xf0001fe8:     0x79737562      0x00786f62      0xf0001fe8      0x00000000
0xf0001ff8:     0x00000000      0x00000000      Cannot access memory at address 0xf0002000

第一次卡住是因为栈顶不是argc
第二次卡住是因为环境变量没有设置


###### 关于elf初始化 参考ultraos
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
