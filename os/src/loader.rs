use core::slice::from_raw_parts;

pub fn get_initproc_binary() -> &'static [u8] {
    extern "C" {
        fn initproc_start();
        fn initproc_end();
    }
    unsafe {
        from_raw_parts(
            initproc_start as *const u8,
            initproc_end as usize - initproc_start as usize,
        )
    }
}

pub fn get_usershell_binary() -> &'static [u8] {
    extern "C" {
        fn usershell_start();
        fn usershell_end();
    }
    unsafe {
        from_raw_parts(
            usershell_start as *const u8,
            usershell_end as usize - usershell_start as usize,
        )
    }
}
