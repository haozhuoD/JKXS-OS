## fat32内核接口设计

## bug: 尝试将efs换成fat32，发生错误

[kernel] Panicked at /home/user/OSComp-2022/simple_fat32/src/vfs.rs:159 attempt to subtract with overflow
---START BACKTRACE---
#0:ra=0x80243676
#1:ra=0x8027d452
#2:ra=0x802706f8
#3:ra=0x8026fc4c
#4:ra=0x802567d2
#5:ra=0x802569de
#6:ra=0x8020fe42
#7:ra=0x80248ce8
#8:ra=0x80242772
#9:ra=0x802442fc
---END   BACKTRACE---

