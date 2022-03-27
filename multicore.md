## QEMU启动多核
```shell
# 启动双核
make run CPUS=2
```

## 如何用gdb调试多核

https://stackoverflow.com/questions/42800801/how-to-use-gdb-to-debug-qemu-with-smp-symmetric-multiple-processors

```
i th 查看线程信息
thr x 切换到x号线程
```

## 初步尝试

将`UPSafeCell`全部替换为`Mutex`，观察到现象：cpu0正常启动，轮到cpu1运行任务时会报错：no tasks available in run_tasks。
之后内核疯狂`panic`，输出字母顺序是乱的。

## console加锁

结果是cpu0直接卡死（卡在console那里），cpu1由于cpu0未初始化完成也卡死。
用`Mutex<ConsoleInner>`代替`Arc<Mutex<ConsoleInner>>`，问题解决。我也不知道为啥....

## 能跑，但貌似不太对
