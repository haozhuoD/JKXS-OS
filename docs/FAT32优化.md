# FAT32优化

## 内存SD卡镜像

* 由于SD卡镜像已放置到内存0x90000000位置处，因此访问SD卡内容无需通过SD卡驱动，而是直接访问对应位置的内存即可。因此原来的五层文件系统架构中的较低两层块设备接口层和块缓存层可以舍弃。我们重新设计了 `fsimg`模块用于计算给定块号在内存中的位置，并向上暴露原有的接口对其进行互斥访问。
* 相较于原来块缓存层的 `BlockCacheManager`和 `BlockCache`的双锁设计，现在可以去除掉 `BlockCacheManager`的锁，但是由于对对应内存区域的访问必须是互斥的，因此 `BlockCache`的锁需要保留。同时我们也优化了检索对应块号的 `BlockCache`的速度。

## 目录项查找优化

- 由于SD卡镜像已放置到内存中，因此现在在进行目录项的性质判断之类的操作时，无需进行拷贝工作，而是可以直接对对应内存的值进行判断。在 `layout`层中原来的接口 `read_at`会进行内存拷贝工作，所以我们新增了接口 `find_short_name`用于短文件名目录项的搜索，`find_free_dirent`用于空闲目录项的搜索。
- 同时我们以更大粒度即一个块 `type DirentBlock = [ShortDirEntry; BLOCK_SZ / DIRENT_SZ]`而不是单个目录项去访问内存中的内容，从而能够减少锁的获取次数。

## 文件和目录的查找和创建优化

* 在进行多级目录查找时，会依次对各级目录都进行查找，这样的查找效率十分慢。为此，我们在内核中加入了文件(目录)索引机制，在进行文件和目录的查找时，会通过 `find_vfile_idx`优先从文件(目录)索引中进行寻找，如果找到了则可以直接得到对应文件或目录的 `vfile `。如果找不到，则正常地调用 `find_vfile_path`进行多级目录查找，并在找到之后将其加入到文件(目录)索引中。同样地，对文件和目录remove后，也需要在索引中去除对应的内容。
* 不存在的文件：对于一个不存在的文件，在引入了索引之后，查找的效率反而变慢了，因为多引入了一层的判断。为了加快不存在的文件的检索速度，我们退一步获取了该文件的父级目录。很显然，通过索引找到其父级目录，再通过 `find_vfile_name`找某个文件，比通过 `find_vfile_path`去找该文件更快。因此现在查找一个文件或目录的流程如下：
  1. 使用 `find_vfile_idx`通过索引直接查找该文件或目录；
  2. 使用 `find_vfile_idx`查找其父级目录，再通过 `find_vfile_name`在父级目录下查找该文件或目录；
  3. 使用 `find_vfile_path`通过路径查找该文件或目录。
* 需要注意的是，如果通过索引找到了其父级目录，但通过 `find_vfile_name`找不到的话，就不需要进行第3步了，这样才能加快不存在的文件的查找效率。当然，如果通过索引找不到父级目录，则还需进行第三步。即 `find_vfile_name`和 `find_vfile_path`是完备的，`find_vfile_idx`是不完备的。

## 簇链优化

簇链的优化主要有两个原因：一是如果将整个FAT表当作一个整体对象并利用读写锁进行互斥访问，当对两个文件分别进行读和写时，虽然这两个文件所拥有的簇链是不同的，但读操作仍然会被写操作阻塞。二是FAT表是链式存储结构，读取某个簇链上的第n个簇的簇号只能O(n)顺序查找，如果能够随机访问的话，则会对性能有较大的提升。

综合上述原因，结合我们设计的FAT表的两个主要接口：

```rust
get_next_cluster(cluster) // 查询簇cluster的下一个簇
get_cluster_at(first_cluster, index) // 查询以first_cluster为首的簇链中的第index个簇
```

可以看出，我们不仅要用cluster，也要用index对簇链进行访问。由此得到了我们的设计：

```rust
struct Chain {
    chain: Vec<u32>,
    chain_map: HashMap<u32, usize>
}
```

其中 `chain`按序维护文件一条簇链上的所有簇号，事实上就是index到cluster的映射；`chain_map`维护簇号cluster到 `chain`中cluster所在index的映射。

`Chain`所暴露出的接口基本与 `fat`一致，当试图通过 `Chain`对簇链进行查询时，需要先通过 `chain_map`来判断first_cluster是否在 `chain`中，在的话可以进行下一步查询。否则需要通过fat表进行查询并更新 `Chain`。思路同目录项查询一样，我们以更大的粒度即 `FatentBlock`去访问内存中的内容。

```
const FAT_ENTRY_PER_SEC: u32 = BLOCK_SZ as u32 / 4;
type FatentBlock = [u32; FAT_ENTRY_PER_SEC as usize];
```

分配簇时，我们不仅更新fat表，也同步更新对应的 `Chain`。对于 `set_next_cluster(curr_cluster, next_cluster)`接口而言，主要有三种情况：

- 之前没有分配过簇，此时只分配一个簇：next_cluster为END_CLUSTER。
- 之前没有分配过簇，此时分配若干个簇：将各个簇连接成一条簇链，最后一个簇的next_cluster为END_CLUSTER。
- 之前分配过簇，此时新分配一个或若干个簇：先将之前已分配的簇链和新分配的第一个簇连接起来，再将新分配的各个簇连接成一条簇链，最后一个簇的next_cluster为END_CLUSTER。

可以得出 `set_next_cluster`时 `chain`的更新规则：

- 当 `chain`为空时，将curr_cluster加入 `chain`中；
- 当next_cluster不为FREE_CLUSTER和END_CLUSTER时，将next_cluster加入 `chain`中。
