# busybox shell

# 遇到的问题

1. 执行时busybox_testcode.sh遇到了写缺页错误。
```
[kernel] Exception(StorePageFault) in application, bad addr = 0x0, bad instruction = 0xc60ca, kernel killed it.
```

特点似乎是只要while read line就会出错。
不对，好像是只要有while就会死掉？？

```shell
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- defconfig
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- menuconfig
make ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu-
```