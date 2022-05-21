## SDcard驱动

其实一开始并没有想要自己实现一个SDcard驱动，但实在找不到可以扒出来用的Rust hifive-sdcard驱动，只好自己动手写一个纯Rust的SDcard驱动。顺带还实现了一些简单的时钟控制模块。

这一个过程虽然不是很难，但是比较繁琐（容易疏忽协议和硬件手册的一些细节），在驱动这一方面还是下了很多功夫。

整个实现过程大概分为三个步骤：

* 基于fu740-pac实现SPI驱动
* 基于SPI驱动实现SPI-SDcard驱动
* 基于实现简单的时钟控制模块

关于一些SPI-SDcard协议相关内容的讲解与介绍，华科xv6-k210的文档已经写的非常详细了，这里就不再过多赘述。

这里主要讲我们实现的整体逻辑与一些比较关键的点。

一些与hifive-SDcard驱动相关的链接在最后的参考文档列表中。

### 整体架构：

尽量保持rCore-tutorial中SDcard驱动的整体机构和接口不变，对于最上层

![](image/SD卡驱动/1652331997475.png)![](image/SDcard驱动/1652359448039.png)

### 已有的FU740-PAC包

write()方法可能存在一些问题，需使用modify()来进行修改指定寄存器的值。

### 基于PAC实现SPI协议

参考K210-soc和rCore-tutorial抽象出来的SPI操作接口，接口基本保持一致。

```rust
pub trait SPI {  
    fn init(&self);  	//初始化/复位SPI相关寄存器
    fn configure(	//配置SPI相关寄存器
        &self,
        protocol: u8, 
        endianness: bool,
        cs_active_high: u32,
        csid: u32,
    );
    fn set_clk_rate(&self, div: u32) -> u32;			// 设置SPI时钟频率
    fn recv_data(&self, chip_select: u32, rx: &mut [u8]);	//接受数据
    fn send_data(&self, chip_select: u32, tx: &[u8]);		//发送数据
    fn fill_data(&self, chip_select: u32, value: u32, tx_len: usize);//未实现
    fn switch_cs(&self, enable: bool, csid: u32);		//片选与SPI-csmode设置
}
```

### 基于SPI协议实现SDcard驱动

主要参考[xv6-k210团队编写的SD卡相关文档](https://qf.rs/2021/05/20/%E5%9F%BA%E4%BA%8ESPI%E6%A8%A1%E5%BC%8F%E7%9A%84SD%E5%8D%A1%E9%A9%B1%E5%8A%A8.html)和rCore-tutorial中SPI-SDcard协议的实现。

SDcard驱动接口和相关抽象基本与rCore-tutorial中保持一致。

### 基于PAC的时钟控制模块

参考fu740-hal，基于读写PLL相关j寄存器，实现对时钟频率的读取和设置。(暂未使用)

### qemu 仿真SDcard

qemu支持对真实板子fu540/hifive unleasd 进行仿真， 而经过阅读硬件手册发现fu540/fu740对于SPI协议的实现完全一致(甚至MMIO的映射地址范围也是一致的)，所以我们可以先在qemu上简单测试好我们的SDcard驱动之后在进一步上板进行测试。

在一个简单内核中直接调用SDcard驱动进行读写SDcard的I/O操作，对比得到的数据进行验证(基于徐文浩同学的SDcard-qemu仿真测试仓库进行修改)。

使用qemu-6.2.0进行SDcard驱动仿真测试的流程：

* 创建SDcard镜像
* 启用qemu 的SPI-SDcard功能进行仿真

部分碰到的SDcard问题与解决方案如下：

* CMDFailed(CMD0, 0) 发送CMD0失败
  * 解决方案： 多次发送，设定最大发送次数。若成功设置则跳出循环
* qemu仿真的SDcard为标准SDcard(按字节寻址)，完善SDcard驱动。对stan。。和HC进行分别处理，同时适配。巴拉巴拉

## todo完善内容


### SDcard驱动上板：

init失败，send CMD0 无响应（时钟频率问题，初始化时时钟频率不能过高）。

原k210-SDcard驱动的一些不足导致：write_sector失败（写扇区之后读取扇区出错）：response的问题，在完成写入数据的传输后需要等待几个response，直到SDcard回到idle状态。在华科文档中也有写到，在完成write_sector的数据传输后需要发送cmd13确定已经完成写操作。这里使用的时比较取巧的方法，循环直到返回值为0xff，即等待sdcard变为空闲状态。

busybox 读取过慢   优化减小div的值，提高SPI频率：div=2 or 3 （最好不要修改pclk的pll分频，因为串口使用的也是plck）。

### 可以优化的地方

当前实现了连续读写多个扇区但并未测试，且fat32文件系统内部的I/O仍然以块为单位。

todo ： DMA？ 在fat32文件系统调用blockdevice时，不再默认只读一块。

### fu740 Schematics 中的一个小错误：

[HiFive Unmatched Schematics (prismic.io)](https://sifive.cdn.prismic.io/sifive/6a06d6c0-6e66-49b5-8e9e-e68ce76f4192_hifive-unmatched-schematics-v3.pdf)

此手册第5页，如下图所示表明SDcard使用的是SPI0 ， 而实际上使用的为SPI2(和fu540中一致)

![](image/SDcard驱动/1652360475313.png)

### 主要的参考实现 :

## todo 完善相关链接

Rust实现的fu740-pac ： https://github.com/riscv-rust/fu740-pac
Rust实现的fu740-hal  ：https://github.com/riscv-rust/fu740-hal    (相关SPI协议抽象并未实现)

hifive-SDK ,

rCore-tutorial中K210的SDcard驱动

Linux中hifive的SPI驱动

参考文档：

[xv6-k210团队编写的SD卡相关文档](https://qf.rs/2021/05/20/%E5%9F%BA%E4%BA%8ESPI%E6%A8%A1%E5%BC%8F%E7%9A%84SD%E5%8D%A1%E9%A9%B1%E5%8A%A8.html)

SDcard官方手册

fu740硬件手册 https://www.sifive.com/boards/hifive-unmatched

特别感谢许文浩同学在SDcard驱动编写方面提供的支持与帮助
