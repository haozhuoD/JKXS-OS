//! SPI peripherals handling
// use core::cmp;
// use core::convert::TryInto;

use fu740_pac as pac;
use pac::SPI2; //,SPI1,SPI0
               // use fu740_hal as hal;
               // use hal::clock::{Clocks,ClockSetup};

/// todo
/// ？ fmt::dir 在发送或者接受数据的时候
/// ？ set_spi-clk rate  ？ speedHz  
/// ？ configure 使dir = 0   -> ?
/// ?  switch_cs 时 关闭应置为 0 or 3 ? auto/off
/// unleased : SPI2
/// umatched : SPI0

/// Extension trait that constrains SPI peripherals
// pub trait SPIExt: Sized {
//     /// Constrains SPI peripheral so it plays nicely with the other abstractions
//     fn constrain(self) -> SPIImpl<Self>;
// }

//todo 可能不叫01，01代表当前soc作为master的spi端口号，一般可能还有
//spi0 cs位只有一位  ， 而spi1 cs位有两位
// 暂时只选SPI0
// pub trait SPI0m: Deref<Target =spi0::RegisterBlock>{
// }

// pub trait SPI1m: Deref<Target =spi0::RegisterBlock>{
// }

// pub trait SPI2m: Deref<Target =spi0::RegisterBlock>{
// }

// impl<SPI: SPI2m> SPIExt for SPI {
//     fn constrain(self) -> SPIImpl<SPI2> {
//         SPIImpl::<SPI>::new(self)
//     }
// }

pub struct SPIImpl {
    pub spi: SPI2,
}

// /** Trait for trunction of a SPI frame from u32 register to other unsigned integer types. */
// pub trait TruncU32 {
//     fn trunc(val: u32)-> Self;
// }
// impl TruncU32 for u32 { fn  trunc(val: u32) -> u32 { return val; } }
// impl TruncU32 for u16 { fn  trunc(val: u32) -> u16 { return (val & 0xffff) as u16; } }
// impl TruncU32 for u8 { fn  trunc(val: u32) -> u8 { return (val & 0xff) as u8; } }

pub trait SPI {
    fn init(&self);
    //todo 相关的寄存器和功能位
    fn configure(
        &self,
        // spi_clk:u32,
        // speed_hz: u32,
        protocol: u8, // use_lines
        endianness: bool,
        cs_active_high: u32,
        csid: u32,
    );
    // fn set_clk_rate(&self, spi_clk: u32, speed_hz: u32) -> u32;
    fn set_clk_rate(&self, div: u32) -> u32;
    // fn recv_data<X: TruncU32>(&self, chip_select: u32, rx: &mut [X]);
    fn recv_data(&self, chip_select: u32, rx: &mut [u8]);
    // fn send_data<X: Into<u32> + Copy>(&self, chip_select: u32, tx: &[X]);
    fn send_data(&self, chip_select: u32, tx: &[u8]);
    fn fill_data(&self, chip_select: u32, value: u32, tx_len: usize);
    // fn fill_data_dma(&self, dmac: &DMAC, channel_num: dma_channel, chip_select: u32, value: u32, tx_len: usize);
    // fn recv_data_dma(&self, dmac: &DMAC, channel_num: dma_channel, chip_select: u32, rx: &mut [u32]);
    // fn send_data_dma(&self, dmac: &DMAC, channel_num: dma_channel, chip_select: u32, tx: &[u32]);
    fn switch_cs(&self, enable: bool, csid: u32);
}

impl SPIImpl {
    pub fn new(spi: SPI2) -> Self {
        Self { spi }
    }
}

