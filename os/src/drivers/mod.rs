pub mod fu740_block;
pub mod k210_block;
pub mod vir_block;

// pub use block::BLOCK_DEVICE;
#[cfg(feature = "board_fu740")]
pub use fu740_block::*;
#[cfg(feature = "board_k210")]
pub use k210_block::*;
#[cfg(feature = "board_qemu")]
pub use vir_block::*;


// use crate::console::println;
