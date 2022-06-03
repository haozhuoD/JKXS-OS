# FAT32

## BPB(BIOS Parameter Block)

引导记录占据一个扇区，且通常位于0号扇区(逻辑0扇区)

字节偏移(十进制)|字节偏移(十六进制)|字节数|意义
-|-|-|-
0|0x00|3|跳转指令
3|0x03|8|文件系统标志和版本号
11|0x0B|2|每个扇区的字节数
13|0x0D|1|每个簇的扇区数
14|0x0E|2|保留区的扇区数(包含引导记录扇区)
16|0x10|1|FAT表个数(通常为2)
17|0x11|2|0
19|0x13|2|0
21|0x15|1|存储介质
22|0x16|2|0
24|0x18|2|每磁道扇区数
26|0x1A|2|磁头数
28|0x1C|4|EBR分区之前所隐藏的扇区数?
32|0x20|4|文件系统总扇区数

<br/>


## EBR(Extended Boot Record)

EBR紧跟在BPB的后面，二者共占一个扇区

字节偏移(十进制)|字节偏移(十六进制)|字节数|意义
-|-|-|-
36|0x24|4|每个FAT表占用扇区数
40|0x28|2|标志
42|0x2A|2|FAT版本号
44|0x2C|4|根目录所在簇号，通常为2
48|0x30|2|FSInfo扇区号
50|0x32|2|备份引导扇区的扇区号
52|0x34|12|保留
64|0x40|1|驱动号
65|0x41|1|保留
66|0x42|1|0x29
67|0x43|4|卷序列号，忽略
71|0x47|11|卷标
82|0x52|8|"FAT32 "
90|0x5A|420|引导代码
510|0x1FE|2|0xAA55

<br/>

## FSInfo Structure

FSINFO信息扇区一般位于文件系统的1号扇区

字节偏移(十进制)|字节偏移(十六进制)|字节数|意义
-|-|-|-
0|0x0|4|0x41615252
4|0x4|480|保留，0
484|0x1E4|4|0x61417272
488|0x1E8|4|文件系统的空簇数
492|0x1EC|4|最近一次分配的簇号
496|0x1F0|12|保留
508|0x1FC|4|0xAA550000

<br/>

## File Allocation Table(文件分配表FAT)

FAT32文件系统中分配磁盘空间按簇来分配，因此文件在占用磁盘空间时，基本单位是簇，即即使是一个只有一个字节的小文件，操作系统也会给它分配一个簇来存储。而对于大文件则可能需要多个簇来存储，这多个簇在磁盘中不一定是连续存放的。因此需要FAT表来描述簇的分配状态以及表明文件或目录的下一个簇的簇号。

FAT32中每个簇的簇地址为32bits，FAT表中的所有字节位置以4字节为单位进行划分，并对所有划分后的位置由0进行地址编号。0号地址与1号地址被系统保留并存储特殊标志内容。从2号地址开始，每个地址对应于数据区的簇号，FAT表中的地址编号与数据区中的簇号相同。

FAT32使用28位来寻址磁盘上的簇。保留最高的4位。这意味着在读取时应忽略它们，而在写入时应保持不变。

当文件系统格式化时，分配给FAT表的区域会被清空，在FAT1与FAT2的0号表项和1号表项写入对应的特定值，并在2号FAT表项写入一个结束标志表示根目录。

在FAT表中提取簇链中下一个簇的簇号的方法为：
```c
unsigned char FAT_table[sector_size];
unsigned int fat_offset = active_cluster * 4;
unsigned int fat_sector = first_fat_sector + (fat_offset / sector_size);
unsigned int ent_offset = fat_offset % sector_size;

//at this point you need to read from sector "fat_sector" on the disk into "FAT_table".

//remember to ignore the high 4 bits.
unsigned int table_value = *(unsigned int*)&FAT_table[ent_offset];
if (fat32) table_value &= 0x0FFFFFFF;

//the variable "table_value" now has the information you need about the next cluster in the chain.
```
注意如果 `table_value` 大于等于0x0FFFFFF8，则表示簇链中不再有簇了，整个文件已经被读取完。如果 `table_value` 等于0x0FFFFFF7，则表示这个簇存在坏扇区，标记为坏簇。如果 `table_value` 为0，说明对应簇未被分配使用。

