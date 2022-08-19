![logo](./docs/image/readme/学校logo.png)

# **jkxs-OS:** `<i>`System Overview`</i>`

![System](https://img.shields.io/badge/System-jkxsOS-brightgreen) ![Lang](https://img.shields.io/badge/Lang-Rust-green) ![ISA](https://img.shields.io/badge/ISA-riscv64-yellowgreen) ![Platform](https://img.shields.io/badge/Platform-Qemu%2c%20Hifive%20Unmatched-yellow)

## `<i>`Current Achievements`</i>`

### 完成情况：通过比赛所有测试用例，针对性能瓶颈进行优化，完成了部分移植工作

#### 内核赛道 决赛第二阶段 `fu740` 得分为116.9109 排名第二（截至2022.8.19 17:27）

![决赛stage2](./docs/image/readme/决赛第二阶段排名.png)

#### 内核赛道 决赛第一阶段 `fu740`测评满分

![决赛stage1](./docs/image/readme/决赛stage1排行榜.png)

#### 内核赛道 初赛双赛道 `(k210 & fu740)`测评满分

![fu740](./docs/image/readme/fu740排行榜.png)

![k210](./docs/image/readme/k210排行榜.png)

## `<i>`Documents `</i>`

#### 项目整体设计文档 [doc_jkxsOS设计文档](./doc_jkxsOS设计文档.pdf)

#### 模块开发相关文档 [./docs](./docs)

## `<i>`Branches `</i>`

* `main`               ------主分支
* `libc-test`          ------`libc`测试提交分支（决赛第一阶段提交）
* `lmbench`            ------`lmbench`开发分支
* `porting`            ------移植工作开发分支
* `hifive-SingleCore`  ------`fu740`平台单核分支
* `pre-k210`           ------`k210`平台单核分支

## 项目代码架构

`<i>`code`</i>`：项目代码框架

|   文件目录   |         描述         |
| :-----------: | :------------------: |
|  bootloader  |    sbi二进制文件    |
| external_libs | 项目依赖的部分外部库 |
|   fat32_fs   |    fat32文件系统    |
|      os      |        OS内核        |
|     user     |       用户程序       |

## `<i>`How to Run`</i>`

#### qemu运行

切换工作目录为 `./os`：

```shell
cd os
```

安装必要的编译环境：

```shell
make env
```

运行 `jkxs-OS`需先生成FAT32标准的文件系统镜像：

```shell
make fs-img
```

运行 `jkxs-OS`：

```shell
make run
```

#### 生成fu740的内核镜像

在项目根目录下运行命令生成 `fu740`平台的 `JKXS-OS`内核镜像：

```shell
make all
```

#### GDB调试

在两个终端中分别运行如下命令，即可启动 `gdb`调试：

```shell
make gdb
make monitor
```

## `<i>`Our Work `</i>`

+ `jkxs-OS`致力于开发一个能在 `RISCV-64`处理器上运行的宏内核操作系统。我们以清华大学吴一凡同学的教学项目 `rCore-Tutorial`为基础，在其代码框架上进行迭代开发。
+ 之所以选择 `rCore-Tutorial`，一是它的开发语言——`Rust`在系统编程领域具有得天独厚的优势，它天然地保证了程序的内存安全和线程安全，能够帮助我们规避内核开发中的诸多潜在问题；二是 `rCore-Tutorial`是一个较为完整的系统，具有良好的可拓展性，基于 `rCore-Tutorial`开发能够避免“重复造轮子”的麻烦，节省了在线程上下文切换、页表访问等大量细节的实现上需要花费的时间，使我们能将更多的精力投入到内核性能的优化、健壮性的增强、用户体验的提升等环节上，做出更有意义、更具有创新性的工作。
+ 对于基于 `riscv64`体系结构的操作系统内核实现，上一届代表哈尔滨工业大学（深圳）参赛的 `UltraOS`队伍已经进行了相当多的探索。作为本科生小组独立开发完成的项目，`UltraOS`无疑是杰出的。它不仅实现了大赛的全部功能要求，还对系统性能进行了针对性的优化，且具有诸多亮点（创造性的 `Monitor`调试模块、初始进程和 `shell`的回收、`kmem`设计、相当完善的文档等）。`UltraOS`的精妙设计给我们提供了许多灵感，有了 `UltraOS`的重要探索，可以说我们已经“站在了巨人的肩膀上”。不过，我们不希望 `jkxs-OS`仅仅成为一个“`UltraOS`的翻版”；我们希望 `jkxs-OS`能够比“巨人”站得更高、看的更远——具体来说，就是拥有比 `UltraOS`更完善的硬件支持、更好的性能、更优雅的设计、更完善的文档、更好的用户体验。

**我们具体工作如下：**

### 1. 内核运行的必要模块

+ 进程调度。我们使用进程控制块 `(PCB)`作为用户进程的抽象，在其上记录了进程运行所需要的一系列关键信息；为每个 `CPU`核抽象出一个任务处理器 `Processor`，记录当前在其上执行的进程。我们还创建了全局任务管理器 `Task Manager`，用于记录正在排队等待运行的进程，便于 `Processor`进行任务的调度。
+ 内存管理。我们对内核和用户拥有的内存空间统一抽象成一个类 `MemorySet`，用于方便地实现对内存的管理。它对外提供了完善的接口，避免了外界对页表的直接操作；我们还扩展了 `MemorySet`的接口，使其能支持 `mmap`和 `heap`区域的管理；最后，我们实现了 `Lazy Allocation`机制，避免一次分配过大的内存，从而减少内存的浪费。

### 2. 多核运行支持

+ 我们对内核中所用的全局变量 `(PCB, Frame Allocator, Task Manager, Console...)`添加了读写锁 `(RwLock)`，确保了线程安全。与互斥锁 `(Mutex Lock)`相比，读写锁支持多核并发的读操作，具有更好的并行度。
+ 文件系统模块也对 `FAT`、文件系统管理器，块缓存等数据结构加锁，保证了多核并行条件下程序的正确性。

### 3. FAT32文件系统

+ 我们采用五层的分层结构来设计 `FAT32`文件系统，从下到上分别为磁盘块设备接口层、块缓存层、磁盘布局层、文件系统管理层、虚拟文件系统层。
+ 我们沿袭 `rCore-Tutorial`的松耦合模块化设计思路，将 `FAT32`文件系统从内核中分离出来，形成一个独立的 `Cargo crate`。这样，我们就可以单独对文件系统进行用户态测试。在用户态测试完毕后，可直接放到内核中，形成有文件系统支持的新内核。

### 4. 现实程序运行支持

+ `rCore-Tutorial`已经提供了 `27`条系统调用，但它们有些是冗余的（如线程、信号量相关的系统调用，我们的内核实现暂时不需要这些功能），且大多数不符合 `POSIX`标准。这使得原生的 `rCore-Tutorial`不能通过 `Online Judge`平台的评测，更不可能支持busybox这类复杂的应用程序。因此，我们修改了并扩展了系统调用的接口，实现了 `syscall`的规范化。
+ 为更好地为现实程序的运行提供支持，我们将系统调用的数量扩充至 `66`个（后续还会增加），也相应地实现了一些机制来支持这些 `syscall`的执行（如 `mmap`机制、信号机制等）
+ 部分复杂的应用程序（如 `busybox`）需要实现更完善的内核机制才能运行。为此我们也做了很多工作，如扩展 `exec`系统调用，将程序运行的必要参数提前压入用户栈；又如开启浮点运算机制、实现进程上下文切换时 `tp`寄存器的保存与恢复等。

### 5. SBI支持与多核启动

+ 在 `qemu`上,  可使用 `Rust-SBI`或 `Open-SBI`支持内核运行, 并在此基础上实现多核启动。
+ 在 `K210`上, 可使用 `Rust-SBI`支持内核运行。
+ 在 `FU740`上, 可使用 `Open-SBI`支持内核运行, 并在此基础上实现多核启动。

### 6. 多硬件平台支持

+ 我们在实践中发现，`Qemu`提供的虚拟硬件环境和真实开发板有诸多细微的差别，这使得能够在 `Qemu`上稳定运行的程序在真实开发板上经常出现各种各样的问题（例如，`lazy_static!`宏无法在 `Hifive Unmatched`开发板上正常工作，需要更换为 `Lazy Cell`）。因此，对具体开发板的硬件环境进行适配，是我们必须完成的工作。
+ 目前，我们的内核已经完成了对 `K210`和 `Hifive Unmatched`的适配，并且均在在线评测平台上获得满分。对于 `Hifive Unmatched`，我们完成了更多的工作，包括 `MMIO`的调整、`SDCard`驱动的开发与测试等。

### 7. 更便捷的调试功能

+ 我们编写了 `Monitor`模块，帮助我们对内核进行更方便的调试。它的基本原理是预留专用内存区域，以其内部数据作为内核调试输出开关。这样，通过 `GDB`修改对应内存的值，就可以达到控制调试目标、输出粒度、调试信息输出是否开启等各类参数的效果。
+ 系统调用追踪是一类极为常用的调试手段，我们的内核也实现了这一功能。内核可为用户反馈系统调用 `id`、名称、输入参数、返回值等信息，用户也可以在 `usershell`中通过 `trace`命令控制 `syscall`调试信息的输出开关。

### 8. 更友好的用户体验

+ 虽然大赛需要的测试程序不需要 `usershell`也可以直接执行，但是如果缺少了 `shell`，操作系统就失去了与用户交互的能力，而且也为调试带来了困难。`rCore-Tutorial`已有一个简单的 `shell`实现，但这个 `shell`使用起来有相当多的不便之处，比如使用左右键命令会出错、使用上下键光标会跑飞，使得命令的输入相当不人性化。
+ 基于此，我们实现了一个功能更加强大、且用户体验更好的 `shell`。它不仅解决了上下左右键按下时命令出错、光标跑飞等问题，还支持 `tab`命令补全、命令历史回溯等功能，为调试带来极大便利。
+ 我们实现了一套较为完整的日志系统，用户可选择最小日志输出等级，并通过 `debug!`，`info!`，`warning!`，`error!`等宏打印不同输出等级的日志信息。日志系统能为不同等级的输出信息设置不同的颜色，且能显示输出语句所在文件的名称和所在行数，使内核输出更加清晰、直观，更利于调试。

## `<i>`Future Plans `</i>`

| 计划                               | 优先级 |
| ---------------------------------- | ------ |
| 支持 `lmbench`，并进行性能测试   | 高     |
| 基于性能测试进行性能优化           | 高     |
| 提升多核运行的稳定性，修复bug      | 高     |
| 提高 `SDcard`驱动的稳定性        | 中     |
| 基于硬件平台拓展内核的功能：网卡等 | 低     |

## `<i>`Contact Us `</i>`

本项目的三位队员均来自哈尔滨工业大学（深圳），指导老师为夏文老师和仇洁婷老师。

丁浩卓（队长）：负责SDCard驱动、多核支持、性能优化。
郑启洋：负责FAT32文件系统的设计与实现。
陈林锟：负责进程、内存管理模块设计，信号系统。

如有相关技术问题，联系 `2567769508@qq.com`。

## `<i>`Future Plans `</i>`

| 计划                               | 优先级 |
| ---------------------------------- | ------ |
| 支持 `lmbench`，并进行性能测试   | 高     |
| 基于性能测试进行性能优化           | 高     |
| 提升多核运行的稳定性，修复bug      | 高     |
| 提高 `SDcard`驱动的稳定性        | 中     |
| 基于硬件平台拓展内核的功能：网卡等 | 低     |

## `<i>`Contact Us `</i>`

本项目的三位队员均来自哈尔滨工业大学（深圳），指导老师为夏文老师和仇洁婷老师。

丁浩卓（队长）：负责 `SDCard` 驱动、多核支持、性能优化。
郑启洋：负责 `FAT32` 文件系统的设计、实现与优化。
陈林锟：负责进程管理、内存管理、信号系统。

如有相关技术问题，联系 `2567769508@qq.com`。

## `<i>`Credits`</i>`

本项目基于吴一凡等开发者的 `rCoreTutorial-v3` 项目进行开发。
感谢同样来自哈尔滨工业大学（深圳）的叶自立、张艺枫、潘智伟等一起参赛的同学，在和你们的交流中，我们学到了很多。
同样感谢夏文老师和仇洁婷老师对我们的帮助和指导。