impl SPI for SPIImpl {
    fn init(&self) {
        println!("[SPI] SPIImpl  init - start");
        // init；
        // 参考 https://github.com/sifive/freedom-metal/blob/master/src/drivers/sifive_spi0.c :: void __metal_driver_sifive_spi0_init()
        // todo ??? about-clock  是否需要一开始就初始化时钟？ 多余的一步(还导致麻烦的重复传参    待修改
        // self.set_clk_rate(spi_clk,speed_hz); //须输入spi_clk
        // todo 使能gpio  ???

        // linux sifive-spi sifive_spi_init()
        unsafe {
            self.spi.ie.modify(|_, w| w.bits(0));
            // self.spi.ie.write(|w| w.txwm(0b1));
            // self.spi.ie.write(|w| w.rxwm(0b0));
            // warter mark

            // 测试对寄存器的操作是否有效
            // let ie_r =  self.spi.ie.read().txwm().bits();
            // println!("before ie_r:: {:#?}",ie_r);
            self.spi.ie.modify(|_, w| w.txwm().bit(true));
            // let ie_r =  self.spi.ie.read().txwm().bits();
            // println!("before ie_r:: {:#?}",ie_r);
            self.spi.ie.modify(|_, w| w.rxwm().bit(false));

            self.spi.delay0.modify(|_, w| w.cssck().bits(1));
            self.spi.delay0.modify(|_, w| w.sckcs().bits(1));
            // self.spi.delay1.modify(|_,w| w.intercs().bits(1));
            // self.spi.delay1.modify(|_,w| w.interxfr().bits(0));

            // 关闭memory-mapped SPI flash mode    u540::unleasd 未实现
            // let fctrl_r =  self.spi.fctrl.read().en().bits();
            // println!("before fctrl:: {:#?}",fctrl_r);
            // self.spi.fctrl.modify(|_,w| w.en().set_bit());
            self.spi.fctrl.modify(|_, w| w.en().clear_bit());
            // let fctrl_r =  self.spi.fctrl.read().en().bits();
            // println!("after fctrl:: {:#?}",fctrl_r);
        }
        println!("[SPI] SPIImpl init - finish");
    }
    /// 未测试
    fn configure(
        &self,
        /*
        * 参考
        * https://github.com/sifive/freedom-metal/blob/master/src/drivers/sifive_spi0.c
        * https://github.com/snow107/HiFive-BareMetal-SPI/blob/master/freedom-metal/metal/spi.h
        * linux sifive-spi-driver
        */

        // spi_clk:u32,
        // speed_hz: u32,
        // freedom spi-config
        // 设为默认值rst       // 对应spi传输的四种模式
        //  /* Set Polarity */
        //  polarity: polarity,  //sckmode  极性   =CPOL POL    1bit  rst=0
        //  /* Set Phase */
        //  phase: phase,        //sckmode CPHA PHA             1bit rst=0
         /* Set protocol */
        protocol: u8, //Frame Format Register (fmt)  2bit 协议  -- 先默认单协议single  或者2？
        /* Set Endianness */
        endianness: bool, //Frame Format Register (fmt)  1bit 大小端
        /* Always populate receive FIFO */ //？？？
        /* Set CS Active */ //CSDEF csdef  rst=1
        cs_active_high: u32,
        /* Set frame length */
        // ???

        /* Set CS line */  //csid  rst= 0
        // 多余，在发送的时候会设置
        csid: u32, // = 0

                   /*Toggle off memory-mapped SPI flash mode, toggle on programmable IO mode*/
                   //SPI Flash Interface Control Register (fctrl)

                   // /*! @brief The chip select ID to activate for the SPI transfer */
                   // unsigned int csid;
                   // /*! @brief The spi command frame number (cycles = num * frame_len) */
                   // unsigned int cmd_num;
                   // /*! @brief The spi address frame number */
                   // unsigned int addr_num;
                   // /*! @brief The spi dummy frame number */
                   // unsigned int dummy_num;
                   // /*! @brief The Dual/Quad spi mode selection.*/
                   // enum {
                   //     MULTI_WIRE_ALL,
                   //     MULTI_WIRE_DATA_ONLY,
                   //     MULTI_WIRE_ADDR_DATA
                   // } multi_wire;
    ) {
        // protocol (QSPI, DSPI, SPI)
        // fmt::protocol/endian   dir=0
        // csdef   csid

        unsafe {
            self.spi.fmt.modify(|_, w| w.proto().bits(protocol));
            self.spi.fmt.modify(|_, w| w.endian().bit(!endianness));
            self.spi.fmt.modify(|_, w| w.dir().clear_bit());

            self.spi.csdef.modify(|_, w| w.bits(cs_active_high));
            self.spi.csid.modify(|_, w| w.bits(csid));
        }
    }

    /// 未测试  spi_clk = pclock
    /// 目前输入： pclk 和 想要的速度hz
    /// Set SPI clock rate 根据输入频率设置波特率设置,返回时钟频率/spi波特率  
    // fn set_clk_rate(&self, _spi_clk: u32, _speed_hz: u32) -> u32 {
    fn set_clk_rate(&self, div: u32) -> u32 {
        // 先获取时钟 pclk:hfpclk_pll 假定已经通过ClockSetup设置好时钟
        // 按k210： 基于输入时钟频率和串行外设时钟频率计算出波特率
        // todo 需要一个初始化好的时钟(在sdcard驱动中初始化一个) X // let clocks = ClockSetup.freeze();
        // let pclk = clocks.pclk();
        //      从pric中获取 hfpclk-pll  -> 如何计算其频率呢？     直接使用一个指向pac::prci的指针再解引用调用set_clock()
        // let spi_baudr = pclk.0 / spi_clk;
        // linux                       按手册说明输入频率为pclk
        // let mut div = spi_clk / speed_hz / 2 - 1;
        // div = div & 0xfff;

        // let div = 3000;

        // Clamp baudrate divider to valid range
        //panic!("{} / {} = {}", clock_freq, spi_clk, spi_baudr);
        // let spi_baudr = cmp::min(cmp::max(spi_baudr, 2), 65534);
        // let div = (pclk.0 / (2 * spi_baudr)) - 1;
        // assert!(div <= 4096);

        // let div =;
        unsafe {
            //todo sckdiv `div` Field only [11:0] 12bit
            self.spi.sckdiv.modify(|_, w| w.bits(div));
        }
        //
        // spi_clk / 2*(div+1)
        div
    }

