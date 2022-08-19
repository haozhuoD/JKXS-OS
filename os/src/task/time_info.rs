use core::ops::{Add, Sub};

use crate::timer::USEC_PER_SEC;

// sys_setitimer
// pub const ITIMER_REAL:isize = 0; /* Timers run in real time.  */
// pub const ITIMER_VIRTUAL:isize = 1; /* Timers run only when the process is executing.  */
// pub const ITIMER_PROF:isize = 2; /* Timers run when the process is executing and when the system is executing on behalf of the process.  */

#[repr(C)]
#[derive(Copy, Clone,Debug)]
pub struct TimeSpec{
    pub tv_sec: usize,
    pub tv_usec: usize,
}

#[repr(C)]
#[derive(Copy, Clone,Debug)]
pub struct ITimerSpec{
    pub it_interval: TimeSpec, /* Interval for periodic timer */
    pub it_value: TimeSpec,    /* Time until next expiration */
}

impl TimeSpec{
    pub fn new() -> Self{
        Self{
            tv_sec:0,
            tv_usec:0
        }
    }

    #[allow(unused)]
    pub fn add_usec(&mut self, usec:usize){
        self.tv_usec += usec;
        self.tv_sec += self.tv_usec/1000_000;
        self.tv_usec %= 1000_000;
    }

    #[allow(unused)]
    pub fn is_zero(&self) -> bool{
        self.tv_sec == 0 && self.tv_usec == 0
    }

}

impl ITimerSpec{
    pub fn new() -> Self{
        Self{
            it_interval: TimeSpec::new(),
            it_value: TimeSpec::new(),
        }
    }

    #[allow(unused)]
    pub fn is_zero(&self) -> bool{
        self.it_interval.is_zero() && self.it_value.is_zero()
    }
    
    // pub fn as_bytes(&self) -> &[u8] {
    //     let size = core::mem::size_of::<Self>();
    //     unsafe {
    //         core::slice::from_raw_parts(
    //             self as *const _ as usize as *const u8,
    //             size,
    //         )
    //     }
    // }
}

impl Add for TimeSpec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut sec = self.tv_sec + other.tv_sec;
        let mut usec = self.tv_usec + other.tv_usec;
        sec += usec/USEC_PER_SEC;
        usec %= USEC_PER_SEC;
        Self {
            tv_sec: sec,
            tv_usec: usec,
        }
    }
}


// if self is less than other, then return 0
impl Sub for TimeSpec {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.tv_sec < other.tv_sec{
            return Self{tv_sec:0,tv_usec:0}
        }
        else if self.tv_sec == other.tv_sec{
            if self.tv_usec < other.tv_usec{
                return Self{tv_sec:0,tv_usec:0}
            }
            else{
                return Self{tv_sec:0,tv_usec:self.tv_usec-other.tv_usec}
            }
        }
        else{
            let mut sec = self.tv_sec - other.tv_sec;
            let mut usec = self.tv_usec - other.tv_usec;
            if self.tv_usec < other.tv_usec{
                sec -= 1;
                usec = USEC_PER_SEC + self.tv_usec - other.tv_usec;
            }
            Self {
                tv_sec: sec,
                tv_usec: usec,
            }
        }
    }
}