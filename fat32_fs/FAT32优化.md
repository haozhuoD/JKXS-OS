# FAT32 优化比较

## 5层结构
 - 磁盘块设备接口层
 - 块缓存层
 - 磁盘数据结构层
 - 磁盘块管理器层
 - 索引节点层

<br/>

## 磁盘块设备接口层

没有变动

<br/>

## 块缓存层

主要变动如下：

1. 未实现：导入`use riscv::register::time;`，其中`time`是RISC-V架构下的一个CSR，用于记录CPU自复位以来共运行了多长时间，猜测想要实现LRU替换策略，但最后去除了。

2. 已实现：将锁由`use spin::Mutex;`改成`use spin::RwLock;`。`RwLock`读写锁支持多个读者或一个写者，但仍然是写者不公平的，即如果总是有读者，那么写者永远无法占有锁。`RwLock`的主要方法如下：

     - `read()`：获取读锁
     - `write()`：获取写锁
     - `upgradeable_read`：获取一个可升级的读锁，获取到后无法获取写锁和可升级的读锁，同时不允许有新的读锁被获取。该可升级的读锁后续可以通过`upgrade`方法升级为写锁。从而可以缓解写者饥饿问题。

3. 未实现：rCore中的缓存块有16个，UltraOS中的缓存块增加到20个(虽然常量`BLOCK_CACHE_SIZE`是这么定义的，但事实上像是由`BlockCacheManager`的`limit`属性来决定)。

4. 已实现：采用双缓存，即信息缓存和数据缓存，其中每个缓存各有10个扇区，信息缓存用于缓存存储检索信息的块，例如文件系统信息扇区、FAT、目录等，数据缓存则用于缓存文件的数据。在全局实例时实例化了两个块缓存全局管理器`DATA_BLOCK_CACHE_MANAGER`和`INFO_CACHE_MANAGER`。因此在向其他模块暴露公共函数时，也需要暴露`get_block_cache`和`get_info_cache`两个函数，分别用来访问文件数据块和访问保留区及目录项。

5. 结构体`BlockCacheManager`新增两个成员`start_sec`和`limit`，一些方法。

     - 已实现：`start_sec`属性表示起始扇区号，似乎是为了支持分区磁盘，通过`set_start_sec`方法来设置起始扇区，在设置起始扇区后，上层模块就可以不用考虑分区的起始偏移，从而只用传入逻辑扇区号，缓存层会自动加上`start_sec`从而得到物理扇区号。
     - 未实现：`limit`属性应该是表示缓存块的个数，在创建`BlockCacheManager`实例调用`new()`时传入，但这样看来两个块缓存全局管理器在实例化时都应该传入10，表示两个缓存都各有10个扇区。但实际上实例化`INFO_CACHE_MANAGER`时传入了10，但在实例化`DATA_BLOCK_CACHE_MANAGER`时传入了1034。
     - 已实现：方法`read_block_cache`同方法`get_block_cache`类似，区别在于在缓存中找到对应`block_id`的缓存块时，返回这个缓存块引用的Some封装，否则返回None而不从磁盘中读取到缓存中。需要`read_block_cache`方法的原因是使用了读写锁，获取读锁时返回的是不可变引用，而`get_block_cache`需要的是可变引用。另外如果调用`read_block_cache`返回的是None，说明磁盘块没在缓存中，则需要获取`BlockCacheManager`的写锁并调用`get_block_cache`方法将磁盘块读入缓存中，再调用一次`read_block_cache`即可。

6. 已实现：使用了读写锁，因此需要判断访问的方式，于是在上述两个公共函数的参数中都增加了`rw_mode`参数，取值为枚举变体`CacheMode::READ`和`CacheMode::WRITE`分别表示读和写。在函数中根据rw_mode的取值来分别获取读锁和写锁。

<br/>

## 磁盘数据结构层

TODO_list: ShortDirEntry的checksum方法

<br/>

### rCore

1. SuperBlock

     - initialize：对超级块进行初始化，传入的数据由上层计算
     - is_valid

2. Bitmap：常驻内存,保存了它磁盘结构所在区域的起始块编号`start_block_id`和区域的块长度`blocks`，对位图的修改均通过缓存块来进行

     - new
     - alloc：分配一个bit，通过`get_block_cache`获取块缓存来修改
     - dealloc：回收一个bit时同样通过`get_block_cache`获取块缓存来修改

3. BitmapBlock

4. DiskInode

     - initialize
     - is_dir
     - is_file
     - get_block_id：由于需要读取索引块，因此需要`get_block_cache`
     - increase_size：扩充容量，给上层磁盘块管理器调用
     - clear_size：回收容量，给上层磁盘块管理器调用
     - read_at、write_at：读写磁盘中的数据

5. DirEntry

     - empty
     - new
     - name
     - inode_number

### 现在的思路

1. UltraOS仅维护了FSInfo的内存结构，仅保存FSInfo所在的扇区号，对FSInfo的读出和写入均采用缓存块来进行，优点是不需要加锁和不用维护FSInfo的写入，通过BlockCache在drop时就会写入磁盘。缺点是对FSInfo读写时需要获取整个BlockCacheManager的读写锁，会影响其他缓存块的读写。

     现在的思路是对FSInfo实现完整的结构体与磁盘数据一一对应，并常驻内存中，相当于给FSInfo一个单独的缓存块。注意点是修改结构体字段的值时需要添加读写锁，同时还需要维护写入磁盘。


<br/>

## 磁盘块管理层

### rCore

EasyFileSystem

 - create：创建并初始化一个文件系统
 - open：打开文件系统
 - get_disk_inode_pos
 - get_data_block_id
 - allow_inode
 - alloc_data
 - dealloc_data

<br/>

## VFS

### Inode

 - new
 - read_disk_inode
 - modify_disk_inode
 - find
   - find_inode_id
 - increase_size
 - create
 - ls
 - read_at
 - write_at
 - clear


