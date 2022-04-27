## 参考资料与文档

fu740的官网： 下面有各种硬件文档的下载链接，主要看的是fu740-c000-manual
https://www.sifive.com/boards/hifive-unmatched

已有的rust实现的硬件抽象包：
https://github.com/riscv-rust/fu740-pac
https://github.com/riscv-rust/fu740-hal

linux的hifive-SPI驱动：
https://github.com/torvalds/linux/blob/master/drivers/spi/spi-sifive.c

官方基于sifive一个裸机实现的一个lib:
https://github.com/sifive/freedom-metal/tree/1cec4a23a7ed7350db79a392be65acd51acd5412

从想扒一个SDcard驱动 -> 自己实现一个SDcard驱动

## K210 SDcard驱动的rust实现

### SPI driver

主要实现三个函数

* config ： 配置SPI协议参数
* send   ： 通过SPI协议发送数据
* recv   ： 通过SPI协议接收数据

目前存在的问题:

* spi config 时是否需要spi_clk、speed_hz
* spi config初始化时确定具体流程，目前已将所有参考资料的实现流程实现

### clock/pll

主要实现获取时钟频率和设置时钟频率功能

目前存在的问题：

* 仅仅可以设置外围串行设备 如SPI 的时钟频率 hfpclk
* todo 支持设置core-clk

### GPIO

直接拉高/拉低 GPIO端口

目前存在的问题：

* GPIO端口号为几？是否真的需要CPIO控制？
* 目前sdcard设置的频率参数   暂时设置为k210中预定的频率    todo


详情请看 os/src/drivers/blocks/   dependency/hal/clock