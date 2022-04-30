pub const CLOCK_FREQ: usize = 403000000 / 62; //???

pub const MMIO: &[(usize, usize)] = &[
    // we don't need clint in S priv when running
    // we only need claim/complete for target0 after initializing
    (0x0000_1000, 0x0000_1FFF), // Rom */
    (0x0000_4000, 0x0000_4FFF), // test status */
    (0x0000_6000, 0x0000_6FFF), // Chip Select */
    (0x0001_0000, 0x0001_7FFF), // Rom */
    (0x0100_0000, 0x0100_1FFF), // S7 DTIM (8 KiB) */
    (0x0170_0000, 0x0170_0FFF), // S7 Hart 0 Bus Error Unit */
    (0x0170_1000, 0x0170_1FFF), // U74 Hart 1 Bus Error Unit */
    (0x0170_2000, 0x0170_2FFF), // U74 Hart 2 Bus Error Unit */
    (0x0170_3000, 0x0170_3FFF), // U74 Hart 3 Bus Error Unit */
    (0x0170_4000, 0x0170_4FFF), // U74 Hart 4 Bus Error Unit */
    (0x0200_0000, 0x0200_FFFF), // CLINT */
    (0x0201_0000, 0x0201_0FFF), // L2 Cache Controller */
    (0x0202_0000, 0x0202_0FFF), // MSI */
    (0x0300_0000, 0x030F_FFFF), // DMA */
    (0x0800_0000, 0x081F_FFFF), // L2 Cache Controller */
    (0x0900_0000, 0x091F_FFFF), // Rom */
    (0x0A00_0000, 0x0bFF_FFFF), // Rom */
    (0x0C00_0000, 0x0FFF_FFFF), // PLIC */
    (0x1000_0000, 0x1000_0FFF), // PRCI */
    (0x1001_0000, 0x1001_0FFF), // UART0 */
    (0x1001_1000, 0x1001_1FFF), // UART1 */
    (0x1002_0000, 0x1002_0FFF), // PWM0 */   
    (0x1002_1000, 0x1002_1FFF), // PWM1 */ 
    (0x1003_0000, 0x1003_0FFF), // I2C 0 */ 
    (0x1003_1000, 0x1003_1FFF), // I2C 1 */ 
    (0x1004_0000, 0x1004_0FFF), // QSPI 0 */ 
    (0x1004_1000, 0x1004_1FFF), // QSPI 1 */ 
    (0x1005_0000, 0x1005_0FFF), // QSPI 2 */ 
    (0x1006_0000, 0x1006_0FFF), // GPIO */ 
    (0x1007_0000, 0x1007_0FFF), // OTP */ 
    (0x1008_0000, 0x1008_0FFF), // Pin Control */ 
    (0x1009_0000, 0x1009_1FFF), // Ethernet */  
    (0x100A_0000, 0x100A_0FFF), // GEMGXL MGMT */ 
    (0x100B_0000, 0x100B_3FFF), // Memory Controller */ 
    (0x100B_8000, 0x100B_8FFF), // Physical Filter */ 
    (0x100C_0000, 0x100C_0FFF), // DDR MGMT */
    (0x100D_0000, 0x100D_0FFF), // PCIE MGMT */
    (0x100E_0000, 0x100E_0FFF), // Order Ogler */
    (0x1400_0000, 0x17FF_FFFF), // Error Device 0 */
    (0x1800_0000, 0x1FFF_FFFF), // Error Device 1 */
    (0x2000_0000, 0x2FFF_FFFF), // SPI 0 */
    (0x3000_0000, 0x3FFF_FFFF), // SPI 1 */
    (0x6000_0000, 0x7FFF_FFFF), // PCIe */
    // (0x8000_0000, 0x0008_7FFF_FFFF), // Memory */
    (0x000D_F000_0000, 0x000D_FFFF_FFFF), // PCIe */
    (0x000E_0000_0000, 0x000E_FFFF_FFFF), // PCIe */
    (0x0020_0000_0000, 0x003F_FFFF_FFFF),  // PCIe */
];

pub type BlockDeviceImpl = crate::drivers::block::SDCardWrapper;