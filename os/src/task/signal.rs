use alloc::{collections::{BTreeMap, VecDeque}, string::String};
use spin::Lazy;

pub const SIGINT: usize = 2;
pub const SIGILL: usize = 4; 
pub const SIGABRT: usize = 6;
pub const SIGFPE: usize = 8;
pub const SIGSEGV: usize = 11;

pub const SIGNAL_ERRORS: Lazy<BTreeMap<usize, String>> = Lazy::new(|| {
    let mut set_ = BTreeMap::new();
    set_.insert(SIGINT, String::from("Killed, SIGINT=2"));
    set_.insert(SIGILL, String::from("Illegal Instruction, SIGILL=4"));
    set_.insert(SIGABRT, String::from("Aborted, SIGABRT=6"));
    set_.insert(SIGFPE, String::from("Erroneous Arithmetic Operation, SIGFPE=8"));
    set_.insert(SIGSEGV, String::from("Segmentation Fault, SIGSEGV=11"));
    set_
});

#[repr(C)]
#[derive(Clone)]
pub struct SigAction {
    pub handler: usize,
    pub sigaction: usize,
    pub mask: u64,
}

#[derive(Clone)]
pub struct SigInfo {
    pub pending_signals: VecDeque<usize>,
    pub sigactions: BTreeMap<usize, SigAction>,
}

impl SigInfo {
    pub fn new() -> Self {
        Self {
            pending_signals: VecDeque::new(),
            sigactions: BTreeMap::new(),
        }
    }
}

pub fn is_signal_valid(signum: usize) -> bool {
    signum >= 1 && signum < 64
}