由于簇号起始于2号，因此FAT表项的0号表项与1号表项不与任何簇对应，0号表项值总为0xFFFFFFF8，1号表项值总为0xFFFFFFFF


<br/>

## 目录

FAT文件系统中有两种类型的目录，分别是 `Standard 8.3目录项`（出现在所有FAT文件系统上）和 `长文件名目录项` （可选地出现以允许更长的文件名）

<br/>

### Standard 8.3(短文件名)

短文件名目录项的格式如下：

字节偏移|字节数|含义
-|-|-
0|11|文件名，前8个字符是名称(不足8个则用0x20填充)，后3个字符是扩展名(如果是子目录则用0x20填充)
11|1|文件属性，包括READ_ONLY=0x01 HIDDEN=0x02 SYSTEM=0x04 VOLUME_ID=0x08 DIRECTORY=0x10 ARCHIVE=0x20 LFN=READ_ONLY|HIDDEN|SYSTEM|VOLUME_ID
12|1|默认为0，表示短文件名全大写表示（包括扩展名）
13|1|以十分之一秒为单位的文件创建时间
14|2|文件创建时间，其中小时占5bits，分钟占6bits，秒占5bits，秒数需要乘2
16|2|文件创建日期，其中年份占7bits（相对于1980年），月份占4bits，日期占5bits
18|2|文件最近访问日期
20|2|文件起始簇号的高16位
22|2|文件最近修改时间
24|2|文件最近修改日期
26|2|文件起始簇号的低16位
28|4|以字节为单位的文件大小（如果是子目录则全置为0）

<br/>

短文件名目录项的注意点：
- 每个文件或子目录都一定会被分配一个短文件名目录项
- 对于一个短文件名目录项的第一个字符，如果该目录项正在使用中则0x0位置的值为文件名或子目录名的第一个字符；如果该目录项未被使用则0x0位置的值为0x00；如果该目录项曾经被使用过但是现在已经被删除则0x0位置的值为0xE5

<br/>

### Long File Names

长文件名目录项的格式如下：

字节偏移|字节数|含义
-|-|-
0|1|长文件名目录项的序列号
1|10|长文件名的1~5个字符（Unicode编码，每个字符两个字节）
11|1|0x0F
12|1|0
13|1|短文件名的校验和(一个文件的不同长文件名的目录项的校验和相同)
14|12|长文件名的6~11个字符
26|2|0
28|4|长文件名的12~13个字符

<br/>

长文件名目录项的注意点：
 - 长文件名目录项总是有个紧随其后的Standard 8.3目录项。
 - 系统将长文件名以13个字符为单位进行切割，每一组占据一个目录项。所以一个文件可能需要多个长文件名目录项，这时长文件名的各个目录项按倒序排列在目录表中，其第一部分距离短文件名目录项是最近的。
 - 长文件名目录项的第一个字节为序列号。一个文件的第一个目录项序列号为 1，然后依次递增。如果是该文件的最后一个长文件名目录项，则将该目录项的序号与 0x40 进行或（OR）运算的结果写入该位置。如果该长文件名目录项对应的文件或子目录被删除，则将该字节设置成删除标志 0xE5。
 - 长文件名如果结束了但还有未使用的字节，则会在在文件名后先填充两个的 0x00 ，然后开始使用 0xFF 填充。

<br/>

当创建一个长文件名文件时，其短文件名的命名原则为：
 - 取长文件名的前6个字符加上”~1”形成短文件名，扩展名不变
 - 如果已存在这个文件名，则符号”~”后的数字递增，直到5
 - 如果文件名中"~"后面的数字达到5，则短文件名只使用长文件名的前两个字母。
 通过数学操纵长文件名的剩余字母生成短文件名的后四个字母，然后加后缀"~1"直到最后(如果有必要，可以是其他数字以避免重复的文件名)。

