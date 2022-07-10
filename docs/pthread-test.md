# pthread_cancel_points执行出错

问题：两个futex同时wait，炸掉了。
```
========== START entry-static.exe pthread_cancel_points ==========
[syscall pid=4] : syscall(135), args = [0, 1d160, f0017b50, 8, 0, fffffffffffff000]
[syscall pid=4] : syscall(220), args = [11, 0, f0017b50, 8, 0, fffffffffffff000]
[syscall pid=4] : sys_clone(flags: SIGCHLD, child_stack: 0x0000000000000000, ptid: 0xf0017b50, ctid: 0x8, newtls: 0x0) = 5
[syscall pid=4] : syscall(135), args = [2, f0017b50, 0, 8, 0, fffffffffffff000]
[syscall pid=4] : syscall(137), args = [f0017c00, 0, f0017bf0, 8, 0, 0]
[syscall pid=4] : Unsupported syscall_id: 137, args = [
    0xf0017c00,
    0x0,
    0xf0017bf0,
    0x8,
    0x0,
    0x0,
]
[syscall pid=4] : syscall(260), args = [5, f0017bf0, 0, 0, 0, 0]
[syscall pid=5] : syscall(178), args = [0, 0, f0017b50, 8, 1db18, 1dbf8]
[syscall pid=5] : sys_getpid() = 5
[syscall pid=5] : syscall(135), args = [2, f0017b50, 0, 8, 1db18, 1d3c8]
[syscall pid=5] : syscall(261), args = [0, 3, 0, f0017bb0, 1db18, fffffffffffff000]
[syscall pid=5] : Unsupported syscall_id: 261, args = [
    0x0,
    0x3,
    0x0,
    0xf0017bb0,
    0x1db18,
    0xfffffffffffff000,
]
[syscall pid=5] : syscall(221), args = [f0017f16, f0017d20, f0017d38, f0017bb0, 1db18, 65]
[syscall pid=5] : sys_exec(path: "entry-static.exe", args: ["entry-static.exe", "pthread_cancel_points"] ) = 2
[syscall pid=5] : syscall(96), args = [a8c18, a6fe6, 0, 1, 1, 1]
[syscall pid=5] : sys_set_tid_address(ptr: 0x00000000000a8c18) = 5
pthread_create
[syscall pid=5] : syscall(135), args = [1, f0017b70, 0, 8, 0, 300000000]
[syscall pid=5] : syscall(222), args = [0, 23000, 0, 22, ffffffffffffffff, 0]
[syscall pid=5] : sys_mmap(aligned_start: 0x80000000, aligned_len: 143360, prot: 0, flags: 22, fd: -1, offset: 0 ) = 0x80000000
[syscall pid=5] : syscall(226), args = [80002000, 21000, 3, fffffffffffff000, 21fff, 80023fff]
[syscall pid=5] : sys_mprotect(addr: 0x80002000, len: 135168, prot: 0x3) = 0
[syscall pid=5] : syscall(135), args = [0, a8990, f0017b70, 8, 1, 0]
[syscall pid=5] : syscall(220), args = [7d0f00, 80022ac0, 80022b28, 80022bd0, a8c18, 80022bd0]
[syscall pid=5] : sys_clone(flags: CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND | CLONE_THREAD | CLONE_SYSVSEM | CLONE_SETTLS | CLONE_PARENT_SETTID | CLONE_CHILD_CLEARTID | CLONE_DETACHED, child_stack: 0x0000000080022ac0, ptid: 0x80022b28, ctid: 0x80022bd0, newtls: 0xa8c18) = 6
[syscall pid=6] : syscall(135), args = [2, 80022ae8, 0, 8, a8c18, 0]
run_execute
[syscall pid=6] : syscall(98), args = [a9190, 80, ffffffffffffffff, 0, 0, 0]
[syscall pid=6] : *****sys_futex(uaddr: 0x00000000000a9190, futex_op: 80, val: ffffffff, timeout: 0x0000000000000000, uaddr2: 0x0000000000000000, val3: 0) = ?
futex_wait: uval: ffffffff, val: ffffffff, timeout: 18446744073709551615
[syscall pid=5] : syscall(135), args = [2, f0017b70, 0, 8, 0, a8b78]
pthread_create ok
[syscall pid=5] : syscall(134), args = [21, f0017b30, 0, 8, 0, f0017b50]
[syscall pid=5] : sys_sigaction(signum: 33, sigaction = 0x00000000f0017b30, old_sigaction = 0x0000000000000000 ) = 0
pthread_cancel ok
unblocking canceled thread ok
[syscall pid=5] : syscall(98), args = [80022b30, 80, 1, 0, 0, 0]
[syscall pid=5] : *****sys_futex(uaddr: 0x0000000080022b30, futex_op: 80, val: 1, timeout: 0x0000000000000000, uaddr2: 0x0000000000000000, val3: 0) = ?
futex_wait: uval: 1, val: 1, timeout: 18446744073709551615
```

