# mmap

1. 修改`memoryset`内容，增加记录已分配的堆内存和`mmap`内存的数据结构。
2. 修改用户栈基地址和`mmap`基地址，见`config.rs`
3. 修改`open`系统调用为`open_at`，增加`fstat`系统调用。
4. 修改文件描述符的表示，由原来的`File`类型转换为`Abstract + OSFile`类型，后者支持获取文件大小等操作。
5. 以`lazy allocation`的方式，将实际分配`mmap`和`heap`的内存延迟到缺页中断发生时，见`trap/page_fault.rs`

! todo: lazy heap

! todo: CopyOnWrite

! todo: fork子进程时，复制堆内存和`mmap`内存

! todo: munmap时写回文件

! todo: mmap_anonymus