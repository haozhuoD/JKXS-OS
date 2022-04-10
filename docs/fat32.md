## 构建fat32文件系统

构建文件系统映像：

```sh
`cd fat32-fuse
dd if=/dev/zero of=fs.img bs=1k count=512k
cd ../os
./makefs.sh
# 或 make fs-img
```

## fat32内核接口设计

...

## 存在的问题

1. `ultraos`的校验和算法写的有问题，目前的策略是不进行checksum。(fixed)
2. `mmap`执行三次之后会panic，目前观察到是创建文件时，clear出现了问题。
3. `mkdir`再 `chdir`再 `mkdir`，会panic。
4. `test_mkdir`文件夹名字变成 `test_m~1`，而且显示的是普通文件，而不是文件夹。
5. 根目录..的问题
6. blockcache的命中率几乎为0？

## 还需实现的系统调用

* #define SYS_getdents64 61
* #define SYS_linkat 37
* #define SYS_unlinkat 35
* #define SYS_umount2 39
* #define SYS_mount 40
* #define SYS_fstat 80
* #define SYS_clone 220

## 调试：`mkdir`再 `chdir`再 `mkdir`崩溃的原因

发现问题在执行 `sys_exec`时。定位问题，发现在 `get_pos`函数中，offset = 4096时，计算current_cluster出了问题。

正常来说current cluster应该是1623.

```
get_pos(4096)
** get_cluster_at index = 1
in fat curr cluster = 1622, next cluster = 0
*** in get pos, cluster index = 1, current cluster = 0, first_cluster = 1622
first_sector_of_cluster: cluster = 0
```

next cluster = 0就很有问题了！不过尝试println有问题的变量之后，问题反而消失。我猜测这可能不是文件系统的问题，而是内核实现的问题。

一个有效的调试方法是在即将panic的地方插入loop{}语句，这样就可以开gdb调试，在卡住的地方backtrace。如果等到panic再backtrace则得不到正确的栈帧信息。