    // 如何分离一次trans的两次 transfer mode : 不分离send和recv都是一次完整的数据交换
    // 不处理time out ，死循环等
    // 参考 linux-source code
    // todo 目前做法：fmt::dir 置为Rx:0  ,永远交换     参考: sifive/freedom-metal
    //       另一些可能： 在单独发送数据时是否需要将fmt::dir 置为Tx:1 使其不填充接收fifo
    //                  linux-设置水位线为: 发送次数减一  不断读取IP ,直到发生中断
    fn send_data(&self, chip_select: u32, tx: &[u8]) {
        //Into<u32> +
        // println!("[SPI driver] send_data  csid:{:#?} \n  tx: {:#?} ",chip_select,tx);
        // let mut remaining_byte = tx.len();
        unsafe {
            // set direction
            self.spi.fmt.modify(|_, w| w.dir().set_bit());
            // chip_select
            self.spi.csid.modify(|_, w| w.bits(chip_select));
            // csmode: hold mode
            self.spi.csmode.modify(|_, w| w.mode().bits(2));
        }
        //hold mode:2  |  auto:0 | off:3
        //在 txdata 为空的时候不断将数据填入
        // 队列只有八个字节
        // for &val in tx {
        //     println!("[SPI driver] send_single_data  val: {:#?} ",val);
        //     while self.spi.txdata.read().full().bits()  {
        //     }
        //     unsafe{
        //         self.spi.txdata.modify(|_,w| w.data().bits(val));
        //     }
        //     while self.spi.rxdata.read().empty().bits() {
        //         //死等
        //     }
        //     //不存储得到的信息
        // }
        let len = tx.len();
        let mut remaining = len;
        while remaining != 0usize {
            // words need to be transferred in a single round
            let n_words = if 8usize < remaining { 8 } else { remaining };
            // set watermark
            unsafe {
                self.spi.txmark.modify(|_, w| w.bits(1));
            }
            // wait for spi
            // TODO implement yielding in wait
            // self.tx_wait();
            while !self.spi.ip.read().txwm().bits() {
                // loop
            }
            // enque spi
            for _ in 0..n_words {
                // self.tx_enque(tx_buf[len - remaining]);
                unsafe {
                    self.spi
                        .txdata
                        .modify(|_, w| w.data().bits(tx[len - remaining]));
                }
                remaining = remaining - 1;
            }
        }

        // 释放csmode
        unsafe {
            self.spi.csmode.modify(|_, w| w.mode().bits(0)); //hold mode:2  |  auto:0 | off:3
        }
    }

    fn recv_data(&self, chip_select: u32, rx: &mut [u8]) {
        //csmode: hold mode
        // let mut remaining_byte = rx.len();
        unsafe {
            // clear fmt::direction
            self.spi.fmt.modify(|_, w| w.dir().clear_bit());
            self.spi.csid.modify(|_, w| w.bits(chip_select));
            self.spi.csmode.modify(|_, w| w.mode().bits(2)); //hold mode:2  |  auto:0 | off:3
        }

        //在 txdata 为空的时候不断将数据填入
        // for val in rx.iter_mut() {
        //     while self.spi.txdata.read().full().bits()  {
        //     }
        //     unsafe{
        //         self.spi.txdata.modify(|_,w| w.bits(0)); //默认发0
        //     }
        //     while self.spi.rxdata.read().empty().bits() {
        //     }
        //     //存储得到的数据
        //     *val = self.spi.rxdata.read().data().bits();
        // }

        let len = rx.len();
        let mut remaining = len;

        while remaining != 0usize {
            // words need to be transferred in a single round
            let n_words = if 8usize < remaining { 8 } else { remaining };
            // enqueue n_words junk for transmission
            for _ in 0..n_words {
                // self.tx_enque(0xffu8);
                unsafe {
                    self.spi.txdata.modify(|_, w| w.bits(0xff)); //默认发0
                }
            }
            // set watermark
            // self.spi.rxmark.write(n_words as u32 - 1);
            unsafe {
                self.spi.rxmark.modify(|_, w| w.bits(n_words as u32 - 1));
            }
            // wait for spi
            // TODO implement yielding in wait
            // self.rx_wait();
            while !self.spi.ip.read().rxwm().bits() {
                // loop
            }
            // read out all data from rx fifo
            for _ in 0..n_words {
                // rx[len - remaining] = self.rx_deque();
                rx[len - remaining] = self.spi.rxdata.read().data().bits();
                remaining = remaining - 1;
            }
        }

        // 释放csmode
        unsafe {
            self.spi.csmode.modify(|_, w| w.mode().bits(0)); //hold mode:2  |  auto:0 | off:3
        }
    }

    fn fill_data(&self, _chip_select: u32, _value: u32, _tx_len: usize) {
        panic!("spi-fill_data Unimplemented");
    }

    fn switch_cs(&self, enable: bool, csid: u32) {
        // hold mode:2  |  auto:0 | off:3
        // self.spi.csmode.modify(|_,w| w.mode().bits(if enable {2} else {3}));
        unsafe {
            self.spi
                .csmode
                .modify(|_, w| w.mode().bits(if enable { 2 } else { 0 }));
            self.spi.csid.modify(|_, w| w.bits(csid));
        }
    }
}
