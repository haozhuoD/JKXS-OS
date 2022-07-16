use alloc::{
    collections::{BTreeMap, VecDeque},
    string::String,
};
use spin::Lazy;

pub const SIG_DFL: usize = 0;
pub const SIG_IGN: usize = 1;

pub const SIGINT: u32 = 2;
pub const SIGILL: u32 = 4;
pub const SIGABRT: u32 = 6;
pub const SIGFPE: u32 = 8;
pub const SIGSEGV: u32 = 11;

pub const SIGNAL_ERRORS: Lazy<BTreeMap<u32, String>> = Lazy::new(|| {
    let mut set_ = BTreeMap::new();
    set_.insert(SIGINT, String::from("Killed, SIGINT=2"));
    set_.insert(SIGILL, String::from("Illegal Instruction, SIGILL=4"));
    set_.insert(SIGABRT, String::from("Aborted, SIGABRT=6"));
    set_.insert(
        SIGFPE,
        String::from("Erroneous Arithmetic Operation, SIGFPE=8"),
    );
    set_.insert(SIGSEGV, String::from("Segmentation Fault, SIGSEGV=11"));
    set_
});

#[repr(C)]
#[derive(Clone, Debug)]
pub struct SigAction {
    pub handler: usize,
    pub sigaction: usize,
    pub mask: u64,
}

pub fn is_signal_valid(signum: u32) -> bool {
    signum >= 1 && signum < 64
}
