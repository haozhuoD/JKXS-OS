# OSCOMP-2022

## 更新日志

- 2022-3-16
  + 修改了内核WAIT4系统调用的逻辑（添加阻塞模式），使`riscv64`测例中`wait, waitpid, fork`等系统调用能正常运行。

- 2022-3-17
  + 修改了`gettimeofday`和`sleep`系统调用，使其能支持`timeval`结构体。