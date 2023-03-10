# sbi与多核启动

#### 旧版rust-sbi

qemu旧版rust-sbi并未在M态在核拦截，本实现参考rcore在S态进行多核拦截。区分为初始化核 与 其余核 。其余核必须等待初始化核完成一些必须的初始化后，其余核才能进行简单的初始化。（使用一个原子bool值进行控制）

# 从rust-sbi 到 opensbi

### OpenSBI- 远程平台/本地hifive平台/qemu

由于远程平台仅能使用opensbi所以需要进行适配，而且一跳转到0x8020_0000后直接卡死(因为之前默认0号核为主核，但冷启动的核是任意的)。

本地qemu使用opensbi，发现永远只有一个核能进入到s态。

###### GDB调试SBI：

直接使用opensbi官方release的bin进行debug，symbol-file的对应关系并不好。需本地编译opensbi，并在GDB中add-symble-file添加SBI-elf文件。

opensbi编译:   `make CROSS_COMPILE=riscv64-linux-gnu- PLATFORM=generic`

GDB调试发现，目前处于处理器核的 已经暂停状态

```
(gdb) i threads
  Id   Target Id                    Frame
* 1    Thread 1.1 (CPU#0 [halted ]) 0x0000000080009630 in ?? ()
  2    Thread 1.2 (CPU#1 [running]) 0x0000000080009630 in ?? ()
  3    Thread 1.3 (CPU#2 [halted ]) 0x0000000080009630 in ?? ()
  4    Thread 1.4 (CPU#3 [running]) core::sync::atomic::atomic_load (
    dst=0x8050a4e0 <os::AP_CAN_INIT> "\000", order=core::sync::atomic::Ordering::Relaxed)
    at /rustc/9ad5d82f822b3cb67637f11be2e65c5662b66ec0/library/core/src/sync/atomic.rs:2365
```

```
(gdb) where
#0  sbi_hsm_hart_wait (hartid=<optimized out>, scratch=0x80043060)
    at /home/dhz/workspace/opensbi-1.0/lib/sbi/sbi_hsm.c:121
#1  sbi_hsm_init (scratch=scratch@entry=0x80043000, hartid=hartid@entry=0, cold_boot=cold_boot@entry=0)
    at /home/dhz/workspace/opensbi-1.0/lib/sbi/sbi_hsm.c:206
#2  0x00000000800006f6 in init_warm_startup (hartid=0, scratch=0x80043000)
    at /home/dhz/workspace/opensbi-1.0/lib/sbi/sbi_init.c:360
#3  init_warmboot (hartid=0, scratch=0x80043000)
    at /home/dhz/workspace/opensbi-1.0/lib/sbi/sbi_init.c:436
#4  sbi_init (scratch=0x80043000) at /home/dhz/workspace/opensbi-1.0/lib/sbi/sbi_init.c:500
#5  0x00000000800003c4 in _start_warm () at /home/dhz/workspace/opensbi-1.0/firmware/fw_base.S:501


(gdb) i r mie
mie            0x8      8
(gdb) i r mip
mip            0x8      8
(gdb) i r mstatus
mstatus        0x0      SD:0 VM:00 MXR:0 PUM:0 MPRV:0 XS:0 FS:0 MPP:0 HPP:0 SPP:0 MPIE:0 HPIE:0 SPIE:0 UPIE:0 MIE:0 HIE:0 SIE:0 UIE:0
 
```

上述其他三个核运行到未知如下

```
### opensbi-1.0/lib/sbi/sbi_hsm.c:121

119 /* Wait for hart_add call*/  
120 while (atomic_read(&hdata->state) !=SBI_HSM_STATE_START_PENDING) {  
121    wfi();  
122 };
```

在qemu中使用sbi_get_hart_status查看核状态时发现其返回值也为0（started状态） 。

```
#define SBI_HSM_STATE_STARTED			0x0
#define SBI_HSM_STATE_STOPPED			0x1
#define SBI_HSM_STATE_START_PENDING		0x2
#define SBI_HSM_STATE_STOP_PENDING		0x3
#define SBI_HSM_STATE_SUSPENDED			0x4
#define SBI_HSM_STATE_SUSPEND_PENDING		0x5
#define SBI_HSM_STATE_RESUME_PENDING		0x6


### 通过sbi查看核状态
[ INFO ] "src/main.rs" @ 76 : (Boot Core) Riscv hartid 3 run  
[wakeup_other_cores]   hartid: 0 status:1
[ DEBUG ] "src/multicore/mod.rs" @ 45 : sbi_hart_start hartid: 0 -> ret: 0 
[wakeup_other_cores]   hartid: 1 status:1
[ DEBUG ] "src/multicore/mod.rs" @ 45 : sbi_hart_start hartid: 1 -> ret: 0 
[wakeup_other_cores]   hartid: 2 status:1
[ DEBUG ] "src/multicore/mod.rs" @ 45 : sbi_hart_start hartid: 2 -> ret: 0 
```

核间转化状态图如下：

![](image/MutiCore/1652353245819.png)![](image/多核启动/1652357558771.png)

所以只用使用 sbi_hart_start 唤醒 （虽然当前核状态已经为0/started）

综上得出目前的解决方法：

之前多核启动的方法(在s态拦截核、使用ipi)不行，需要使用SBI核状态管理扩展(hsm)的处理器核启动函数直接以S态从某个内存地址开始执行以启动

需要使用完整的ecall, 此前内核只使用了sbi-legacy的功能

### 当前还未解决的一个问题

SBI hart start return value （hart_start返回值/核状态相关问题）

本地hifive运行环境，基于uboot通过nfs将内核镜像加载到内存0x8020_0000处。然后跳转到0x8020_0000执行。

与远程提交平台环境类似 ，区别在于SDcard内容不同。本地SDcard前半部分存储了Uboot等数据，从10274开始才是我们所需的文件系统。

hifive上运行的问题：

大部分时候除了boot hard外，只有两个核能被唤醒，第四个核心在sbi_hart_start之后返回-6(表示该hart已经start)。

且在少部分时间四个核心全部启动的时候极大概率会卡死（未知，原因应该比较复杂，目前并未整明白）

目前的解决方法是，在第四个核唤醒失败时输出log，然后不管它。仅保证稳定三核运行稳定

### 在多核启动方面，真实上板子与qemu仿真环境有较大的不同！！！

qemu上每个核心都是启动后，直接跳转到0x8000_0000以M态从Opensbi开始运行和初始化，

hifive平台上已经完成了所有核心的初始化，且四个核都在运行Uboot(应该)。Opensbi是直接被加载到一定区域的内存中提供内核所需的运行时服务。

在hifive平台上的内核运行：

    由Uboot加载内核镜像到以0x8020_0000开始的一端内存区域，然后会跳转到0x8020_0000执行内核镜像（只有一个核跳转过去）。其他核心的状态通过sbi_get_hart_status查看发现都为1（stop状态）。调用sbi_hart_start指定其余核到内核入口进行执行时会比较稳定的有一个核执行sbi调用失败，原因是(-6)该核已经处于start状态。todo

参考 ： Rust-sbi官方文档

    官方仓库 https://github.com/riscv-non-isa/riscv-sbi-doc

    sbi手册下载地址 https://github.com/riscv-non-isa/riscv-sbi-doc/releases/download/v1.0.0/riscv-sbi.pdf
