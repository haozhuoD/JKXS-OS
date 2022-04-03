# Monitor

无需编译即可在GDB中控制内核中各种调试输出开关 --- 思路借鉴UltraOS
其基本原理为：预留专用内存区域，以其内部数据作为内核调试输出开关，这样，通过GDB修改对应的地址值，就可以达到控制调试目标、输出粒度、是否开启等各类参数。



### 简单介绍

```
in gdb-debug:
    p {char}0x807ff000              -0x807ff000 为控制信息预留的内存地址
    set {char}0x807ff000 = 0        -修改内存值,关闭信息输出
    set {char}0x807ff000 = 1        -开启信息输出
```
set {char}0x87fff000 = 1
插入pin
`gdb_println!(SYSCALL_ENABLE,"+++ gdb_println test1");`