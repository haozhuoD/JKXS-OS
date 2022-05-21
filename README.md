# system overview

## Our Work

jkxs-OS致力于开发一个能在RISCV-64处理器上运行的宏内核操作系统。我们以清华大学吴一凡同学的教学项目rCore-Tutorial为基础，在其代码框架上进行迭代开发。

之所以选择rCore-Tutorial，一是它的开发语言——Rust在系统编程领域具有得天独厚的优势，它天然地保证了程序的内存安全和线程安全，能够帮助我们规避内核开发中的诸多潜在问题；二是rCore-Tutorial是一个较为完整的系统，具有良好的可拓展性，基于rCore-Tutorial开发能够避免“重复造轮子”的麻烦，节省了在线程上下文切换、页表访问等大量细节的实现上需要花费的时间，使我们能将更多的精力投入到内核性能的优化、健壮性的增强、用户体验的提升等环节上，做出更有意义、更具有创新性的工作。

至此，我们已经实现的内核功能已经比rCore-Tutorial更加强大，稳定性更加良好，对现实程序的支持能力更强。

截止初赛结束，我们具体工作如下：

1. 内核运行的必要模块

   + 进程调度。我们使用进程控制块(PCB)作为用户进程的抽象，在其上记录了进程运行所需要的一系列关键信息；为每个CPU核抽象出一个任务处理器Processor，记录当前在其上执行的进程。我们还创建了全局任务管理器Task Manager，用于记录正在排队等待运行的进程，便于Processor进行任务的调度。
   + 内存管理。我们对内核和用户拥有的内存空间统一抽象成一个类MemorySet，用于方便地实现对内存的管理。它对外提供了完善的接口，避免了外界对页表的直接操作；我们还扩展了MemorySet的接口，使其能支持mmap和heap区域的管理；最后，我们实现了Lazy Allocation机制，避免一次分配过大的内存，从而减少内存的浪费。
2. 多核运行支持

   + 我们对内核中所用的全局变量(PCB, Frame Allocator, Task Manager, Console等)添加了读写锁(RwLock)，确保了线程安全。互斥锁(Mutex Lock)相比，读写锁支持多核并发的读操作，具有更好的并行度。
   + 文件系统模块也对FAT、文件系统管理器，块缓存等数据结构加锁，保证了多核并行条件下程序的正确性。
3. FAT32文件系统

   + 我们采用五层的分层结构来设计FAT32文件系统，从下到上分别为磁盘块设备接口层、块缓存层、磁盘布局层、文件系统管理层、虚拟文件系统层。
   + 我们沿袭rCore-Tutorial的松耦合模块化设计思路，将FAT32文件系统从内核中分离出来，形成一个独立的Cargo crate。这样，我们就可以单独对文件系统进行用户态测试。在用户态测试完毕后，可直接放到内核中，形成有文件系统支持的新内核。
4. 现实程序运行支持

   + rCore-Tutorial已经提供了27条系统调用，但它们有些是冗余的（如线程、信号量相关的系统调用，我们的内核实现暂时不需要这些功能），且大多数不符合POSIX标准。这使得原生的rCore-Tutorial不能通过Online Judge平台的评测，更不可能支持busybox这类复杂的应用程序。因此，我们修改了并扩展了系统调用的接口，实现了syscall的规范化。
   + 为更好地为现实程序的运行提供支持，我们将系统调用的数量扩充至43个（后续还会增加），也相应地实现了一些机制来支持这些syscall的执行（如mmap机制、信号机制等）
   + 部分复杂的应用程序（如busybox）需要实现更完善的内核机制才能运行。为此我们也做了很多工作，如扩展exec系统调用，将程序运行的必要参数提前压入用户栈；又如开启浮点运算机制、实现进程上下文切换时tp寄存器的保存与恢复等。
5. SBI支持与多核启动

   + 在qemu上,  可使用Rust-SBI或Open-SBI支持内核运行, 并在此基础上实现多核启动
   + 在K210上, 可使用Rust-SBI支持内核运行
   + 在FU740上, 可使用Open-SBI支持内核运行, 并在此基础上实现多核启动
6. 多硬件平台支持

   + 我们在实践中发现，Qemu提供的虚拟硬件环境和真实开发板有诸多细微的差别，这使得能够在Qemu上稳定运行的程序在真实开发板上经常出现各种各样的问题（例如，lazy_static!宏无法在Hifive Unmatched开发板上正常工作，需要更换为Lazy Cell）。因此，对具体开发板的硬件环境进行适配，是我们必须完成的工作。
   + 目前，我们的内核已经完成了对K210和Hifive Unmatched的适配，并且均在在线评测平台上获得满分。对于Hifive Unmatched，我们完成了更多的工作，包括MMIO的调整、SDCard驱动的开发与测试等。
7. 更便捷的调试功能

   + 我们编写了Monitor模块，帮助我们对内核进行更方便的调试。它的基本原理是预留专用内存区域，以其内部数据作为内核调试输出开关。这样，通过GDB修改对应内存的值，就可以达到控制调试目标、输出粒度、调试信息输出是否开启等各类参数的效果。
   + 系统调用追踪是一类极为常用的调试手段，我们的内核也实现了这一功能。内核可为用户反馈系统调用id、名称、输入参数、返回值等信息，用户也可以在user_shell中通过trace命令控制syscall调试信息的输出开关。
8. 更好的用户体验

   + 虽然大赛需要的测试程序不需要user_shell也可以直接执行，但是如果缺少了shell，操作系统就失去了与用户交互的能力，而且也为调试带来了困难。rCore-Tutorial已有一个简单的shell实现，但这个shell使用起来有相当多的不便之处，比如使用左右键命令会出错、使用上下键光标会跑飞，使得命令的输入相当不人性化。
   + 基于此，我们实现了一个功能更加强大、且用户体验更好的shell。它不仅解决了上下左右键按下时命令出错、光标跑飞等问题，还支持tab命令补全、命令历史回溯等功能，为调试带来极大便利。
   + 我们实现了一套较为完整的日志系统，用户可选择最小日志输出等级，并通过debug!，info!，warning!，error!等宏打印不同输出等级的日志信息。日志系统能为不同等级的输出信息设置不同的颜色，且能显示输出语句所在文件的名称和所在行数，使内核输出更加清晰、直观，更利于调试。


## other doc

[sdcard文档](./sdcard.md)

#### dependency中的依赖包
* https://gitee.com/dhz_ggg/fu740-hal
* https://gitee.com/dhz_ggg/fu740-pac
* https://github.com/rust-embedded/    RISC-V 0.7 

[开发文档](./docs/dev.md)

[sdcard-qemu仿真测试仓库](https://gitee.com/dhz_ggg/sdcard_qemu)
