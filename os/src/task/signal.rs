use alloc::{collections::BTreeMap, string::String};
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
    pub sa_handler: usize,
    pub sa_sigaction: usize,
    pub sa_mask: u64,
    pub sa_flags: SAFlags,
    pub sa_restorer: usize,
}

bitflags! {
    pub struct SAFlags: u32 {
        const SA_NOCLDSTOP = 1;		 /* Don't send SIGCHLD when children stop.  */
        const SA_NOCLDWAIT = 2;		 /* Don't create zombie on child death.  */
        const SA_SIGINFO   = 4;  	 /* Invoke signal-catching function with
                                        three arguments instead of one.  */
        const SA_ONSTACK   = 0x08000000; /* Use signal stack by using `sa_restorer'. */
        const SA_RESTART   = 0x10000000; /* Restart syscall on signal return.  */
        const SA_NODEFER   = 0x40000000; /* Don't automatically block the signal when
                                            its handler is being executed.  */
        const SA_RESETHAND = 0x80000000; /* Reset to SIG_DFL on entry to handler.  */
    }
}

pub fn is_signal_valid(signum: u32) -> bool {
    signum >= 1 && signum < 64
}

pub struct MContext {
    __gregs: [usize; 32],
}

pub struct Signaltstack {
    ss_sp: usize,
    ss_flags: u32,
    ss_size: u64,
}

#[repr(C)]
pub struct Sigset {
    pub __bits: [usize; 16],
}

#[repr(C)]
pub struct UContext {
    pub uc_flags: u64,
    pub uc_link: usize,
    pub uc_stack: Signaltstack,
    pub uc_sigmask: Sigset,
    pub uc_mcontext: MContext,
}

impl UContext {
    pub fn new() -> Self {
        Self {
            uc_flags: 0,
            uc_link: 0,
            uc_stack: Signaltstack {
                ss_sp: 0,
                ss_flags: 0,
                ss_size: 0,
            },
            uc_sigmask: Sigset { __bits: [0; 16] },
            uc_mcontext: MContext { __gregs: [0; 32] },
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let size = core::mem::size_of::<Self>();
        unsafe { core::slice::from_raw_parts(self as *const _ as usize as *mut u8, size) }
    }
}