在标准linux下strace，结果如下：
```
[pid    89] execve("entry-static.exe", ["entry-static.exe", "pthread_cancel_points"], 0x3fffc25d40 /* 9 vars */) = 0
[pid    89] set_tid_address(0xa8c18)    = 89
[pid    89] write(1, "pthread_create\n", 15pthread_create
) = 15
[pid    89] rt_sigprocmask(SIG_UNBLOCK, [RT_1 RT_2], NULL, 8) = 0
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[90], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 90
strace: Process 90 attached
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    90] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    90] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
 <unfinished ...>
[pid    90] write(1, "run_execute\n", 12run_execute
 <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    90] <... write resumed>)        = 12
[pid    89] rt_sigaction(SIGRT_1, {sa_handler=0x40af8, sa_mask=[QUIT ILL ABRT BUS FPE KILL USR1 SEGV USR2 PIPE TERM STKFLT STOP], sa_flags=SA_RESTART|SA_SIGINFO|0x4000000},  <unfinished ...>
[pid    90] futex(0xa9190, FUTEX_WAIT_PRIVATE, 4294967295, NULL <unfinished ...> // 子线程：卡在这里！
[pid    89] <... rt_sigaction resumed>NULL, 8) = 0
[pid    89] tkill(90, SIGRT_1 <unfinished ...>
[pid    90] <... futex resumed>)        = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
[pid    89] <... tkill resumed>)        = 0
[pid    90] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    90] rt_sigreturn({mask=[CHLD]} <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    90] <... rt_sigreturn resumed>) = 692624
[pid    89] futex(0xa9190, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    90] futex(0xa9190, FUTEX_WAIT_PRIVATE, 4294967295, NULL <unfinished ...>
[pid    89] <... futex resumed>)        = 0
[pid    90] <... futex resumed>)        = -1 EAGAIN (Resource temporarily unavailable)
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    90] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    89] <... write resumed>)        = 30
[pid    90] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    90] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    89] <... futex resumed>)        = -1 EAGAIN (Resource temporarily unavailable)
[pid    90] <... futex resumed>)        = 0
[pid    89] futex(0xa8c18, FUTEX_WAIT, 90, NULL <unfinished ...>
[pid    90] exit(0)                     = ?
[pid    90] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "pthread_create\n", 15pthread_create
) = 15
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[91], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 91
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], strace: Process 91 attached
NULL, 8) = 0
[pid    91] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
 <unfinished ...>
[pid    91] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... write resumed>)        = 18
[pid    91] write(1, "run_execute\n", 12run_execute
 <unfinished ...>
[pid    89] tkill(91, SIGRT_1 <unfinished ...>
[pid    91] <... write resumed>)        = 12
[pid    89] <... tkill resumed>)        = 0
[pid    91] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    91] tkill(91, SIGRT_1 <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    91] <... tkill resumed>)        = 0
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    91] rt_sigreturn({mask=[CHLD RT_1]} <unfinished ...>
[pid    89] <... write resumed>)        = 30
[pid    91] <... rt_sigreturn resumed>) = 12
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    91] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD RT_1], 8) = 0
[pid    91] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    91] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 91, NULL <unfinished ...>
[pid    91] <... exit resumed>)         = ?
[pid    91] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "pthread_create\n", 15pthread_create
) = 15
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[92], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 92
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
strace: Process 92 attached
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
 <unfinished ...>
[pid    92] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    92] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] tkill(92, SIGRT_1 <unfinished ...>
[pid    92] futex(0x3ff307cba8, FUTEX_WAIT_PRIVATE, 2147483650, NULL <unfinished ...>
[pid    89] <... tkill resumed>)        = 0
[pid    92] <... futex resumed>)        = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
[pid    89] futex(0x3ff307cba8, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    92] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] <... futex resumed>)        = 0
[pid    92] rt_sigreturn({mask=[CHLD]} <unfinished ...>
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    92] <... rt_sigreturn resumed>) = 274660314024
[pid    89] <... write resumed>)        = 18
[pid    92] futex(0x3ff307cba8, FUTEX_WAIT_PRIVATE, 2147483650, NULL <unfinished ...>
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    92] <... futex resumed>)        = -1 EAGAIN (Resource temporarily unavailable)
[pid    89] <... write resumed>)        = 30
[pid    92] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    92] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    92] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    92] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 92, NULL <unfinished ...>
[pid    92] <... exit resumed>)         = ?
[pid    92] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "src/functional/pthread_cancel-po"..., 94src/functional/pthread_cancel-points.c:148: seqno == 1 failed (blocking sem_timedwait, seqno)
) = 94
[pid    89] write(1, "pthread_create\n", 15pthread_create
) = 15
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[93], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 93
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
strace: Process 93 attached
) = 18
[pid    93] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] tkill(93, SIGRT_1 <unfinished ...>
[pid    93] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... tkill resumed>)        = 0
[pid    93] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    93] tkill(93, SIGRT_1 <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    93] <... tkill resumed>)        = 0
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    93] rt_sigreturn({mask=[CHLD RT_1]} <unfinished ...>
[pid    89] <... write resumed>)        = 30
[pid    93] <... rt_sigreturn resumed>) = 0
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    93] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD RT_1], 8) = 0
[pid    93] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    93] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 93, NULL <unfinished ...>
[pid    93] <... exit resumed>)         = ?
[pid    93] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "src/functional/pthread_cancel-po"..., 98src/functional/pthread_cancel-points.c:148: seqno == 1 failed (non-blocking sem_timedwait, seqno)
) = 98
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[94], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 94
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
[pid    89] write(1, "pthread_create\n", 15pthread_create
strace: Process 94 attached
) = 15
[pid    94] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0 <unfinished ...>
[pid    94] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... mmap resumed>)         = 0x3ff3037000
[pid    94] futex(0xa9170, FUTEX_WAIT_PRIVATE, 4294967295, NULL <unfinished ...>
[pid    89] mprotect(0x3ff3039000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff3059ac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[95], tls=0x3ff3059bd0, child_tidptr=0xa8c18) = 95
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
strace: Process 95 attached
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
 <unfinished ...>
[pid    95] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    95] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] tkill(95, SIGRT_1 <unfinished ...>
[pid    95] futex(0x3ff3059ba8, FUTEX_WAIT_PRIVATE, 2147483650, NULL <unfinished ...>
[pid    89] <... tkill resumed>)        = 0
[pid    95] <... futex resumed>)        = ? ERESTARTSYS (To be restarted if SA_RESTART is set)
[pid    89] futex(0x3ff3059ba8, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    95] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] <... futex resumed>)        = 0
[pid    95] rt_sigreturn({mask=[CHLD]} <unfinished ...>
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    95] <... rt_sigreturn resumed>) = 274660170664
[pid    89] <... write resumed>)        = 18
[pid    95] futex(0x3ff3059ba8, FUTEX_WAIT_PRIVATE, 2147483650, NULL <unfinished ...>
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    95] <... futex resumed>)        = -1 EAGAIN (Resource temporarily unavailable)
[pid    89] <... write resumed>)        = 30
[pid    95] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    89] futex(0x3ff3059b30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    95] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    95] futex(0x3ff3059b30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    95] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 95, NULL <unfinished ...>
[pid    95] <... exit resumed>)         = ?
[pid    95] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff3037000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "src/functional/pthread_cancel-po"..., 93src/functional/pthread_cancel-points.c:148: seqno == 1 failed (blocking pthread_join, seqno)
) = 93
[pid    89] futex(0xa9170, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    94] <... futex resumed>)        = 0
[pid    89] <... futex resumed>)        = 1
[pid    94] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    94] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    94] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    94] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 94, NULL <unfinished ...>
[pid    94] <... exit resumed>)         = ?
[pid    94] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[96], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 96
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
[pid    89] write(1, "pthread_create\n", 15pthread_create
strace: Process 96 attached
) = 15
[pid    96] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0 <unfinished ...>
[pid    96] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... mmap resumed>)         = 0x3ff3037000
[pid    96] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    89] mprotect(0x3ff3039000, 135168, PROT_READ|PROT_WRITE <unfinished ...>
[pid    96] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    89] <... mprotect resumed>)     = 0
[pid    96] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1 <unfinished ...>
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2],  <unfinished ...>
[pid    96] <... futex resumed>)        = 0
[pid    89] <... rt_sigprocmask resumed>[CHLD], 8) = 0
[pid    96] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 96, NULL <unfinished ...>
[pid    96] <... exit resumed>)         = ?
[pid    96] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] clone(child_stack=0x3ff3059ac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[97], tls=0x3ff3059bd0, child_tidptr=0xa8c18) = 97
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
strace: Process 97 attached
) = 18
[pid    97] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] tkill(97, SIGRT_1 <unfinished ...>
[pid    97] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... tkill resumed>)        = 0
[pid    97] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    97] tkill(97, SIGRT_1 <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    97] <... tkill resumed>)        = 0
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    97] rt_sigreturn({mask=[CHLD RT_1]} <unfinished ...>
[pid    89] <... write resumed>)        = 30
[pid    97] <... rt_sigreturn resumed>) = 0
[pid    89] futex(0x3ff3059b30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    97] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD RT_1], 8) = 0
[pid    97] futex(0x3ff3059b30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    97] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 97, NULL <unfinished ...>
[pid    97] <... exit resumed>)         = ?
[pid    97] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff3037000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "src/functional/pthread_cancel-po"..., 97src/functional/pthread_cancel-points.c:148: seqno == 1 failed (non-blocking pthread_join, seqno)
) = 97
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "pthread_create\n", 15pthread_create
) = 15
[pid    89] mmap(NULL, 143360, PROT_NONE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0) = 0x3ff305a000
[pid    89] mprotect(0x3ff305c000, 135168, PROT_READ|PROT_WRITE) = 0
[pid    89] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD], 8) = 0
[pid    89] clone(child_stack=0x3ff307cac0, flags=CLONE_VM|CLONE_FS|CLONE_FILES|CLONE_SIGHAND|CLONE_THREAD|CLONE_SYSVSEM|CLONE_SETTLS|CLONE_PARENT_SETTID|CLONE_CHILD_CLEARTID|0x400000, parent_tid=[98], tls=0x3ff307cbd0, child_tidptr=0xa8c18) = 98
[pid    89] rt_sigprocmask(SIG_SETMASK, [CHLD], NULL, 8) = 0
[pid    89] write(1, "pthread_create ok\n", 18pthread_create ok
strace: Process 98 attached
) = 18
[pid    98] rt_sigprocmask(SIG_SETMASK, [CHLD],  <unfinished ...>
[pid    89] tkill(98, SIGRT_1 <unfinished ...>
[pid    98] <... rt_sigprocmask resumed>NULL, 8) = 0
[pid    89] <... tkill resumed>)        = 0
[pid    98] --- SIGRT_1 {si_signo=SIGRT_1, si_code=SI_TKILL, si_pid=89, si_uid=0} ---
[pid    89] write(1, "pthread_cancel ok\n", 18pthread_cancel ok
 <unfinished ...>
[pid    98] tkill(98, SIGRT_1 <unfinished ...>
[pid    89] <... write resumed>)        = 18
[pid    98] <... tkill resumed>)        = 0
[pid    89] write(1, "unblocking canceled thread ok\n", 30unblocking canceled thread ok
 <unfinished ...>
[pid    98] rt_sigreturn({mask=[CHLD RT_1]} <unfinished ...>
[pid    89] <... write resumed>)        = 30
[pid    98] <... rt_sigreturn resumed>) = 0
[pid    89] futex(0x3ff307cb30, FUTEX_WAIT_PRIVATE, 1, NULL <unfinished ...>
[pid    98] rt_sigprocmask(SIG_BLOCK, ~[RTMIN RT_1 RT_2], [CHLD RT_1], 8) = 0
[pid    98] futex(0x3ff307cb30, FUTEX_WAKE_PRIVATE, 1) = 1
[pid    89] <... futex resumed>)        = 0
[pid    98] exit(0 <unfinished ...>
[pid    89] futex(0xa8c18, FUTEX_WAIT, 98, NULL <unfinished ...>
[pid    98] <... exit resumed>)         = ?
[pid    98] +++ exited with 0 +++
[pid    89] <... futex resumed>)        = 0
[pid    89] munmap(0x3ff305a000, 143360) = 0
[pid    89] write(1, "joining canceled thread\n", 24joining canceled thread
) = 24
[pid    89] write(1, "src/functional/pthread_cancel-po"..., 115src/functional/pthread_cancel-points.c:150: res != PTHREAD_CANCELED failed (shm_open, canceled thread exit status)
) = 115
[pid    89] exit_group(1)               = ?
[pid    89] +++ exited with 1 +++
[pid    88] <... rt_sigtimedwait resumed>) = 17 (SIGCHLD)
[pid    88] wait4(89, [{WIFEXITED(s) && WEXITSTATUS(s) == 1}], 0, NULL) = 89
[pid    88] write(1, "FAIL pthread_cancel_points [stat"..., 38FAIL pthread_cancel_points [status 1]
) = 38
[pid    88] write(1, "========== END entry-static.exe "..., 65========== END entry-static.exe pthread_cancel_points ==========
) = 65
[pid    88] exit_group(1)               = ?
[pid    88] +++ exited with 1 +++
<... wait4 resumed>[{WIFEXITED(s) && WEXITSTATUS(s) == 1}], 0, NULL) = 88
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=88, si_uid=0, si_status=1, si_utime=0, si_stime=0} ---
rt_sigreturn({mask=[]})                 = 88
read(10, "-w entry-static.exe search_insqu"..., 1023) = 1023
read(10, ".exe -w entry-static.exe strtold"..., 1023) = 1023
read(10, "e getpwnam_r_errno\n# ./runtest.e"..., 1023) = 1023
read(10, "smasher\n# ./runtest.exe -w entry"..., 1023) = 1023
read(10, ".exe -w entry-static.exe sscanf_"..., 1023) = 347
read(10, "", 1023)                      = 0
exit_group(1)                           = ?
+++ exited with 1 +++
```