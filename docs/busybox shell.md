# busybox shell

# 遇到的问题

1. 执行时遇到了写缺页错误。
```
[kernel] Exception(StorePageFault) in application, bad addr = 0x0, bad instruction = 0xc60ca, kernel killed it.
```

特点似乎是只要while read line就会出错。