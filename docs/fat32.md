## 构建fat32文件系统

构建文件系统映像：
```sh
cd fat32-fuse
dd if=/dev/zero of=fs.img bs=1k count=512k
cd ../os
./makefs.sh
# 或 make fs-img
```

## fat32内核接口设计

...

## 存在的问题

1. `ultraos`的校验和算法写的有问题，目前的策略是不进行checksum。
2. `mmap`执行三次之后会panic，目前观察到是创建文件时，clear出现了问题。