#[allow(unused)]
use crate::config::{MEMORY_END, PAGE_SIZE};

pub const QEMU: usize = 1; // 1: open in qemu mode, 0: close in real world
                           // pub const MEMORY_GDB_START: usize = MEMORY_END - PAGE_SIZE;
                           // pub const SYSCALL_ENABLE: usize = MEMORY_GDB_START + 0; // (char)0x807ff000
                           // pub const MAPPING_ENABLE: usize = MEMORY_GDB_START + 1; // (char)0x807ff001

// always open channel
pub const SYSCALL_ENABLE: usize = 1; //always open
pub const MAPPING_ENABLE: usize = 0;

#[macro_export]
macro_rules! gdb_print {
    ($place:literal, $fmt: literal $(, $($arg: tt)+)?) => {
        unsafe{
            let enable:*mut u8 =  $place;
            if ($place == 1 )||(*enable > 0 && QEMU == 1){
                print!($fmt $(, $($arg)+)?);
            }
        }
    };

    ($place:expr, $fmt: literal $(, $($arg: tt)+)?) => {
        unsafe{
            let enable:*mut u8 =  $place as *mut u8;
            if ($place == 1 )||(*enable > 0 && QEMU == 1){
                print!($fmt $(, $($arg)+)?);
            }
        }
    };
}

#[macro_export]
macro_rules! gdb_println {
    ($place:literal, $fmt: literal $(, $($arg: tt)+)?) => {
        unsafe{
            let enable:*mut u8 =  $place;
            if ($place == 1 )||(*enable > 0 && QEMU == 1){
                println!($fmt $(, $($arg)+)?);
            }
        }
    };

    ($place:expr, $fmt: literal $(, $($arg: tt)+)?) => {
        unsafe{
            let enable:*mut u8 =  $place as *mut u8;
            if ($place == 1 )||(*enable > 0 && QEMU == 1){
                println!($fmt $(, $($arg)+)?);
            }
        }
    };
}