<br/>

### "."目录项和".."目录项

一个子目录的起始簇中的前两个目录项为"."目录项和".."目录项，注意"."目录项中记录的起始簇号也就是该子目录目前所处的位置。

<br/>


# 相关操作

## 读取引导扇区

读取BPB：
```c
typedef struct fat_BS
{
	unsigned char 		bootjmp[3];
	unsigned char 		oem_name[8];
	unsigned short 	    bytes_per_sector;
	unsigned char		sectors_per_cluster;
	unsigned short		reserved_sector_count;
	unsigned char		table_count;
	unsigned short		root_entry_count;
	unsigned short		total_sectors_16;
	unsigned char		media_type;
	unsigned short		table_size_16;
	unsigned short		sectors_per_track;
	unsigned short		head_side_count;
	unsigned int 		hidden_sector_count;
	unsigned int 		total_sectors_32;

}__attribute__((packed)) fat_BS_t;
```

读取EBR：
```c
typedef struct fat_extBS_32
{
	//extended fat32 stuff
	unsigned int		table_size_32;
	unsigned short		extended_flags;
	unsigned short		fat_version;
	unsigned int		root_cluster;
	unsigned short		fat_info;
	unsigned short		backup_BS_sector;
	unsigned char 		reserved_0[12];
	unsigned char		drive_number;
	unsigned char 		reserved_1;
	unsigned char		boot_signature;
	unsigned int 		volume_id;
	unsigned char		volume_label[11];
	unsigned char		fat_type_label[8];
 
}__attribute__((packed)) fat_extBS_32_t;
```
有了上述的信息后，则有

总扇区数：
`total_sectors = fat_boot->total_sectors_32；`

每个FAT表占用扇区数：`fat_size = fat_boot_ext_32->table_size_32;`

第一个数据扇区(即根目录的第一个扇区号)：`first_data_sector = fat_boot->reserved_sector_count + (fat_boot->table_count * fat_size);`

FAT表中的第一个扇区:
`first_fat_sector = fat_boot->reserved_sector_count;`

数据扇区的总数：
`data_sectors = fat_boot->total_sectors - fat_boot->reserved_sector_count - fat_boot->table_count * fat_size;`

簇的总数（向下取整）:
`total_clusters = data_sectors / fat_boot->sectors_per_cluster;`

<br/>

<br/>

## 读取目录

读取目录的第一步就是找到并读取根目录。在FAT32中，根目录出现在给定簇的数据区中：
`root_cluster_32 = fat_boot_ext_32->root_cluster;`

对于给定的簇号cluster，其第一个扇区的位置为：`first_sector_of_cluster = ((cluster - 2) * fat_boot->sectors_per_cluster) + first_data_sector;`

### ls

对于簇中的每个32字节的条目，有：

1. 如果条目的第一个字节等于0，则此目录中没有文件/目录。即FirstByte==0，结束； FirstByte!=0，转到2。
2. 如果条目的第一个字节等于0xE5，说明这个条目不再使用。即FirstByte==0xE5，转到8; FirstByte!=0xE5，转到3。
3. 检查该条目是否是长文件名目录项，如果该条目的第11个字节等于0x0F，则是长文件名目录项，否则不是。即11thByte==0x0F，转到4； 11thByte!=0x0F，转到5。
4. 将长文件名的部分读到临时缓冲区中，转到8
5. 解析短文件名目录项中的数据，转到6
6. 在临时缓冲区中是否有长文件名，如果有则转到7，否则转到8
7. 临时缓冲区的长文件名就是刚刚解析的短文件名目录项的文件名，清除临时缓冲区，转到8
8. 递增指针和计数器读取下一个条目

重复上述操作直至簇中所有的条目都被读出

<br/>

# 参考资料

https://wiki.osdev.org/FAT

https://blog.csdn.net/li_wen01/article/details/79929730

https://blog.csdn.net/u010650845/article/details/60780979


